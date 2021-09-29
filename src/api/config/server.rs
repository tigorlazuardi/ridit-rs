use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct ServerConfig {
	pub port: u16,
	pub ip: IpAddr,
}

impl Default for ServerConfig {
	fn default() -> Self {
		ServerConfig {
			port: 9876,
			ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
		}
	}
}
