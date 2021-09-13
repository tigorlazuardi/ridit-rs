use std::sync::Arc;

use anyhow::Result;

use crate::api::{config::config::Config, reddit::repository::Repository};

/// Start downloading once
pub async fn start(config: &Config) -> Result<()> {
	let cfg = Arc::new(config.to_owned());
	let repo = Repository::new(cfg.clone());

	let configuration = Arc::new(cfg.get_configuration()?.to_owned());
	repo.download(configuration).await;
	Ok(())
}
