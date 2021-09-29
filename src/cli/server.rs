use std::net::IpAddr;

use anyhow::Result;
use structopt::StructOpt;

use crate::{
	api::config::config::{write_config, Config},
	server,
};

#[derive(Debug, Clone, StructOpt)]
pub enum ServerCMD {
	Start,
	Port { port: u16 },
	IP { ip_addr: IpAddr },
}

impl ServerCMD {
	pub async fn handle(&self, mut config: Config) -> Result<()> {
		match *self {
			ServerCMD::Start => self.start(config).await?,
			ServerCMD::Port { port } => self.port(port, &mut config).await?,
			ServerCMD::IP { ip_addr } => self.ip(ip_addr, &mut config).await?,
		}
		Ok(())
	}

	async fn start(&self, config: Config) -> Result<()> {
		println!(
			"grpc server is running on {}:{}",
			config.server.ip, config.server.port
		);
		server::start_server(config).await?;
		Ok(())
	}

	async fn port(&self, port: u16, config: &mut Config) -> Result<()> {
		config.server.port = port;
		write_config(config).await?;
		println!("server port is set to {}", port);
		Ok(())
	}

	async fn ip(&self, ip: IpAddr, config: &mut Config) -> Result<()> {
		config.server.ip = ip;
		write_config(config).await?;
		println!("server ip is set to {}", ip);
		Ok(())
	}
}
