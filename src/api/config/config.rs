use std::collections::HashMap;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use tokio::fs;

use super::configuration::Configuration;

pub type Config = HashMap<String, Configuration>;

pub async fn read_config() -> Result<Config> {
	let pd = ProjectDirs::from("id.web", "tigor", "ridit").unwrap();

	let filename = pd.config_dir().join("ridit.toml");
	let content = fs::read_to_string(&filename)
		.await
		.context("config file does not exist in path or unreadable")?;

	let config: Config =
		toml::from_str(&content).context("bad configuration. failed to parse config file")?;
	Ok(config)
}
