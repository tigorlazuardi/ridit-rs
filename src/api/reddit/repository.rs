use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Error, Result};
use reqwest::{Client, Response};
use tokio::{
	fs::{self, File},
	io::AsyncWriteExt,
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
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl Repository {
	pub fn new(config: Arc<Config>) -> Self {
		let client = reqwest::Client::builder()
			.user_agent(APP_USER_AGENT)
			.connect_timeout(Duration::from_secs(10))
			.build()
			.context("failed to create request client")
			.unwrap();
		Self {
			client: Arc::new(client),
			config,
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
		self.download_images(config, downloads).await;
		Ok(())
	}

	async fn download_images(&self, config: Arc<Configuration>, downloads: Vec<DownloadMeta>) {
		let mut handlers = Vec::new();
		for meta in downloads.into_iter() {
			if Repository::file_exists(&*config, &meta).await {
				continue;
			}
			let this = self.clone();
			let config = config.clone();
			let handle = tokio::spawn(async move {
				this.download_image(&*config, &meta).await?;
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
				"failed to open connecttion to download listing from: {}",
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

	async fn download_image(&self, config: &Configuration, meta: &DownloadMeta) -> Result<()> {
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

		{
			let download_dir = Repository::download_dir(config, meta);
			fs::create_dir_all(&download_dir).await.with_context(|| {
				format!(
					"failed to create download directory on: {}",
					download_dir.display()
				)
			})?;
		}

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
}
