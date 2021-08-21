use std::{
	collections::HashMap,
	default::Default,
	ops::{Deref, DerefMut},
	path::PathBuf,
};

use directories::UserDirs;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Subreddits(HashMap<String, Subreddit>);

impl Deref for Subreddits {
	type Target = HashMap<String, Subreddit>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Subreddits {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Default for Subreddits {
	fn default() -> Self {
		let mut m = Subreddits(HashMap::new());
		m.insert("wallpaper".to_string(), Subreddit::default());
		m.insert("wallpapers".to_string(), Subreddit::default());
		m
	}
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Config {
	download: Download,
	subreddits: Subreddits,
	aspect_ratio: AspectRatio,
	minimum_size: MinimumSize,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AspectRatio {
	pub enable: bool,
	pub height: u32,
	pub width: u32,
	pub range: f32,
}

impl Default for AspectRatio {
	fn default() -> Self {
		AspectRatio {
			enable: true,
			height: 9,
			width: 16,
			range: 0.3,
		}
	}
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct MinimumSize {
	pub enable: bool,
	pub height: u32,
	pub width: u32,
}

impl Default for MinimumSize {
	fn default() -> Self {
		MinimumSize {
			enable: true,
			height: 1080,
			width: 1920,
		}
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct Download {
	pub path: PathBuf,
	pub connect_timeout: u32,
}

impl Default for Download {
	fn default() -> Self {
		let dir = UserDirs::new().expect("failed to determine user directory");
		let dir = dir
			.picture_dir()
			.expect("failed to get user picture directory")
			.to_path_buf();
		Download {
			path: dir.join("ridit"),
			connect_timeout: 10,
		}
	}
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Subreddit {
	pub nsfw: bool,
	pub download_first: bool,
	pub sort: Sort,
}

impl Default for Subreddit {
	fn default() -> Self {
		Subreddit {
			nsfw: true,
			download_first: false,
			sort: Sort::New,
		}
	}
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
	Hot,
	New,
	Rising,
	Controversial,
	Top,
}

impl Default for Sort {
	fn default() -> Self {
		Self::New
	}
}
