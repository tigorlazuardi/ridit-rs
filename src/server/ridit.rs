pub mod ridit_proto {
	tonic::include_proto!("ridit");
}

use std::{
	ops::Add,
	sync::{Arc, Mutex},
};

use crate::api::config::config::Config;

use chrono::{DateTime, Duration, Local, SecondsFormat, Timelike};
use ridit_proto::ridit_server::Ridit;
use ridit_proto::{AppState, DownloadStatus, EmptyMsg};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct RiditController {
	config: Arc<Mutex<Config>>,
}

#[derive(Debug)]
pub struct State {
	pub status: u32,
	pub message: String,
	pub next_download_time: DateTime<Local>,
}

impl From<State> for AppState {
	fn from(state: State) -> Self {
		AppState {
			status: state.status,
			message: state.message.to_owned(),
			next_download_time: state
				.next_download_time
				.to_rfc3339_opts(SecondsFormat::Secs, true),
		}
	}
}

impl RiditController {
	pub fn new(config: Arc<Mutex<Config>>) -> RiditController {
		RiditController { config }
	}
}

#[tonic::async_trait]
impl Ridit for RiditController {
	async fn state(&self, _request: Request<EmptyMsg>) -> Result<Response<AppState>, Status> {
		Ok(Response::new(
			State {
				status: 0,
				message: "healthy".to_string(),
				next_download_time: Local::now()
					.add(Duration::minutes(1))
					.with_second(0)
					.unwrap(),
			}
			.into(),
		))
	}

	type TriggerDownloadStream = ReceiverStream<Result<DownloadStatus, Status>>;

	async fn trigger_download(
		&self,
		_request: Request<EmptyMsg>,
	) -> Result<Response<Self::TriggerDownloadStream>, Status> {
		Err(Status::unimplemented("sorry, not ready yet"))
	}
}
