use anyhow::Result;
use structopt::StructOpt;

use crate::{api::config::config::Config, server};

#[derive(Debug, Clone, StructOpt)]
pub enum ServerCMD {
	Start,
}

impl ServerCMD {
	pub async fn handle(&self, config: Config) -> Result<()> {
		match self {
			ServerCMD::Start => self.start(config).await?,
		}
		Ok(())
	}

	async fn start(&self, config: Config) -> Result<()> {
		println!("grpc server is running on 9876");
		server::start_server(config).await?;
		Ok(())
	}
}
