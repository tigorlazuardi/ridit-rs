use std::{
	collections::BTreeMap,
	ops::{Deref, DerefMut},
	path::PathBuf,
};

use anyhow::{Context, Result};
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::configuration::{AspectRatio, Configuration, MinimumSize, Subreddit};

pub static CONFIG_FILENAME: &str = "ridit.toml";

pub type Subreddits = BTreeMap<String, Subreddit>;
pub type Settings = BTreeMap<String, Configuration>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	/// Profile to set configurations to
	pub focused_profile: String,
	pub timeout: u32,
	pub download_threads: usize,
	pub path: PathBuf,
	pub settings: Settings,
	pub subreddits: Subreddits,
}

impl Config {
	pub fn get_mut_configuration(&mut self) -> Result<&mut Configuration> {
		let active = self.focused_profile.to_owned();
		Ok(self
			.get_mut(&active)
			.with_context(|| format!("profile {} does not exist!", active))?)
	}

	pub fn get_configuration(&self) -> Result<&Configuration> {
		Ok(self
			.get(&self.focused_profile)
			.with_context(|| format!("profile {} does not exist!", self.focused_profile))?)
	}
}

impl Deref for Config {
	type Target = BTreeMap<String, Configuration>;

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
		let mut m: BTreeMap<String, Configuration> = BTreeMap::new();
		m.insert("main".to_string(), Configuration::default());
		let mut subs: Subreddits = BTreeMap::new();
		let wallpaper = "wallpaper".to_string();
		let wallpapers = "wallpapers".to_string();
		subs.insert(wallpaper.clone(), Subreddit::new_default(wallpaper));
		subs.insert(wallpapers.clone(), Subreddit::new_default(wallpapers));
		subs.insert(
			"mobilewallpaper".to_string(),
			Subreddit::new_default(String::from("MobileWallpaper")),
		);
		let mobile_config = Configuration {
			aspect_ratio: AspectRatio {
				enable: true,
				height: 16,
				width: 9,
				range: 0.3,
			},
			minimum_size: MinimumSize {
				enable: true,
				height: 1920,
				width: 1080,
			},
		};
		m.insert("mobile".to_string(), mobile_config);
		let p = UserDirs::new()
			.expect("cannot find user directory for current user")
			.picture_dir()
			.expect("cannot find picture directory for current user")
			.to_path_buf()
			.join("ridit");
		Config {
			focused_profile: "main".to_string(),
			path: p,
			download_threads: 4,
			timeout: 10,
			settings: m,
			subreddits: subs,
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
