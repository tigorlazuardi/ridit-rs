use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Error, Result};
use imagesize::blob_size;
use reqwest::{header::RANGE, Client, Response};
use tokio::{
	fs::{self, File},
	io::AsyncWriteExt,
	sync::Semaphore,
};
use tokio_retry::{
	strategy::{jitter, FixedInterval},
	Retry,
};

use super::models::download_meta::DownloadMeta;
use crate::{
	api::{
		config::{
			config::Config,
			configuration::{Configuration, Subreddit},
		},
		reddit::models::listing::Listing,
	},
	pkg::OnError,
};

#[derive(Clone, Debug)]
pub struct Repository {
	client: Arc<Client>,
	config: Arc<Config>,
	semaphore: Arc<Semaphore>,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Repository {
	pub fn new(config: Arc<Config>) -> Self {
		let client = reqwest::Client::builder()
			.user_agent(APP_USER_AGENT)
			.connect_timeout(Duration::from_secs(config.timeout.into()))
			.build()
			.context("failed to create request client")
			.unwrap();

		let semaphore = Arc::new(Semaphore::new(6));
		Self {
			client: Arc::new(client),
			config,
			semaphore,
		}
	}

	pub async fn download(&self, config: Arc<Configuration>) {
		let mut handlers = Vec::new();

		for (name, subreddit) in config.subreddits.iter() {
			let this = self.clone();
			let name = name.clone();
			let subreddit = subreddit.clone();
			let config = config.clone();
			let handle = tokio::spawn(async move {
				this.exec_download(config, &name, subreddit).await.ok();
			});
			handlers.push(handle);
		}
		for handle in handlers {
			handle.await.log_error().ok();
		}
	}

	async fn exec_download(
		&self,
		config: Arc<Configuration>,
		name: &str,
		subreddit: Subreddit,
	) -> Result<()> {
		let downloads = self.download_listing(&*config, name, subreddit).await?;
		self.download_images(config, downloads, subreddit).await;
		Ok(())
	}

	async fn download_images(
		&self,
		config: Arc<Configuration>,
		downloads: Vec<DownloadMeta>,
		subreddit: Subreddit,
	) {
		let mut handlers = Vec::new();
		for mut meta in downloads.into_iter() {
			if Repository::file_exists(&*config, &meta).await {
				continue;
			}
			let this = self.clone();
			let config = config.clone();
			let sem = self.semaphore.clone();
			let handle = tokio::spawn(async move {
				// release semaphore lock on end of scope
				let _x = sem.acquire().await.unwrap();
				this.download_image(&*config, &mut meta, subreddit).await?;
				Ok::<(), Error>(())
			});
			handlers.push(handle);
		}
		for handle in handlers {
			handle.await.log_error().ok();
		}
	}

	async fn download_listing(
		&self,
		config: &Configuration,
		name: &str,
		subreddit: Subreddit,
	) -> Result<Vec<DownloadMeta>> {
		let listing_url = format!(
			"https://reddit.com/r/{}/{}.json?limit=100",
			name, subreddit.sort
		);

		println!("[{}] fetching listing", name);
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

		let result: Listing = resp.json().await.with_context(|| {
			format!(
				"failed to deserialize json response body (unsupported body format) from: {}",
				listing_url
			)
		})?;
		Ok(result.into_download_metas(config))
	}

	async fn download_image(
		&self,
		config: &Configuration,
		meta: &mut DownloadMeta,
		subreddit: Subreddit,
	) -> Result<()> {
		if subreddit.download_first {
			self.poke_image_size(meta).await?;
			if !meta.passed_checks(config) {
				return Ok(());
			}
		}

		println!("[{}] downloading image {}", meta.subreddit_name, meta.url);
		let retry_strategy = FixedInterval::from_millis(100).map(jitter).take(3);
		let response = Retry::spawn(retry_strategy, || async {
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

		Repository::ensure_download_dir(config, meta).await?;

		let temp_file = Repository::store_to_temp(response, meta).await?;
		let download_location = Repository::download_location(config, meta);

		fs::copy(temp_file, &download_location)
			.await
			.with_context(|| {
				format!(
					"failed to copy file from tmp dir to {}",
					download_location.display()
				)
			})?;

		Ok(())
	}

	async fn ensure_download_dir(config: &Configuration, meta: &DownloadMeta) -> Result<()> {
		let download_dir = Repository::download_dir(config, meta);
		fs::create_dir_all(&download_dir).await.with_context(|| {
			format!(
				"failed to create download directory on: {}",
				download_dir.display()
			)
		})?;

		Ok(())
	}

	/// Checks for image size by downloading small image size first, then updates the given
	/// DownloadMeta information on success. Note this does not download the whole file.
	async fn poke_image_size(&self, meta: &mut DownloadMeta) -> Result<()> {
		const LIMIT: usize = 0x200;
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
			.with_context(|| format!("error getting dimension from: {}", meta.url))?;

		meta.image_height = size.height;
		meta.image_width = size.width;
		Ok(())
	}

	fn download_dir(config: &Configuration, meta: &DownloadMeta) -> PathBuf {
		config.download.path.join(&meta.subreddit_name)
	}

	fn download_location(config: &Configuration, meta: &DownloadMeta) -> PathBuf {
		Repository::download_dir(config, meta).join(&meta.filename)
	}

	async fn file_exists(config: &Configuration, meta: &DownloadMeta) -> bool {
		fs::metadata(Repository::download_location(config, meta))
			.await
			.is_ok()
	}

	async fn store_to_temp(mut resp: Response, meta: &DownloadMeta) -> Result<PathBuf> {
		let file_path = std::env::temp_dir().join("ridit").join(&meta.filename);
		let mut file = File::create(&file_path)
			.await
			.context("cannot create file on tmp dir")?;

		while let Some(chunk) = resp.chunk().await? {
			file.write(&chunk)
				.await
				.context("failed to write to temp dir")?;
		}

		Ok(file_path)
	}

	/// Checks to reddit if subreddit exists
	pub async fn subreddit_exist(subreddit: &str) -> Result<bool> {
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

		Ok(listing.data.children.len() > 0)
	}
}
