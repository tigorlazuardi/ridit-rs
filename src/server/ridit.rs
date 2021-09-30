use std::{
	ops::Add,
	sync::{Arc, Mutex},
};

use crate::api::{
	config::config::{read_config, Config},
	reddit::repository::{PrintOut, Repository},
};

use super::ridit_proto::ridit_server::Ridit;
use super::ridit_proto::{AppState, DownloadStatus as ProtoDownloadStatus, EmptyMsg};
use anyhow::Error;
use chrono::{DateTime, Duration, Local, SecondsFormat, Timelike};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct RiditController {
	config: Arc<Mutex<Config>>,
	state: Arc<Mutex<State>>,
}

#[derive(Debug, Clone)]
pub struct State {
	pub status: u32,
	pub message: String,
	pub next_download_time: DateTime<Local>,
}

impl Default for State {
	fn default() -> Self {
		State {
			status: 0,
			message: "standby".to_string(),
			next_download_time: Local::now()
				.add(Duration::minutes(1))
				.with_second(0)
				.unwrap(),
		}
	}
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
		RiditController {
			config,
			state: Arc::new(Mutex::new(State::default())),
		}
	}
}

#[tonic::async_trait]
impl Ridit for RiditController {
	async fn state(&self, _: Request<EmptyMsg>) -> Result<Response<AppState>, Status> {
		Ok(Response::new(self.state.lock().unwrap().to_owned().into()))
	}

	type TriggerDownloadStream = UnboundedReceiverStream<Result<ProtoDownloadStatus, Status>>;

	async fn trigger_download(
		&self,
		_request: Request<EmptyMsg>,
	) -> Result<Response<Self::TriggerDownloadStream>, Status> {
		let config = read_config()
			.await
			.map_err(|err| Status::failed_precondition(err.to_string()))?;
		let (tx, mut rx) = mpsc::unbounded_channel();
		let (tx_proto, rx_proto) = mpsc::unbounded_channel::<Result<ProtoDownloadStatus, Status>>();

		{
			let mut state = self.state.lock().unwrap();
			*state = State {
				status: 1,
				message: "downloading".to_string(),
				next_download_time: Local::now()
					.add(Duration::minutes(1))
					.with_second(0)
					.unwrap(),
			};
		}

		tokio::spawn({
			let state = self.state.clone();
			async move {
				let repo = Repository::new(Arc::new(config));
				// TODO: add sqlite integration later
				repo.download(PrintOut::None, tx).await;
				*state.lock().unwrap() = State::default();
				Ok::<(), Error>(())
			}
		});

		tokio::spawn(async move {
			while let Some(status) = rx.recv().await {
				tx_proto.send(Ok(status.into())).unwrap();
			}
		});

		Ok(Response::new(UnboundedReceiverStream::new(rx_proto)))
	}
}
