pub mod aspect_ratio;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct Opt {
    /// Profile to use
    #[structopt(short, long, default_value = "main")]
    profile: String,
    #[structopt(subcommand)]
    subcmd: SubCommand,
}

impl Opt {
    pub fn execute(&self) {
        match self.subcmd {
            SubCommand::AspectRatio(ar) => ar.handle(&self.profile),
        }
    }
}

#[derive(Debug, StructOpt, Clone)]
pub enum SubCommand {
    /// Configures aspect ratio settings
    AspectRatio(aspect_ratio::AspectRatio),
}
