use std::{convert::TryInto, path::PathBuf, sync::Arc, time::Duration, usize};

use anyhow::{bail, Context, Error, Result};
use imagesize::blob_size;

use reqwest::{header::RANGE, Client, Response};
use tokio::{
	fs::{self, File},
	io::AsyncWriteExt,
	sync::{mpsc::UnboundedSender, Semaphore},
};
use tokio_retry::{
	strategy::{jitter, FixedInterval},
	Retry,
};

use super::models::{download_meta::DownloadMeta, download_status::DownloadStatus};
use crate::api::{
	config::{config::Config, configuration::Subreddit},
	reddit::models::listing::Listing,
};

#[derive(Clone, Debug)]
pub struct Repository {
	client: Arc<Client>,
	config: Arc<Config>,
	semaphore: Arc<Semaphore>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrintOut {
	Bar,
	Text,
	None,
}

static APP_USER_AGENT: &str = concat!(
	"id.web.tigor.",
	env!("CARGO_PKG_NAME"),
	"/v",
	env!("CARGO_PKG_VERSION"),
	" (by /u/CrowFX)"
);

impl Repository {
	pub fn new(config: Arc<Config>) -> Self {
		let os = std::env::consts::OS;
		let user_agent = os.to_string() + ":" + APP_USER_AGENT;
		let client = reqwest::Client::builder()
			.user_agent(user_agent)
			.connect_timeout(Duration::from_secs(config.timeout.into()))
			.build()
			.context("failed to create request client")
			.unwrap();

		let semaphore = Arc::new(Semaphore::new(config.download_threads));
		Self {
			client: Arc::new(client),
			config,
			semaphore,
		}
	}

	pub async fn download(
		&self,
		display: PrintOut,
		progress: UnboundedSender<DownloadStatus>,
	) -> Vec<(DownloadMeta, Result<(), Error>)> {
		let mut handlers = Vec::new();

		for (_, subreddit) in &self.config.subreddits {
			let this = self.clone();
			let subreddit = subreddit.clone();
			let progress = progress.clone();
			let handle =
				tokio::spawn(async move { this.exec_download(subreddit, display, progress).await });
			handlers.push(handle);
		}
		let mut v = Vec::new();
		for handle in handlers {
			let op = handle.await.unwrap();
			match op {
				Ok(res) => v.extend(res),
				Err(err) => println!("{:#?}", err),
			}
		}
		v
	}

	async fn exec_download(
		&self,
		subreddit: Subreddit,
		display: PrintOut,
		progress: UnboundedSender<DownloadStatus>,
	) -> Result<Vec<(DownloadMeta, Result<(), Error>)>> {
		let print = || {
			println!("{} downloading listing", subreddit.padded_proper_name());
		};
		match display {
			PrintOut::None => {}
			_ => print(),
		}
		let downloads = self.download_listing(&subreddit).await?;
		Ok(self.download_images(downloads, subreddit, progress).await)
	}

	async fn download_images(
		&self,
		downloads: Vec<DownloadMeta>,
		subreddit: Subreddit,
		progress: UnboundedSender<DownloadStatus>,
	) -> Vec<(DownloadMeta, Result<(), Error>)> {
		let mut handlers = Vec::new();
		'meta: for mut meta in downloads.into_iter() {
			for profile in &meta.profile {
				if self.file_exists(profile, &meta).await {
					continue 'meta;
				}
			}
			let this = self.clone();
			let sem = self.semaphore.clone();
			let subreddit = subreddit.clone();
			let progress = progress.clone();
			let handle = tokio::spawn(async move {
				// release semaphore lock on end of scope
				let _x = sem.acquire().await.unwrap();
				let op = this.download_image(&mut meta, subreddit, progress).await;
				(meta, op)
			});
			handlers.push(handle);
		}
		let mut v = Vec::new();
		for handle in handlers {
			v.push(handle.await.unwrap());
		}
		v
	}

	async fn download_listing(&self, subreddit: &Subreddit) -> Result<Vec<DownloadMeta>> {
		let listing_url = format!(
			"https://reddit.com/r/{}/{}.json?limit=100",
			subreddit.proper_name, subreddit.sort
		);

		let retry_strategy = FixedInterval::from_millis(100).map(jitter).take(3);
		let resp: Response = Retry::spawn(retry_strategy, || async {
			let res = self.client.get(&listing_url).send().await?;
			Ok::<Response, Error>(res)
		})
		.await
		.with_context(|| {
			format!(
				"failed to open connection to download listing from: {}",
				listing_url
			)
		})?;

		let listing: Listing = resp
			.json()
			.await
			.with_context(|| format!("failed to deserialize json body from: {}", listing_url))?;

		Ok(listing.into_download_metas(&self.config))
	}

	async fn download_image(
		&self,
		meta: &mut DownloadMeta,
		subreddit: Subreddit,
		progress: UnboundedSender<DownloadStatus>,
	) -> Result<()> {
		if subreddit.download_first {
			self.poke_image_size(meta).await?;
			let mut should_continue = false;
			for (profile, setting) in self.config.iter() {
				if !meta.passed_checks(setting) {
					continue;
				}
				if self.file_exists(profile, meta).await {
					continue;
				}
				should_continue = true;
				meta.profile.push(profile.to_owned());
			}
			if !should_continue {
				return Ok(());
			}
		}

		// println!(
		// 	"{:?} [{}] downloading image {}",
		// 	meta.profile, meta.subreddit_name, meta.url
		// );
		let retry_strategy = FixedInterval::from_millis(100).map(jitter).take(3);
		let response: Response = Retry::spawn(retry_strategy, || async {
			let res = self.client.get(&meta.url).send().await?;
			Ok::<Response, Error>(res)
		})
		.await
		.with_context(|| {
			format!(
				"failed to open connection to download image from: {}",
				meta.url
			)
		})?;

		let status = response.status();
		if !status.is_success() {
			bail!(format!(
				"download from {} gives [{}: {}] status code",
				meta.url,
				status.as_u16(),
				status
					.canonical_reason()
					.unwrap_or_else(|| "Unknown Reason"),
			));
		}

		self.ensure_download_dir(meta).await?;

		let temp_file = self.store_to_temp(response, meta, progress).await?;

		for profile in &meta.profile {
			let download_location = self.download_location(profile, meta);
			fs::copy(&temp_file, &download_location)
				.await
				.with_context(|| {
					format!(
						"failed to copy file from tmp dir to {}",
						download_location.display()
					)
				})?;
			let dir_path = std::env::temp_dir()
				.join("ridit")
				.join(&meta.subreddit_name)
				.join(&meta.filename);
			fs::remove_file(&dir_path).await.with_context(|| {
				format!(
					"failed to remove temp downloaded file {}",
					download_location.display()
				)
			})?;
		}

		Ok(())
	}

	async fn ensure_download_dir(&self, meta: &DownloadMeta) -> Result<()> {
		for profile in &meta.profile {
			let download_dir = self.config.path.join(profile).join(&meta.subreddit_name);
			fs::create_dir_all(&download_dir).await.with_context(|| {
				format!(
					"failed to create download directory on: {}",
					download_dir.display()
				)
			})?;
		}
		Ok(())
	}

	/// Checks for image size by downloading small image size first, then updates the given
	/// DownloadMeta information on success. Note this does not download the whole file.
	async fn poke_image_size(&self, meta: &mut DownloadMeta) -> Result<()> {
		const LIMIT: usize = 1024 * 2 * 10;
		let retry_strategy = FixedInterval::from_millis(100).map(jitter).take(3);
		let mut resp = Retry::spawn(retry_strategy, || async {
			let res = self
				.client
				.get(&meta.url)
				.header(RANGE, LIMIT)
				.send()
				.await?;
			Ok::<Response, Error>(res)
		})
		.await
		.with_context(|| {
			format!(
				"failed to partial download an image to get image size from: {}",
				meta.url
			)
		})?;
		let mut data: Vec<u8> = Vec::new();
		while let Some(chunk) = resp.chunk().await? {
			data.append(&mut chunk.to_vec());
			// Just in case the server does not respect Range header
			if data.len() >= LIMIT {
				break;
			}
		}
		let size = blob_size(&data)
			.with_context(|| format!("error getting image dimension from: {}", meta.url))?;

		meta.image_height = size.height;
		meta.image_width = size.width;
		Ok(())
	}

	fn download_dir(&self, profile: &str, meta: &DownloadMeta) -> PathBuf {
		self.config.path.join(profile).join(&meta.subreddit_name)
	}

	fn download_location(&self, profile: &str, meta: &DownloadMeta) -> PathBuf {
		self.download_dir(profile, meta).join(&meta.filename)
	}

	async fn file_exists(&self, profile: &str, meta: &DownloadMeta) -> bool {
		fs::metadata(self.download_location(profile, meta))
			.await
			.is_ok()
	}

	async fn store_to_temp(
		&self,
		mut resp: Response,
		meta: &DownloadMeta,
		progress: UnboundedSender<DownloadStatus>,
	) -> Result<PathBuf> {
		let dir_path = std::env::temp_dir()
			.join("ridit")
			.join(&meta.subreddit_name);
		fs::create_dir_all(&dir_path).await?;
		let file_path = dir_path.join(&meta.filename);
		let mut file = File::create(&file_path)
			.await
			.context("cannot create file on tmp dir")?;

		let download_length = resp.content_length().unwrap_or(0);
		progress
			.send(meta.as_download_status(download_length, 0))
			.unwrap();
		while let Some(chunk) = resp.chunk().await? {
			progress
				.send(meta.as_download_status(download_length, chunk.len().try_into().unwrap()))
				.unwrap();

			if let Err(err) = file.write(&chunk).await {
				progress
					.send(
						meta.as_download_status(download_length, chunk.len().try_into().unwrap())
							.with_error(err.to_string()),
					)
					.unwrap();
				bail!("failed to save image from {}. cause: {}", meta.url, err)
			}
		}
		progress
			.send(meta.as_download_status(download_length, 0).set_finished())
			.unwrap();
		Ok(file_path)
	}

	/// Checks to reddit if subreddit exists
	/// Also mutates the given subreddit name to proper casing.
	pub async fn subreddit_exist(subreddit: &mut String) -> Result<bool> {
		let url = format!("https://reddit.com/r/{}.json", subreddit);
		let retry_strategy = FixedInterval::from_millis(100).map(jitter).take(3);
		let resp: Response = Retry::spawn(retry_strategy, || async {
			let res = reqwest::get(&url).await?;
			Ok::<Response, Error>(res)
		})
		.await
		.with_context(|| format!("failed to check subreddit {}", subreddit))?;

		let listing: Listing = resp
			.json()
			.await
			.with_context(|| format!("failed to deserialize json body from: {}", url))?;

		match listing.data.children.get(0) {
			Some(v) => {
				subreddit.clear();
				subreddit.push_str(&v.data.subreddit);
				Ok(true)
			}
			None => Ok(false),
		}
	}
}
