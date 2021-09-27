pub mod hello_world {
	tonic::include_proto!("helloworld");
}

pub mod ridit;

use std::sync::{Arc, Mutex};

use ridit::ridit_proto::ridit_server::RiditServer;
use tonic::transport::Server;

use crate::api::config::config::Config;

use self::ridit::RiditController;

pub async fn start_server(config: Config) -> anyhow::Result<()> {
	let addr = format!("127.0.0.1:{}", config.port).parse()?;

	let config = Arc::new(Mutex::new(config));
	let ridit_server = RiditServer::new(RiditController::new(config.clone()));

	Server::builder()
		.add_service(ridit_server)
		.serve(addr)
		.await?;
	Ok(())
}
