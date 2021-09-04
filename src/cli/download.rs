use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::api::config::config::{
	modify_config, modify_config_profile, read_config, write_config, Config,
};

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
	pub async fn handle(&self, config: &mut Config) -> Result<()> {
		Ok(match &self {
			Self::Path { input } => Download::path(input, config).await?,
			Self::ConnectTimeout { input } => Download::connect_timeout(*input, config).await?,
			Self::Print(p) => Download::print(p, config).await?,
		})
	}

	async fn path<P: AsRef<Path>>(path: P, config: &mut Config) -> Result<()> {
		let p = path.as_ref().to_path_buf();
		let cfg = config.get_mut_configuration()?;
		cfg.download.path = p.clone();
		write_config(config).await?;
		println!("download path is set to {}", p.display());
		Ok(())
	}

	async fn connect_timeout(input: u32, config: &mut Config) -> Result<()> {
		config.timeout = input;
		write_config(config).await?;
		println!("timeout is set to {} seconds", input);
		Ok(())
	}

	async fn print(opts: &PrintOpts, config: &Config) -> Result<()> {
		let cfg = config.get_configuration()?;
		match opts.out_format {
			OutFormat::TOML => {
				let val = toml::to_string_pretty(&cfg.download)?;
				println!("{}", val);
			}
			OutFormat::JSON => {
				let val = serde_json::to_string_pretty(&cfg.download)?;
				println!("{}", val);
			}
		}
		Ok(())
	}
}
