use structopt::StructOpt;

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
	pub fn handle(&self, profile: &str) {
		match self {
			Self::Enable => self.enable(profile),
			Self::Disable => self.disable(profile),
			&Self::Height { input } => self.height(input, profile),
			&Self::Width { input } => self.width(input, profile),
			&Self::Range { input } => self.range(input, profile),
		}
	}

	fn enable(&self, profile: &str) {}

	fn disable(&self, profile: &str) {}

	fn height(&self, input: u32, profile: &str) {}

	fn width(&self, input: u32, profile: &str) {}

	fn range(&self, input: f32, profile: &str) {}
}
