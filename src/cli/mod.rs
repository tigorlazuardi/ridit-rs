pub mod aspect_ratio;
pub mod download;
pub mod print;
pub mod subreddit;

use anyhow::Result;
use structopt::{
	clap::{crate_authors, crate_version},
	StructOpt,
};

use crate::api::config::config::read_config;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "ridit", about = "Reddit image downloader written in rust", version = crate_version!(), author = crate_authors!())]
pub struct Opt {
	/// Profile to use
	#[structopt(subcommand)]
	subcmd: SubCommand,
}

impl Opt {
	pub async fn execute(&self) -> Result<()> {
		let mut config = read_config().await?;
		match &self.subcmd {
			SubCommand::AspectRatio(aspect) => aspect.handle(&mut config).await?,
			SubCommand::Subreddit(sub) => sub.handle(&mut config).await?,
			SubCommand::Download(dl) => dl.handle(&mut config).await?,
			SubCommand::Start => {}
			SubCommand::Print(p) => p.print(&config)?,
		}
		Ok(())
	}
}

#[derive(Debug, StructOpt, Clone)]
pub enum SubCommand {
	/// Configures aspect ratio settings.
	///
	/// Aspect Ratio handles how `square` the image is. Aspect ratio value is gained by dividing
	/// `width` with `height`.
	///
	/// Aspect ratio with value of 1 is considered square. Value of >1 will prone to landscape images
	/// while <1 will prone to potrait images.
	///
	/// Usually you want to set aspect ratio like your device monitor would
	/// so you can fetch images that will fit nicely as desktop wallpaper for your monitor.
	/// if your monitor is 16x9, then set width to 16, while height to 9.
	///
	/// Range handles if image is within acceptable range of your aspect ratio value.
	///
	/// Let's say you set height to 16, width to 9, and range to 0.3. Your aspect ratio value is
	/// 16/9 = 1.777~. With range value of 0.3, this means you will accept images with aspect ratio between 1.477~ to
	/// 2.077~. An image with resolution of 4500x2000 has aspect ratio value of 4500/2000 = 2.25,
	/// outside range value of 1.477-2.077, meaning the image will be rejected from being downloaded.
	///
	/// High range value means more images, but there will also be more images that may not fit
	/// well with your device monitor. Low range value means more accurate images, but also means lower amount of
	/// images to fetch.
	///
	/// Example commands:
	///
	/// Enabling Aspect Ratio Check: `ridit aspect-ratio enable`
	///
	/// Disabling Aspect Ratio Check: `ridit aspect-ratio disable`
	///
	/// Set Aspect Ratio Height: `ridit aspect-ratio height 9`
	///
	/// Set Aspect Ratio Width: `ridit aspect-ratio width 16`
	AspectRatio(aspect_ratio::AspectRatio),
	/// Add or remove subreddit(s) from subscription.
	///
	/// Example adding a subreddit: `ridit subreddit add wallpaper`
	///
	/// Example adding subreddits while filtering content rated as nsfw:
	/// `ridit subreddit add --no-nsfw wallpaper wallpapers`
	Subreddit(subreddit::Subreddit),
	/// Configures download settings.
	Download(download::Download),
	/// Start the server
	Start,
	/// Print<'a>s whole configuration
	Print(print::Print),
}
