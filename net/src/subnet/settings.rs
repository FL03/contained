/*
    Appellation: settings <subnet>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Network configuration for the subnet
*/
use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ClusterConfig {
    pub addr: Multiaddr,
}

impl ClusterConfig {
    pub fn new() -> Self {
        Self {
            addr: crate::DEFAULT_SUBNET_ADDR.parse().unwrap(),
        }
    }
    pub fn addr(&self) -> Multiaddr {
        self.addr.clone()
    }
    pub fn set_addr(mut self, addr: Multiaddr) -> Self {
        self.addr = addr;
        self
    }
}
