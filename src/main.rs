pub mod api;
pub mod cli;
pub mod pkg;

use cli::Opt;
use structopt::StructOpt;

fn main() {
	Opt::from_args().execute();
}
