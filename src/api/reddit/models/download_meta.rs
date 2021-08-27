use std::path::{Path, PathBuf};

use crate::api::config::configuration::Configuration;

pub struct DownloadMeta {
	pub url: String,
	pub subreddit_name: String,
	pub image_height: u32,
	pub image_width: u32,
	pub post_link: String,
	pub nsfw: bool,
	pub filename: String,
	pub title: String,
	pub author: String,
}

impl DownloadMeta {
	pub fn get_file_location<P: AsRef<Path>>(&self, base_location: P) -> PathBuf {
		Path::new(base_location.as_ref())
			.join(&self.subreddit_name)
			.join(&self.filename)
			.to_path_buf()
	}

	pub fn passed_checks(&self, config: &Configuration) -> bool {
		self.passed_aspect_ratio(config) && self.passed_mininum_size(config)
	}

	pub fn passed_aspect_ratio(&self, config: &Configuration) -> bool {
		if !config.aspect_ratio.enable {
			return true;
		}
		let ar = config.aspect_ratio.width as f32 / config.aspect_ratio.height as f32;
		let min_ratio = ar - config.aspect_ratio.range;
		let max_ratio = ar + config.aspect_ratio.range;
		let image_ratio = self.image_width as f32 / self.image_height as f32;
		image_ratio >= min_ratio && image_ratio <= max_ratio
	}

	pub fn passed_mininum_size(&self, config: &Configuration) -> bool {
		if !config.minimum_size.enable {
			return true;
		}
		self.image_width >= config.minimum_size.width
			&& self.image_height >= config.minimum_size.height
	}
}
