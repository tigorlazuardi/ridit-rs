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

	for op in repo.download(PrintOut::Bar).await.into_iter() {
		if let Err(err) = op {
			println!("{:?}", err);
		}
	}
	Ok(())
}
