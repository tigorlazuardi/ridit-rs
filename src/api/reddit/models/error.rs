use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RedditError {
	pub reason: String,
	pub message: String,
	pub error: u16,
}

impl Display for RedditError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"code: {}, message: {}, reason: {}",
			self.error, self.message, self.reason
		)
	}
}

impl Error for RedditError {}
