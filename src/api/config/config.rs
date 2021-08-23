use std::{
	collections::HashMap,
	ops::{Deref, DerefMut},
	path::PathBuf,
};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::configuration::Configuration;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	/// Profile to set configurations to
	pub active: String,
	/// Subreddits that will be downloaded by daemon
	pub active_daemon: Vec<String>,
	pub settings: HashMap<String, Configuration>,
}

impl Deref for Config {
	type Target = HashMap<String, Configuration>;

	fn deref(&self) -> &Self::Target {
		&self.settings
	}
}

impl DerefMut for Config {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.settings
	}
}

impl Default for Config {
	fn default() -> Self {
		let mut m: HashMap<String, Configuration> = HashMap::new();
		m.insert("main".to_string(), Configuration::default());
		Config {
			active: "main".to_string(),
			active_daemon: vec!["main".to_string()],
			settings: m,
		}
	}
}

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

/// Reads the whole config and set them as closure parameter. On closure ends, the config variable
/// will be written to disk.
///
/// If returned closure contains Err value, it will short circuit and pass the Err value to the
/// caller function.
pub async fn modify_config<F>(mut f: F) -> Result<()>
where
	F: FnMut(&mut Config) -> Result<()>,
{
	let mut config = read_config().await?;
	f(&mut config)?;
	write_config(&config).await?;
	Ok(())
}

/// Wrapper for `modify_config` function.
///
/// Get specific profile Configuration and write immediately after modification.
/// Immediately short circuit and return error if profile does not exist.
///
/// On closure ends, the changes are written to disk.
///
/// If returned closure contains Err value, it will short circuit and pass the Err value to the
/// caller function.
pub async fn modify_config_profile<F>(profile: &str, mut f: F) -> Result<()>
where
	F: FnMut(&mut Configuration) -> Result<()>,
{
	modify_config(|c| {
		let mut config = c.get_mut(profile).context("profile does not exist")?;
		f(&mut config)?;
		Ok(())
	})
	.await?;
	Ok(())
}

pub async fn create_config_dir() {
	let pd = project_dir();
	let pd = pd.config_dir();
	fs::create_dir_all(&pd).await.ok();
}
