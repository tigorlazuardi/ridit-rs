use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use backoff::future::retry;
use backoff::ExponentialBackoff;
use reqwest::Client;
use tokio::sync::mpsc;

use super::models::download_meta::DownloadMeta;
use crate::api::{
	config::{config::Config, configuration::Configuration},
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

	pub async fn get_download_lists(&self, config: &Configuration) -> Vec<DownloadMeta> {
		let mut result = vec![];

		let (tx, mut rx) = mpsc::unbounded_channel();

		for (name, subreddit) in config.subreddits.iter() {
			let name = name.clone();
			let subreddit = subreddit.clone();
			let client = self.client.clone();
			let tx = tx.clone();
			tokio::spawn(async move {
				let listing_url = format!(
					"https://reddit.com/r/{}/{}.json?limit=100",
					name, subreddit.sort
				);
				let result: Result<Listing> = retry(ExponentialBackoff::default(), || async {
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
				.await;

				tx.send(result).ok();
			});
		}

		while let Some(res) = rx.recv().await {
			match res {
				Ok(listing) => {
					let mut meta = listing.into_download_metas(config);
					result.append(&mut meta);
				}
				Err(e) => println!("{}", e),
			}
		}
		result
	}
}
