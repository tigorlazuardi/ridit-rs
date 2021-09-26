use atty::Stream;
use std::sync::Arc;

use anyhow::Result;

use crate::api::{
	config::config::Config,
	reddit::repository::{PrintOut, Repository},
};

/// Start downloading once
pub async fn start(config: &Config) -> Result<()> {
	let cfg = Arc::new(config.to_owned());
	let repo = Repository::new(cfg.clone());

	let text: PrintOut;

	if atty::is(Stream::Stdout) {
		text = PrintOut::Bar;
	} else {
		text = PrintOut::Text;
	}

	for (meta, operation) in repo.download(text).await.into_iter() {
		if let Err(err) = operation {
			println!(
				"{} {} {}",
				meta.padded_profiles(),
				meta.padded_subreddit_name(),
				err
			)
		}
	}
	Ok(())
}
