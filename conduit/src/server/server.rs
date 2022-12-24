/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use hyper::server::{conn::AddrIncoming, Builder};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ServerParams {
    pub address: SocketAddr,
}

impl ServerParams {
    pub fn new(host: Option<[u8; 4]>, port: u16) -> Self {
        let address = SocketAddr::from((host.unwrap_or([127, 0, 0, 1]), port));
        Self { address }
    }
    pub fn address(&self) -> SocketAddr {
        self.address
    }
}

impl From<([u8; 4], u16)> for ServerParams {
    fn from(data: ([u8; 4], u16)) -> Self {
        let address = SocketAddr::from(data);
        Self::from(address)
    }
}

impl From<SocketAddr> for ServerParams {
    fn from(address: SocketAddr) -> Self {
        Self { address }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Server {
    pub params: ServerParams,
}

impl Server {
    pub fn new(params: ServerParams) -> Self {
        Self { params }
    }
    pub fn address(&self) -> &SocketAddr {
        &self.params.address
    }
    pub fn builder(&self) -> Builder<AddrIncoming> {
        tracing::debug!("Initializing the server");
        hyper::Server::bind(self.address())
    }
}
