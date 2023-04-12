/*
    Appellation: network <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::net::{Multiaddr, PeerId};
use clap::{ArgAction, Args, Subcommand};
use decanter::prelude::Hashable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{Display, EnumVariantNames};

#[derive(
    Args,
    Clone,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
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

impl std::fmt::Display for NetworkArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumVariantNames,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Subcommand,
)]
pub enum NetworkOpts {
    Dial {
        /// The address to dial
        #[clap(long, short)]
        addr: Multiaddr,
        /// The peer ID to dial
        #[clap(long, short)]
        pid: PeerId,
    },
    Provide {
        /// The cid of the file to be provided
        #[clap(long, short)]
        cid: Option<String>,
        /// The path to the file to be provided
        #[clap(long, short)]
        path: Option<PathBuf>,
    },
    Providers {
        /// The CID of the file to find providers for
        #[clap(long, short)]
        cid: String,
    },
}
