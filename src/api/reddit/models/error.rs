use std::fmt::Display;

use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Error)]
pub struct RedditError {
	pub reason: String,
	pub message: String,
	pub error: u16,
}

impl Display for RedditError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			r#"reddit returned [{}: {}] with reason "{}""#,
			self.error, self.message, self.reason
		)
	}
}
