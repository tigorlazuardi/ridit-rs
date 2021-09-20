use anyhow::Result;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::api::config::config::{write_config, Config};

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
	/// Sets the download threads
	#[structopt(visible_aliases = &["thr", "thread"])]
	Threads { input: usize },
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
			Self::Threads { input } => Download::threads(*input, config).await?,
		})
	}

	async fn path<P: AsRef<Path>>(path: P, config: &mut Config) -> Result<()> {
		let p = path.as_ref().to_path_buf();
		config.path = p.clone();
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

	async fn threads(input: usize, config: &mut Config) -> Result<()> {
		config.download_threads = input;
		write_config(config).await?;
		println!("download thread is set to {} threads", input);
		Ok(())
	}
}
