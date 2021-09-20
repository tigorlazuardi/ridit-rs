use std::{convert::Infallible, default::Default, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Configuration {
	pub aspect_ratio: AspectRatio,
	pub minimum_size: MinimumSize,
}

impl Default for Configuration {
	fn default() -> Self {
		Configuration {
			aspect_ratio: AspectRatio::default(),
			minimum_size: MinimumSize::default(),
		}
	}
}

#[derive(Debug, Deserialize, Clone, Copy, Serialize)]
pub struct AspectRatio {
	pub enable: bool,
	pub height: usize,
	pub width: usize,
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
	pub height: usize,
	pub width: usize,
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
pub struct Subreddit {
	pub proper_name: String,
	pub nsfw: bool,
	pub download_first: bool,
	pub sort: Sort,
}

impl Subreddit {
	pub fn new_default(proper_name: String) -> Subreddit {
		Subreddit {
			proper_name,
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

impl Display for Sort {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Hot => write!(f, "hot"),
			Self::New => write!(f, "new"),
			Self::Rising => write!(f, "rising"),
			Self::Controversial => write!(f, "controversial"),
			Self::Top => write!(f, "top"),
		}
	}
}

impl FromStr for Sort {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_lowercase().as_str() {
			"hot" => Self::Hot,
			"rising" => Self::Rising,
			"controversial" => Self::Controversial,
			"top" => Self::Top,
			_ => Self::New,
		})
	}
}
