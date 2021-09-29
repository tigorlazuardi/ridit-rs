use anyhow::Result;
use structopt::StructOpt;

use crate::api::config::config::Config;

use super::subreddit::OutFormat;

#[derive(Debug, StructOpt, Clone)]
pub struct Print {
	/// Selects output format
	#[structopt(long, short, default_value = "toml")]
	pub format: OutFormat,
}

impl Print {
	pub fn print(&self, config: &Config) -> Result<()> {
		Ok(match self.format {
			OutFormat::JSON => {
				let val = serde_json::to_string_pretty(config)?;
				println!("{}", val);
			}
			OutFormat::TOML => {
				let val = toml::to_string_pretty(config)?;
				println!("{}", val);
			}
		})
	}
}
