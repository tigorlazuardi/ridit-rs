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

pub static CONFIG_FILENAME: &str = "ridit.toml";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	/// Profile to set configurations to
	pub active: String,
	pub timeout: u32,
	pub settings: HashMap<String, Configuration>,
}

impl Config {
	pub fn get_mut_configuration(&mut self) -> Result<&mut Configuration> {
		let active = self.active.to_owned();
		Ok(self
			.get_mut(&active)
			.with_context(|| format!("profile {} does not exist!", active))?)
	}

	pub fn get_configuration(&self) -> Result<&Configuration> {
		Ok(self
			.get(&self.active)
			.with_context(|| format!("profile {} does not exist!", self.active))?)
	}
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
			timeout: 10,
			settings: m,
		}
	}
}

fn project_dir() -> ProjectDirs {
	ProjectDirs::from("id.web", "tigor", "ridit")
		.context("failed to get project directory")
		.unwrap()
}

fn filename() -> PathBuf {
	let pd = project_dir();
	pd.config_dir().join(CONFIG_FILENAME)
}

pub async fn read_config() -> Result<Config> {
	let filename = filename();
	if !config_exist().await {
		println!(
			"file config does not exist. creating a new config on {}",
			project_dir().config_dir().join(CONFIG_FILENAME).display()
		);
		create_config_dir().await;
		write_config(&Config::default()).await?;
	}
	let content = fs::read_to_string(&filename)
		.await
		.with_context(|| format!("cannot find configuration file in: {}", filename.display()))?;

	let config: Config =
		toml::from_str(&content).context("bad configuration. failed to parse config file")?;
	Ok(config)
}

pub async fn write_config(c: &Config) -> Result<()> {
	let filename = filename();
	let buf = toml::to_string_pretty(c)?;
	fs::write(&filename, &buf).await.with_context(|| {
		format!(
			"failed to write configuration to config directory: {}",
			filename.display(),
		)
	})?;
	Ok(())
}

pub async fn create_config_dir() {
	let pd = project_dir();
	let pd = pd.config_dir();
	fs::create_dir_all(&pd).await.ok();
}

pub async fn config_exist() -> bool {
	let pd = project_dir();
	let pd = pd.config_dir().join(CONFIG_FILENAME);
	fs::metadata(pd).await.is_ok()
}
