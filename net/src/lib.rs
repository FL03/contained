/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, peers::*, primitives::*, specs::*, utils::*};

mod errors;
mod peers;
mod primitives;
mod specs;
mod utils;

pub mod events;
pub mod subnet;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[strum(serialize_all = "snake_case")]
pub enum Overlay {
    Mainnet,
    #[default]
    #[strum(serialize = "subnet", serialize = "cluster")]
    Subnet,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NetworkConfig {
    pub addr: Multiaddr,
    pub seed: Option<u8>,
}

impl NetworkConfig {
    pub fn new(addr: Multiaddr, seed: Option<u8>) -> Self {
        Self { addr, seed }
    }
    pub fn set_address(mut self, addr: Multiaddr) -> Self {
        self.addr = addr;
        self
    }
    pub fn set_seed(mut self, seed: Option<u8>) -> Self {
        self.seed = seed;
        self
    }
    pub fn peer(&self) -> Peer {
        if let Some(seed) = self.seed {
            Peer::try_from(seed).unwrap_or_default()
        } else {
            Peer::default()
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            addr: DEFAULT_MULTIADDR.parse().unwrap(),
            seed: None,
        }
    }
}

impl std::fmt::Display for NetworkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct Starter {
    pub cnf: NetworkConfig,
    pub overlay: Overlay,
}

impl Starter {
    pub fn new() -> Self {
        Self {
            cnf: NetworkConfig::default(),
            overlay: Overlay::default(),
        }
    }
    pub fn set_overlay(mut self, overlay: Overlay) -> Self {
        self.overlay = overlay;
        self
    }
    pub fn with_config(mut self, cnf: NetworkConfig) -> Self {
        self.cnf = cnf;
        self
    }

    pub fn start(
        self,
    ) -> (
        subnet::node::Node,
        subnet::Client,
        tokio::sync::mpsc::Receiver<events::NetworkEvent>,
    ) {
        let peer = self.cnf.peer();
        let (chan, cmd, events) = match self.overlay {
            Overlay::Mainnet => todo!("Mainnet overlay"),
            Overlay::Subnet => subnet::node::Channels::with_capacity(9),
        };
        let mut swarm = libp2p::Swarm::from_peer(peer);
        swarm.listen_on(self.cnf.addr).unwrap();
        (
            subnet::node::Node::new(chan, swarm),
            subnet::Client::new(cmd),
            events,
        )
    }
}
