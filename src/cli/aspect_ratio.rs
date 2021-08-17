use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Copy)]
pub enum AspectRatio {
    /// Enables aspect ratio check
    Enable,
    /// Disables aspect ratio check
    Disable,
    /// Set aspect ratio range
    Range { input: f32 },
    /// Set aspect ratio height
    Height { input: u32 },
    /// Set aspect ratio width
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
