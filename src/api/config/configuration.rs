use std::{collections::HashMap, default::Default, path::PathBuf};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

pub type Subreddits = HashMap<String, Subreddit>;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Configuration {
	pub download: Download,
	pub subreddits: Subreddits,
	pub aspect_ratio: AspectRatio,
	pub minimum_size: MinimumSize,
}

impl Default for Configuration {
	fn default() -> Self {
		let mut m: HashMap<String, Subreddit> = HashMap::new();
		m.insert("wallpaper".to_string(), Subreddit::default());
		m.insert("wallpapers".to_string(), Subreddit::default());
		Configuration {
			download: Download::default(),
			subreddits: m,
			aspect_ratio: AspectRatio::default(),
			minimum_size: MinimumSize::default(),
		}
	}
}

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
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

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Copy, Serialize)]
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

#[derive(Deserialize, Debug, Clone, Copy, Serialize)]
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
