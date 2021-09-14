use serde::Deserialize;

use crate::api::config::config::Config;

use super::download_meta::DownloadMeta;

#[derive(Deserialize)]
pub struct Listing {
	pub data: Data,
}

impl Listing {
	pub fn into_download_metas(self, config: &Config) -> Vec<DownloadMeta> {
		let mut result: Vec<DownloadMeta> = Vec::new();
		for children in self.data.children.into_iter() {
			let data = children.data;
			if data.is_video {
				continue;
			}

			let sub_name = data.subreddit.to_lowercase();

			let sub = config.subreddits.get(&sub_name).unwrap_or_else(|| {
				panic!("subreddit '{}' does not exist in configuration", sub_name)
			});

			if data.over_18 && !sub.nsfw {
				continue;
			}

			let filename = match Listing::get_filename_from_url(&data.url) {
				Some(name) => name,
				None => continue,
			};

			let (width, height) = match data.get_image_size() {
				Some(s) => s,
				// return (1, 1) to prevent panic divide by 0
				None => (1, 1),
			};

			let mut meta = DownloadMeta {
				subreddit_name: data.subreddit,
				post_link: format!("https://reddit.com{}", data.permalink),
				image_width: width,
				image_height: height,
				filename,
				url: data.url,
				nsfw: data.over_18,
				title: data.title,
				author: data.author,
				profile: Vec::new(),
			};

			if sub.download_first {
				result.push(meta);
				continue;
			}

			let mut should_download = false;

			for (profile, setting) in config.settings.iter() {
				if !meta.passed_checks(setting) {
					continue;
				}
				meta.profile.push(profile.to_owned());
				should_download = true;
			}

			if !should_download {
				continue;
			}

			result.push(meta);
		}
		result
	}

	fn get_filename_from_url(url: &str) -> Option<String> {
		let s: String = url.split("/").last().unwrap().split("?").take(1).collect();
		if let Some(ext) = s.split(".").last() {
			if ext.len() > 3 || (ext != "jpg" && ext != "png") {
				return None;
			}
			return Some(s);
		}
		None
	}
}

#[derive(Deserialize)]
pub struct Data {
	pub modhash: String,
	pub dist: i64,
	pub children: Vec<Children>,
	pub after: String,
}

#[derive(Deserialize)]
pub struct Children {
	pub data: ChildrenData,
}

#[derive(Deserialize)]
pub struct ChildrenData {
	pub subreddit: String,
	pub title: String,
	pub post_hint: Option<String>,
	pub created: f64,
	pub over_18: bool,
	pub preview: Option<Preview>,
	pub id: String,
	pub author: String,
	pub permalink: String,
	pub stickied: bool,
	pub url: String,
	pub is_video: bool,
	pub is_gallery: Option<bool>,
}

impl ChildrenData {
	/// Returned tuple looks like this `(width, height)`
	pub fn get_image_size(&self) -> Option<(usize, usize)> {
		if let Some(preview) = &self.preview {
			return preview.get_image_size();
		}
		None
	}
}

#[derive(Deserialize)]
pub struct MediaEmbed {}

#[derive(Deserialize)]
pub struct SecureMediaEmbed {}

#[derive(Deserialize)]
pub struct Gildings {
	pub gid1: Option<i64>,
	pub gid2: Option<i64>,
}

#[derive(Deserialize)]
pub struct Preview {
	pub images: Vec<Image>,
	pub enabled: bool,
}

impl Preview {
	/// tuple looks like this `(width, height)`
	pub fn get_image_size(&self) -> Option<(usize, usize)> {
		if let Some(img) = self.images.get(0) {
			let source = &img.source;
			return Some((source.width, source.height));
		}
		None
	}
}

#[derive(Deserialize)]
pub struct Image {
	pub source: Source,
	pub resolutions: Vec<Resolution>,
	pub id: String,
}

#[derive(Deserialize)]
pub struct Source {
	pub url: String,
	pub width: usize,
	pub height: usize,
}

#[derive(Deserialize)]
pub struct Resolution {
	pub url: String,
	pub width: i64,
	pub height: i64,
}
