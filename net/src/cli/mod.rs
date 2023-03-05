/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::cmds::*;

pub(crate) mod cmds;

pub mod args;

use crate::{clients::Client, peer::Peer, NetResult};
use clap::Parser;
use libp2p::{Multiaddr, PeerId};
use libp2p::multiaddr::Protocol;

pub fn new() -> CommandLineInterface {
    CommandLineInterface::parse()
}

#[derive(Clone, Debug, Parser)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    #[clap(subcommand)]
    cmd: Option<Command>,
    #[clap(long)]
    listen: Option<Multiaddr>,
    #[clap(long)]
    peer: Option<Multiaddr>,
    /// Fixed value to generate deterministic peer ID.
    #[clap(long)]
    seed: Option<u8>,
}

impl CommandLineInterface {
    pub async fn handle(&self, client: &mut Client) -> NetResult {
        if let Some(cmd) = self.clone().cmd() {
            match cmd {
                Command::Get { .. } => {},
                Command::Provide { .. } => {}
            }
        };
        let _peer = if let Some(seed) = self.clone().seed() {
            Peer::try_from(seed).unwrap_or_default()
        } else {
            Peer::default()
        };
        // Handle the optional listening address
        if let Some(addr) = self.clone().listen() {
            client.start_listening(addr).await?;
        } else {
            client.start_listening(crate::DEFAULT_MULTIADDR.parse()?).await?;
        }
        // Handle the optional peer address 
        if let Some(addr) = self.clone().peer() {
            let peer_id = match addr.iter().last() {
                Some(Protocol::P2p(hash)) => PeerId::from_multihash(hash).expect("Valid hash."),
                _ => return Err("Expect peer multiaddr to contain peer ID.".into()),
            };
            client.dial(peer_id, addr).await?;
        }
        Ok(())
    }
    pub fn cmd(self) -> Option<Command> {
        self.cmd
    }
    pub fn listen(self) -> Option<Multiaddr> {
        self.listen
    }
    pub fn peer(self) -> Option<Multiaddr> {
        self.peer
    }
    pub fn seed(self) -> Option<u8> {
        self.seed
    }
}
