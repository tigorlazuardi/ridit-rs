use anyhow::{bail, Result};
use structopt::StructOpt;

use crate::api::config::{
	config::{write_config, Config},
	configuration::Configuration,
};

use super::{aspect_ratio::AspectRatio, minimum_size::MinimumSize, subreddit::OutFormat, Format};

#[derive(Debug, StructOpt, Clone)]
pub enum Profile {
	/// Sets the main focused profile to set settings with
	#[structopt(visible_alias = "s")]
	Set { profile_name: String },
	/// Shows the focused profile
	#[structopt(visible_aliases = &["a", "show"])]
	Active,
	/// Configures aspect ratio settings for current profile
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
	AspectRatio(AspectRatio),
	/// List all profiles in configuration
	#[structopt(visible_alias = "ls")]
	List(Format),
	/// Add new profile
	Add(AddOption),
	/// Remove profile
	#[structopt(visible_alias = "rm")]
	Remove { profile_name: String },
	/// Configures minimum size image checks for current profile
	MinimumSize(MinimumSize),
}

#[derive(Debug, StructOpt, Clone)]
pub struct AddOption {
	/// Disables checking aspect ratio of images
	#[structopt(long)]
	pub disable_aspect_ratio_check: bool,
	/// aspect ratio height to check
	#[structopt(long, default_value = "9")]
	pub aspect_ratio_height: u32,
	/// aspect ratio width to check
	#[structopt(long, default_value = "16")]
	pub aspect_ratio_width: u32,
	/// aspect ratio range to check
	#[structopt(long, default_value = "0.3")]
	pub aspect_ratio_range: f32,
	/// Disables checking minimum size of images
	#[structopt(long)]
	pub disable_minimum_size_check: bool,
	/// minimum size height to check
	#[structopt(long, default_value = "1080")]
	pub minimum_size_height: u32,
	/// minimum size width to check
	#[structopt(long, default_value = "1920")]
	pub minimum_size_width: u32,
	/// profile name to add
	pub profile_name: String,
}

impl Profile {
	pub async fn handle(&self, config: &mut Config) -> Result<()> {
		match self {
			Profile::AspectRatio(ar) => ar.handle(config).await?,
			Profile::Set { profile_name } => self.set_profile(profile_name, config).await?,
			Profile::Active => println!("{}", config.focused_profile),
			Profile::List(fmt) => self.list_profile(fmt, config)?,
			Profile::Add(ao) => self.add_profile(ao, config).await?,
			Profile::Remove { profile_name } => self.remove_profile(profile_name, config).await?,
			Profile::MinimumSize(ms) => ms.handle(config).await?,
		};
		Ok(())
	}

	async fn set_profile(&self, profile_name: &str, config: &mut Config) -> Result<()> {
		if let None = config.get(profile_name) {
			bail!("profile '{}' does not exist in configuration", profile_name)
		}
		config.focused_profile = profile_name.to_string();
		write_config(config).await?;
		println!("profile is set to '{}'", profile_name);
		Ok(())
	}

	fn list_profile(&self, fmt: &Format, config: &Config) -> Result<()> {
		let text = match fmt.format {
			OutFormat::TOML => toml::to_string_pretty(&config.settings)?,
			OutFormat::JSON => serde_json::to_string_pretty(&config.settings)?,
		};
		println!("{}", text);
		Ok(())
	}

	async fn add_profile(&self, opt: &AddOption, config: &mut Config) -> Result<()> {
		let mut setting = Configuration::default();
		setting.aspect_ratio.enable = !opt.disable_aspect_ratio_check;
		setting.aspect_ratio.height = opt.aspect_ratio_height;
		setting.aspect_ratio.width = opt.aspect_ratio_width;
		setting.minimum_size.enable = !opt.disable_minimum_size_check;
		setting.minimum_size.height = opt.minimum_size_height;
		setting.minimum_size.width = opt.minimum_size_width;

		let text = toml::to_string_pretty(&setting).unwrap();
		println!(
			"added (or replaced) profile: '{}' with setting:\n{}",
			opt.profile_name, text
		);
		config.insert(opt.profile_name.to_owned(), setting);
		write_config(config).await?;
		Ok(())
	}

	async fn remove_profile(&self, input: &str, config: &mut Config) -> Result<()> {
		if let None = config.remove(input) {
			bail!("profile '{}' does not exist in configuration", input)
		}
		write_config(config).await?;
		println!("removed profile '{}'", input);
		Ok(())
	}
}
