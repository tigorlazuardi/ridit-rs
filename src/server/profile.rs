use std::collections::HashMap;

use crate::api::config::{
	config::{read_config, write_config, Config, Settings},
	configuration::Configuration,
};

use super::ridit_proto::profile_server::Profile;
use super::ridit_proto::{
	AspectRatio, AspectRatioOptional, EmptyMsg, MinimumSize, MinimumSizeOptional, ProfileData,
	ProfileListMap, ProfileRemove, ProfileUpsert, Reply,
};
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
		let mut config = read_config()
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))?;

		config.upsert_profile(request.into_inner()).await?;
		Ok(Response::new(Reply::acknowledged()))
	}

	async fn remove(&self, request: Request<ProfileRemove>) -> Result<Response<Reply>, Status> {
		let mut config = read_config()
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))?;

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
		let config = read_config()
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))?;

		Ok(Response::new(ProfileListMap::from(config.settings)))
	}
}

impl Config {
	pub async fn upsert_profile(&mut self, profile_upsert: ProfileUpsert) -> Result<(), Status> {
		let mut cfg = if let Some(cfg) = self.get(&profile_upsert.name) {
			cfg.to_owned()
		} else {
			Configuration::default()
		};

		if let Some(new_ar) = profile_upsert.aspect_ratio {
			cfg.update_aspect_ratio(new_ar);
		}

		if let Some(new_ms) = profile_upsert.minimum_size {
			cfg.update_minimum_size(new_ms);
		}

		self.insert(profile_upsert.name, cfg);

		write_config(self)
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))?;
		Ok(())
	}
}

impl From<Settings> for ProfileListMap {
	fn from(settings: Settings) -> Self {
		let mut value = HashMap::new();
		for (k, cfg) in settings.into_iter() {
			value.insert(k, ProfileData::from(cfg));
		}
		ProfileListMap { value }
	}
}

impl Configuration {
	pub fn update_aspect_ratio(&mut self, asp: AspectRatioOptional) {
		self.aspect_ratio.enable = asp.enable.unwrap_or(self.aspect_ratio.enable);
		self.aspect_ratio.height = asp.height.unwrap_or(self.aspect_ratio.height);
		self.aspect_ratio.width = asp.width.unwrap_or(self.aspect_ratio.width);
		self.aspect_ratio.range = asp.range.unwrap_or(self.aspect_ratio.range);
	}

	pub fn update_minimum_size(&mut self, msp: MinimumSizeOptional) {
		self.minimum_size.enable = msp.enable.unwrap_or(self.minimum_size.enable);
		self.minimum_size.height = msp.height.unwrap_or(self.minimum_size.height);
		self.minimum_size.width = msp.height.unwrap_or(self.minimum_size.width);
	}
}

impl From<Configuration> for ProfileData {
	fn from(cfg: Configuration) -> Self {
		ProfileData {
			aspect_ratio: Some(AspectRatio {
				enable: cfg.aspect_ratio.enable,
				width: cfg.aspect_ratio.width,
				height: cfg.aspect_ratio.height,
				range: cfg.aspect_ratio.range,
			}),
			minimum_size: Some(MinimumSize {
				enable: cfg.minimum_size.enable,
				height: cfg.minimum_size.height,
				width: cfg.minimum_size.width,
			}),
		}
	}
}
