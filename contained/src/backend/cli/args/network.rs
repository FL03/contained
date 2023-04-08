/*
    Appellation: network <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::net::{Multiaddr, PeerId};
use clap::{ArgAction, Args, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{Display, EnumVariantNames};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct NetworkArgs {
    #[clap(subcommand)]
    pub cmd: Option<NetworkOpts>,
    /// The address to listen on
    #[clap(long, short)]
    pub addr: Option<Multiaddr>,
    /// Spawn the node in the background
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub detached: bool,
    /// Provide a seed to generate a deterministic peer ID
    #[clap(long, short)]
    pub seed: Option<u8>,
    /// Start the network
    #[arg(action = ArgAction::SetTrue, long, short)]
    pub up: bool,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Subcommand,
)]
pub enum NetworkOpts {
    Dial {
        #[clap(long, short)]
        addr: Multiaddr,
        #[clap(long, short)]
        pid: PeerId,
    },
    Provide {
        #[clap(long, short)]
        file: Option<PathBuf>,
    },
    Providers {
        #[clap(long, short)]
        cid: String,
    },
}
