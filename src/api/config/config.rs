use std::{collections::HashMap, path::PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use tokio::fs;

use super::configuration::Configuration;

pub type Config = HashMap<String, Configuration>;

fn project_dir() -> ProjectDirs {
	ProjectDirs::from("id.web", "tigor", "ridit").unwrap()
}

fn filename() -> PathBuf {
	let pd = project_dir();
	pd.config_dir().join("ridit.toml")
}

pub async fn read_config() -> Result<Config> {
	let filename = filename();
	let content = fs::read_to_string(&filename)
		.await
		.context("config file does not exist in path or unreadable")?;

	let config: Config =
		toml::from_str(&content).context("bad configuration. failed to parse config file")?;
	Ok(config)
}

pub async fn write_config(c: &Config) -> Result<()> {
	let filename = filename();
	let buf = toml::to_string_pretty(c)?;
	fs::write(&filename, &buf)
		.await
		.context("failed to write configuration to config directory")?;
	Ok(())
}

pub async fn modify_config<F>(mut f: F) -> Result<()>
where
	F: FnMut(&mut Config) -> Result<()>,
{
	let mut config = read_config().await?;
	f(&mut config)?;
	write_config(&config).await?;
	Ok(())
}

pub async fn create_config_dir() {
	let pd = project_dir();
	let pd = pd.config_dir();
	fs::create_dir_all(&pd).await.ok();
}
