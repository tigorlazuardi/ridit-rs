pub mod api;
pub mod cli;

use cli::Opt;
use structopt::StructOpt;

fn main() {
    Opt::from_args().execute();
}
