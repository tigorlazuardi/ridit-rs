use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use backoff::future::retry;
use backoff::ExponentialBackoff;
use reqwest::Client;

use super::models::download_meta::DownloadMeta;
use crate::api::{
	config::{
		config::Config,
		configuration::{Configuration, Subreddit},
	},
	reddit::models::listing::Listing,
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
			.connect_timeout(Duration::from_secs(5))
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
			let name = name.clone();
			let subreddit = subreddit.clone();
			let client = self.client.clone();
			let config = config.clone();
			let handle = tokio::spawn(async move {
				if let Ok(meta) =
					Repository::download_listing(&*client, &*config, &name, subreddit).await
				{
					let mut handlers = Vec::new();
					for meta in meta.into_iter() {
						let handle = tokio::spawn(async move {});
						handlers.push(handle);
					}
					for handle in handlers {
						let _ = handle.await;
					}
				}
			});

			handlers.push(handle);
		}
		for handle in handlers {
			let _ = handle.await;
		}
	}

	async fn download_listing(
		client: &Client,
		config: &Configuration,
		name: &str,
		subreddit: Subreddit,
	) -> Result<Vec<DownloadMeta>> {
		let listing_url = format!(
			"https://reddit.com/r/{}/{}.json?limit=100",
			name, subreddit.sort
		);

		let result: Listing = retry(ExponentialBackoff::default(), || async {
			Ok(client
				.get(&listing_url)
				.send()
				.await
				.with_context(|| format!("failed to get response from: {}", listing_url))?
				.json()
				.await
				.with_context(|| {
					format!(
						"failed to deserialize json response body from: {}",
						listing_url
					)
				})?)
		})
		.await?;
		Ok(result.into_download_metas(config))
	}
}
