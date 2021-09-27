pub mod aspect_ratio;
pub mod download;
pub mod minimum_size;
pub mod print;
pub mod profile;
pub mod server;
pub mod start;
pub mod subreddit;

use anyhow::Result;
use structopt::{
	clap::{crate_authors, crate_version},
	StructOpt,
};

use crate::api::config::config::read_config;

use self::{server::ServerCMD, subreddit::OutFormat};

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "ridit", about = "Reddit image downloader written in rust", version = crate_version!(), author = crate_authors!())]
pub struct Opt {
	#[structopt(subcommand)]
	subcmd: SubCommand,
}

#[derive(Debug, StructOpt, Clone)]
pub struct Format {
	#[structopt(short, long, default_value = "toml")]
	/// Sets otuput format. defaults to TOML.
	format: OutFormat,
}

impl Opt {
	pub async fn execute(&self) -> Result<()> {
		let mut config = read_config().await?;
		match &self.subcmd {
			SubCommand::Profile(p) => p.handle(&mut config).await?,
			SubCommand::Subreddit(sub) => sub.handle(&mut config).await?,
			SubCommand::Download(dl) => dl.handle(&mut config).await?,
			SubCommand::Start => start::start(&config).await?,
			SubCommand::Print(p) => p.print(&config)?,
			SubCommand::Server(cmd) => cmd.handle(config).await?,
		}
		Ok(())
	}
}

#[derive(Debug, StructOpt, Clone)]
pub enum SubCommand {
	/// Add or remove subreddit(s) from subscription.
	///
	/// Example adding a subreddit: `ridit subreddit add wallpaper`
	///
	/// Example adding subreddits while filtering content rated as nsfw:
	/// `ridit subreddit add --no-nsfw wallpaper wallpapers`
	Subreddit(subreddit::Subreddit),
	/// Configures download settings.
	Download(download::Download),
	/// Start the download manually
	Start,
	/// Prints whole configuration
	Print(print::Print),
	/// Runs a downloading server.
	///
	/// Server will run for all profiles
	Server(ServerCMD),
	/// Sets profile specific configuration like aspect ratio and minimum size check
	Profile(profile::Profile),
}
