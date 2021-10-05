use atty::Stream;
use std::{collections::HashMap, convert::TryInto, sync::Arc};
use tokio::sync::mpsc::{self, UnboundedReceiver};

use anyhow::Result;

use crate::api::{
	config::config::Config,
	reddit::{
		models::download_status::DownloadStatus,
		repository::{PrintOut, Repository},
	},
};

use linya::Progress;
use twox_hash::RandomXxHashBuilder64;

/// Start downloading once
pub async fn start(config: &Config) -> Result<()> {
	let config = Arc::new(config.to_owned());
	let repo = Repository::new(config);

	let text = if atty::is(Stream::Stdout) {
		PrintOut::Bar
	} else {
		PrintOut::Text
	};

	let (tx, rx) = mpsc::unbounded_channel();

	let handle = tokio::spawn(async move { display(rx).await });

	for (meta, operation) in repo.download(text, tx).await.into_iter() {
		if let Err(err) = operation {
			println!(
				"{} {} {}",
				meta.padded_profiles(),
				meta.padded_subreddit_name(),
				err
			);
		}
	}
	handle.await.ok();
	Ok(())
}

async fn display(rx: UnboundedReceiver<DownloadStatus>) {
	if atty::is(Stream::Stdout) {
		display_bar(rx).await;
	} else {
		display_text(rx).await;
	}
}

async fn display_bar(mut rx: UnboundedReceiver<DownloadStatus>) {
	let mut mpb = Progress::new();
	let s = RandomXxHashBuilder64::default();
	let mut bars = HashMap::with_hasher(s);
	while let Some(status) = rx.recv().await {
		if status.download_length == 0 {
			continue;
		}
		if status.finished {
			if let Some(bar) = bars.remove(&status.url) {
				drop(bar);
			}
			continue;
		}
		if let Some(bar) = bars.get(&status.url) {
			mpb.inc_and_draw(bar, status.chunk_length.try_into().unwrap());
		} else {
			let bar = mpb.bar(
				status.download_length.try_into().unwrap(),
				status.cli_label(),
			);
			bars.insert(status.url.to_owned(), bar);
		};
	}
}

async fn display_text(mut rx: UnboundedReceiver<DownloadStatus>) {
	let mut v_err: Vec<DownloadStatus> = Vec::new();

	while let Some(status) = rx.recv().await {
		if status.error.is_some() {
			v_err.push(status);
			continue;
		}
		if status.finished {
			println!("{} finished", status.cli_label());
			continue;
		}

		if status.chunk_length == 0 {
			println!("{} started", status.cli_label());
		}
	}

	for status in v_err {
		eprintln!("{} {}", status.cli_label(), status.error.unwrap());
	}
}
