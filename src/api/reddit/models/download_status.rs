use crate::server::ridit_proto::DownloadStatus as ProtoDownloadStatus;

use pad::PadStr;

#[derive(Clone, Debug)]
pub struct DownloadStatus {
	pub subreddit_name: String,
	pub profiles: Vec<String>,
	pub download_length: u64,
	pub chunk_length: u64,
	pub finished: bool,
	pub error: Option<String>,
	pub url: String,
}

impl DownloadStatus {
	pub fn new(
		subreddit_name: String,
		profiles: Vec<String>,
		download_length: u64,
		chunk_length: u64,
		url: String,
	) -> Self {
		Self {
			subreddit_name,
			profiles,
			download_length,
			chunk_length,
			url,
			finished: false,
			error: None,
		}
	}

	/// Givem error to self and set to finished
	pub fn with_error(mut self, error: String) -> Self {
		self.error = Some(error);
		self.set_finished()
	}

	pub fn set_finished(mut self) -> Self {
		self.finished = true;
		self
	}

	pub fn cli_label(&self) -> String {
		let profiles = format!("{:?}", self.profiles).pad_to_width(23);
		let subreddit_name = ("[".to_string() + &self.subreddit_name + "]").pad_to_width(23);
		format!(
			"{} {} {}",
			profiles,
			subreddit_name,
			self.url.with_exact_width(35)
		)
	}
}

impl From<DownloadStatus> for ProtoDownloadStatus {
	fn from(ds: DownloadStatus) -> Self {
		ProtoDownloadStatus {
			subreddit_name: ds.subreddit_name,
			profiles: ds.profiles,
			download_length: ds.download_length,
			chunk_length: ds.chunk_length,
			finished: ds.finished,
			error: ds.error,
		}
	}
}
