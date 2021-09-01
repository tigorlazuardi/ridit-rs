use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub enum Download {
	/// Sets download path
	#[structopt(visible_alias = "p")]
	Path {
		#[structopt(parse(from_os_str))]
		input: PathBuf,
	},
	/// Sets connect timeout (in seconds)
	#[structopt(visible_aliases = &["ct", "connect"])]
	ConnectTimeout { input: usize },
	/// Prints download configuration
	#[structopt(visible_aliases = &["ls", "list"])]
	Print(PrintOpts),
}

#[derive(StructOpt, Debug, Clone)]
pub struct PrintOpts {
	#[structopt(long, short)]
	pub out_format: String,
}

impl Download {
	pub fn handle(&self, profile: &str) {
		match &self {
			Self::Path { input } => Download::path(input, profile),
			Self::ConnectTimeout { input } => Download::connect_timeout(*input),
			Self::Print(p) => Download::print(p, profile),
		}
	}

	fn path<P: AsRef<Path>>(path: P, profile: &str) {}

	fn connect_timeout(input: usize) {}

	fn print(opts: &PrintOpts, profile: &str) {}
}
