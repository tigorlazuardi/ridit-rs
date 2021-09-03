use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::api::config::config::{modify_config, modify_config_profile, read_config};

use super::subreddit::OutFormat;

#[derive(StructOpt, Debug, Clone)]
pub enum Download {
	/// Sets download path
	#[structopt(visible_alias = "p")]
	Path {
		#[structopt(parse(from_os_str))]
		input: PathBuf,
	},
	/// Sets connect timeout (in seconds)
	#[structopt(visible_aliases = &["ct", "connect"])]
	ConnectTimeout { input: u32 },
	/// Prints download configuration
	#[structopt(visible_aliases = &["ls", "list"])]
	Print(PrintOpts),
}

#[derive(StructOpt, Debug, Clone)]
pub struct PrintOpts {
	#[structopt(long, short, default_value = "toml")]
	pub out_format: OutFormat,
}

impl Download {
	pub async fn handle(&self, profile: &str) -> Result<()> {
		Ok(match &self {
			Self::Path { input } => Download::path(input, profile).await?,
			Self::ConnectTimeout { input } => Download::connect_timeout(*input, profile).await?,
			Self::Print(p) => Download::print(p, profile).await?,
		})
	}

	async fn path<P: AsRef<Path>>(path: P, profile: &str) -> Result<()> {
		let p = path.as_ref().to_path_buf();
		modify_config_profile(profile, |cfg| {
			cfg.download.path = p.clone();
			Ok(())
		})
		.await?;
		println!("download path is set to {}", p.display());
		Ok(())
	}

	async fn connect_timeout(input: u32, profile: &str) -> Result<()> {
		Ok(modify_config(|c| Ok(c.timeout = input)).await?)
	}

	async fn print(opts: &PrintOpts, profile: &str) -> Result<()> {
		let c = read_config().await?;
		let cfg = c
			.get(profile)
			.with_context(|| format!(r#"profile "{}" does not exist in configuration"#, profile))?;
		match opts.out_format {
			OutFormat::JSON => {
				let val = serde_json::to_string_pretty(&cfg.download)?;
				println!("{}", val)
			}
			OutFormat::TOML => {
				let val = toml::to_string_pretty(&cfg.download)?;
				println!("{}", val)
			}
		}
		Ok(())
	}
}
