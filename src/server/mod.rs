pub mod ridit_proto {
	tonic::include_proto!("ridit");
}

pub mod foreign_impl;
pub mod profile;
pub mod ridit;

use std::{
	net::SocketAddr,
	sync::{Arc, Mutex},
};

use ridit_proto::profile_server::ProfileServer;
use ridit_proto::ridit_server::RiditServer;
use tonic::transport::Server;

use crate::api::config::config::Config;

use self::ridit::RiditController;
use profile::ProfileController;

pub async fn start_server(config: Config) -> anyhow::Result<()> {
	let addr = SocketAddr::new(config.server.ip, config.server.port);

	let config = Arc::new(Mutex::new(config));
	let ridit_server = RiditServer::new(RiditController::new(config.clone()));
	let profile_server = ProfileServer::new(ProfileController);

	Server::builder()
		.add_service(ridit_server)
		.add_service(profile_server)
		.serve(addr)
		.await?;
	Ok(())
}
