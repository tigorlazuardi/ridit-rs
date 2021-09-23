pub mod hello_world {
	tonic::include_proto!("helloworld");
}

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
	async fn say_hello(
		&self,
		request: Request<HelloRequest>,
	) -> Result<Response<HelloReply>, Status> {
		println!("Got a request: {:?}", request);
		let reply = HelloReply {
			message: format!("Hello {}", request.into_inner().name).into(),
		};

		Ok(Response::new(reply))
	}
}

pub async fn start_server() -> anyhow::Result<()> {
	let addr = "127.0.0.1:9090".parse()?;
	let greeter = MyGreeter::default();
	Server::builder()
		.add_service(GreeterServer::new(greeter))
		.serve(addr)
		.await?;
	Ok(())
}
