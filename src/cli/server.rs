use anyhow::Result;
use structopt::StructOpt;

use crate::{api::config::config::Config, server};

#[derive(Debug, Clone, StructOpt)]
pub enum ServerCMD {
	Start,
}

impl ServerCMD {
	pub async fn handle(&self, config: &Config) -> Result<()> {
		match self {
			ServerCMD::Start => self.start(config).await?,
		}
		Ok(())
	}

	async fn start(&self, _: &Config) -> Result<()> {
		println!("grpc server is running on 9090");
		server::start_server().await?;
		Ok(())
	}
}
