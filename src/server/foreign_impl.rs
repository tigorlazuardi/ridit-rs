use std::collections::HashMap;

use super::ridit_proto::{
	AspectRatio, AspectRatioOptional, MinimumSize, MinimumSizeOptional, ProfileData,
	ProfileListMap, ProfileUpsert,
};
use crate::api::config::{
	config::{read_config, write_config, Config, Settings},
	configuration::Configuration,
};
use tonic::Status;

impl Config {
	/// Server side implementation. Adds new profile.
	pub async fn server_upsert_profile(
		&mut self,
		profile_upsert: ProfileUpsert,
	) -> Result<(), Status> {
		let mut cfg = if let Some(cfg) = self.get(&profile_upsert.name) {
			cfg.to_owned()
		} else {
			Configuration::default()
		};

		if let Some(new_ar) = profile_upsert.aspect_ratio {
			cfg.server_update_aspect_ratio(new_ar);
		}

		if let Some(new_ms) = profile_upsert.minimum_size {
			cfg.server_update_minimum_size(new_ms);
		}

		self.insert(profile_upsert.name, cfg);
		self.server_write_config().await?;
		Ok(())
	}

	pub async fn server_read_config() -> Result<Self, Status> {
		read_config()
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))
	}

	pub async fn server_write_config(&self) -> Result<(), Status> {
		write_config(self)
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))
	}
}

impl Configuration {
	/// Server side implementation. Handles Aspect Ratio gRPC data.
	pub fn server_update_aspect_ratio(&mut self, asp: AspectRatioOptional) {
		self.aspect_ratio.enable = asp.enable.unwrap_or(self.aspect_ratio.enable);
		self.aspect_ratio.height = asp.height.unwrap_or(self.aspect_ratio.height);
		self.aspect_ratio.width = asp.width.unwrap_or(self.aspect_ratio.width);
		self.aspect_ratio.range = asp.range.unwrap_or(self.aspect_ratio.range);
	}

	/// Server side implementation. Handles Minium Size gRPC data.
	pub fn server_update_minimum_size(&mut self, msp: MinimumSizeOptional) {
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

impl From<Settings> for ProfileListMap {
	fn from(settings: Settings) -> Self {
		let mut value = HashMap::new();
		for (k, cfg) in settings.into_iter() {
			value.insert(k, ProfileData::from(cfg));
		}
		ProfileListMap { value }
	}
}
