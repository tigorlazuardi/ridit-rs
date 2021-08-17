use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AspectRatio {
    Enable,
    Disable,
    Height { input: u32 },
    Width { input: u32 },
}

impl AspectRatio {
    pub fn handle(&self, profile: &str) {
        match self {
            &Self::Enable => enable(profile),
            &Self::Disable => disable(profile),
            &Self::Height { input } => height(input, profile),
            &Self::Width { input } => width(input, profile),
        }
    }
}

fn enable(profile: &str) {}

fn disable(profile: &str) {}

fn height(height: u32, profile: &str) {}

fn width(width: u32, profile: &str) {}
