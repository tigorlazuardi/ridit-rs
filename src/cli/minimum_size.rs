use anyhow::Result;
use structopt::StructOpt;

use crate::api::config::config::{write_config, Config};

#[derive(StructOpt, Debug, Clone, Copy)]
pub enum MinimumSize {
	/// Enables minimum size check for focused profile
	#[structopt(visible_aliases = &["enabled", "e"])]
	Enable,
	/// Disables minimum size check for focused profile
	#[structopt(visible_aliases = &["disabled", "d"])]
	Disable,
	/// Set minimum size height for focused profile
	#[structopt(visible_alias = "h")]
	Height { input: usize },
	/// Set minimum size width for focused profile
	#[structopt(visible_alias = "w")]
	Width { input: usize },
}

impl MinimumSize {
	pub async fn handle(&self, config: &mut Config) -> Result<()> {
		match self {
			MinimumSize::Enable => self.enable(config).await?,
			MinimumSize::Disable => self.disable(config).await?,
			MinimumSize::Height { input } => self.height(*input, config).await?,
			MinimumSize::Width { input } => self.width(*input, config).await?,
		}
		Ok(())
	}

	async fn enable(&self, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.minimum_size.enable = true;
		write_config(config).await?;
		println!(
			"minimum size check enabled for '{}'",
			config.focused_profile
		);
		Ok(())
	}

	async fn disable(&self, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.minimum_size.enable = false;
		write_config(config).await?;
		println!(
			"minimum size check disabled for '{}'",
			config.focused_profile
		);
		Ok(())
	}

	async fn height(&self, input: usize, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.minimum_size.height = input;
		write_config(config).await?;
		println!(
			"minimum size height is set to '{}' for '{}'",
			input, config.focused_profile
		);
		Ok(())
	}

	async fn width(&self, input: usize, config: &mut Config) -> Result<()> {
		let cfg = config.get_mut_configuration()?;
		cfg.minimum_size.width = input;
		write_config(config).await?;
		println!(
			"minimum size width is set to '{}' for '{}'",
			input, config.focused_profile
		);
		Ok(())
	}
}
