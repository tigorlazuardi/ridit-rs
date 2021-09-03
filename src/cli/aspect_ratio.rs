use anyhow::Result;
use structopt::StructOpt;

use crate::api::config::config::modify_config;

#[derive(StructOpt, Debug, Clone, Copy)]
pub enum AspectRatio {
	/// Enables aspect ratio check
	#[structopt(visible_aliases = &["enabled", "e"])]
	Enable,
	/// Disables aspect ratio check
	#[structopt(visible_aliases = &["disabled", "d"])]
	Disable,
	/// Set aspect ratio range
	#[structopt(visible_alias = "r")]
	Range { input: f32 },
	/// Set aspect ratio height
	#[structopt(visible_alias = "h")]
	Height { input: usize },
	/// Set aspect ratio width
	#[structopt(visible_alias = "w")]
	Width { input: usize },
}

impl AspectRatio {
	pub async fn handle(&self, profile: &str) -> Result<()> {
		match self {
			Self::Enable => self.enable(profile).await?,
			Self::Disable => self.disable(profile).await?,
			&Self::Height { input } => self.height(input, profile).await?,
			&Self::Width { input } => self.width(input, profile).await?,
			&Self::Range { input } => self.range(input, profile).await?,
		};
		Ok(())
	}

	async fn enable(&self, profile: &str) -> Result<()> {
		Ok(modify_config(|cfg| {
			let mut configuration = cfg.get_mut(profile).unwrap();
			configuration.aspect_ratio.enable = true;
			Ok(())
		})
		.await?)
	}

	async fn disable(&self, profile: &str) -> Result<()> {
		Ok(modify_config(|cfg| {
			let mut configuration = cfg.get_mut(profile).unwrap();
			configuration.aspect_ratio.enable = false;
			Ok(())
		})
		.await?)
	}

	async fn height(&self, input: usize, profile: &str) -> Result<()> {
		Ok(modify_config(|cfg| {
			let mut configuration = cfg.get_mut(profile).unwrap();
			configuration.aspect_ratio.height = input;
			Ok(())
		})
		.await?)
	}

	async fn width(&self, input: usize, profile: &str) -> Result<()> {
		Ok(modify_config(|cfg| {
			let mut configuration = cfg.get_mut(profile).unwrap();
			configuration.aspect_ratio.width = input;
			Ok(())
		})
		.await?)
	}

	async fn range(&self, input: f32, profile: &str) -> Result<()> {
		Ok(modify_config(|cfg| {
			let mut configuration = cfg.get_mut(profile).unwrap();
			configuration.aspect_ratio.range = input;
			Ok(())
		})
		.await?)
	}
}
