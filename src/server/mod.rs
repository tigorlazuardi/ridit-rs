pub mod ridit_proto {
	tonic::include_proto!("ridit");
}

pub mod profile;
pub mod ridit;

use std::{
	net::SocketAddr,
	sync::{Arc, Mutex},
};

use ridit_proto::ridit_server::RiditServer;
use tonic::transport::Server;

use crate::api::config::config::Config;

use self::ridit::RiditController;

pub async fn start_server(config: Config) -> anyhow::Result<()> {
	let addr = SocketAddr::new(config.server.ip, config.server.port);

	let config = Arc::new(Mutex::new(config));
	let ridit_server = RiditServer::new(RiditController::new(config.clone()));

	Server::builder()
		.add_service(ridit_server)
		.serve(addr)
		.await?;
	Ok(())
}
