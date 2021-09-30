use anyhow::Result;
use structopt::StructOpt;

use crate::api::config::config::{write_config, Config};

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
	Height { input: u32 },
	/// Set aspect ratio width
	#[structopt(visible_alias = "w")]
	Width { input: u32 },
}

impl AspectRatio {
	pub async fn handle(&self, config: &mut Config) -> Result<()> {
		match self {
			Self::Enable => self.enable(config).await?,
			Self::Disable => self.disable(config).await?,
			&Self::Height { input } => self.height(input, config).await?,
			&Self::Width { input } => self.width(input, config).await?,
			&Self::Range { input } => self.range(input, config).await?,
		};
		Ok(())
	}

	async fn enable(&self, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.aspect_ratio.enable = true;
		write_config(config).await?;
		println!(
			"aspect ratio check enabled for '{}'",
			config.focused_profile
		);
		Ok(())
	}

	async fn disable(&self, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.aspect_ratio.enable = false;
		write_config(config).await?;
		println!(
			"aspect ratio check disabled for '{}'",
			config.focused_profile
		);
		Ok(())
	}

	async fn height(&self, input: u32, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.aspect_ratio.height = input;
		write_config(config).await?;
		println!(
			"aspect ratio height is set to '{}' for '{}'",
			input, config.focused_profile
		);
		Ok(())
	}

	async fn width(&self, input: u32, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.aspect_ratio.width = input;
		write_config(config).await?;
		println!(
			"aspect ratio width is set to '{}' for '{}'",
			input, config.focused_profile
		);
		Ok(())
	}

	async fn range(&self, input: f32, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.aspect_ratio.range = input;
		write_config(config).await?;
		println!(
			"aspect ratio range is set to '{}' for '{}'",
			input, config.focused_profile
		);
		Ok(())
	}
}
