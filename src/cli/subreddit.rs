use std::{convert::Infallible, fmt::Display, str::FromStr};
use structopt::StructOpt;

use crate::api::config::{
	config::{modify_config_profile, read_config},
	configuration::Sort,
};
use anyhow::{bail, Context, Result};

use crate::api::config::configuration::Subreddit as SubredditConf;

#[derive(Debug, StructOpt, Clone)]
pub enum Subreddit {
	/// Add subreddit(s) to subscribe
	///
	/// Examples:
	///
	/// adding subreddit using default settings: `ridit subreddit add wallpaper wallpapers`
	#[structopt(visible_aliases = &["insert", "update"])]
	Add(AddSubreddit),
	/// Remove subreddit(s) from subscription
	#[structopt(visible_aliases = &["delete", "rm"])]
	Remove(InputOnly),
	/// List added subreddits
	#[structopt(visible_alias = "ls")]
	List(ListOptions),
}

impl Subreddit {
	pub async fn handle(&self, profile: &str) -> Result<()> {
		Ok(match &self {
			Self::Add(add) => Self::add_subreddit(add, profile).await?,
			Self::Remove(rem) => Self::remove_subreddit(rem, profile).await?,
			Self::List(opts) => Self::list(opts, profile).await?,
		})
	}

	async fn add_subreddit(add: &AddSubreddit, profile: &str) -> Result<()> {
		if add.input.len() < 1 {
			bail!("no new subreddits specified")
		}
		modify_config_profile(profile, |cfg| {
			let mut conf = SubredditConf::default();
			conf.nsfw = !add.no_nsfw;
			conf.download_first = add.download_first;
			conf.sort = add.sort;
			for name in &add.input {
				cfg.subreddits.insert(name.to_owned(), conf);
			}
			Ok(())
		})
		.await?;
		println!("added subreddits: {:?}", add.input);
		Ok(())
	}

	async fn remove_subreddit(remove: &InputOnly, profile: &str) -> Result<()> {
		Ok(modify_config_profile(profile, |cfg| {
			for name in &remove.input {
				cfg.subreddits.remove(name);
			}
			Ok(())
		})
		.await?)
	}

	async fn list(opts: &ListOptions, profile: &str) -> Result<()> {
		let config = read_config().await?;
		let profile_config = config.get(profile).unwrap();

		match opts.out_format {
			OutFormat::JSON => {
				let val = serde_json::to_string_pretty(&profile_config.subreddits)
					.context("failed to serialize subreddits to json format")?;
				println!("{}", val);
			}
			OutFormat::TOML => {
				let val = toml::to_string_pretty(&profile_config.subreddits)
					.context("failed to serialize subreddits to toml format")?;
				println!("{}", val);
			}
		}

		Ok(())
	}
}

#[derive(Debug, StructOpt, Clone)]
pub struct AddSubreddit {
	input: Vec<String>,
	/// Prevent nsfw tagged images from being downloaded.
	#[structopt(short, long)]
	no_nsfw: bool,
	/// Images are downloaded first before being checked for size.
	///
	/// Not all subreddit has metadata for image size. For those kind of subreddits, you have to
	/// download them first before the size can be checked and added to list.
	///
	/// How to know which subreddits have them? Add to subscribe list and see if any images are downloaded
	/// from them. If there's no images downloaded after making sure the settings are correct and the
	/// subreddit is in fact, an images collection subreddit, then enable this flag when adding
	/// subreddit.
	///
	/// You can replace existing subreddit settings using the add command. It will update the
	/// settings instead of adding double entry.
	#[structopt(short, long)]
	download_first: bool,

	/// Sets the sort method. defaults to `new`
	#[structopt(short, long, default_value = "new")]
	sort: Sort,
}

#[derive(Debug, StructOpt, Clone)]
pub struct InputOnly {
	input: Vec<String>,
}

#[derive(Debug, StructOpt, Clone)]
pub struct ListOptions {
	/// Set output format. supported value: json, toml
	#[structopt(short, long, default_value = "toml")]
	out_format: OutFormat,
}

#[derive(Clone, Copy, Debug)]
pub enum OutFormat {
	JSON,
	TOML,
}

impl Default for OutFormat {
	fn default() -> Self {
		Self::TOML
	}
}

impl Display for OutFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::JSON => write!(f, "json"),
			Self::TOML => write!(f, "toml"),
		}
	}
}

impl FromStr for OutFormat {
	type Err = Infallible;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"json" => Ok(Self::JSON),
			"toml" => Ok(Self::TOML),
			_ => Ok(Self::TOML),
		}
	}
}
