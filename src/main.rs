pub mod api;
pub mod cli;

use anyhow::Result;
use structopt::StructOpt;
use tokio;

use crate::cli::Opt;

#[tokio::main]
async fn main() -> Result<()> {
	Opt::from_args().execute().await?;
	Ok(())
}
