use crate::api::config::config::{write_config, Config};

use super::ridit_proto::profile_server::Profile;
use super::ridit_proto::{EmptyMsg, ProfileListMap, ProfileRemove, ProfileUpsert, Reply};
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct ProfileController;

impl Reply {
	pub fn new(message: String) -> Self {
		Self { message }
	}

	pub fn acknowledged() -> Self {
		Self::new("acknowledged".to_string())
	}
}

#[tonic::async_trait]
impl Profile for ProfileController {
	async fn upsert(&self, request: Request<ProfileUpsert>) -> Result<Response<Reply>, Status> {
		let mut config = Config::server_read_config().await?;
		config.server_upsert_profile(request.into_inner()).await?;
		Ok(Response::new(Reply::acknowledged()))
	}

	async fn remove(&self, request: Request<ProfileRemove>) -> Result<Response<Reply>, Status> {
		let mut config = Config::server_read_config().await?;
		let req = request.into_inner();
		if let None = config.remove(&req.name) {
			Err(Status::not_found(format!(
				"profile '{}' does not exist in configuration",
				req.name
			)))
		} else {
			write_config(&config)
				.await
				.map_err(|err| Status::failed_precondition(err.to_string()))?;
			Ok(Response::new(Reply::acknowledged()))
		}
	}

	async fn list(&self, _: Request<EmptyMsg>) -> Result<Response<ProfileListMap>, Status> {
		let config = Config::server_read_config().await?;
		// check crate::server::foreign_impl for implementation
		Ok(Response::new(ProfileListMap::from(config.settings)))
	}
}
