/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::cmds::*;

pub(crate) mod cmds;

pub mod args;

use crate::{nodes::Client, peers::Peer, NetResult};
use clap::{ArgAction, Parser};
use libp2p::multiaddr::Protocol;
use libp2p::{Multiaddr, PeerId};

pub fn new() -> CommandLineInterface {
    CommandLineInterface::parse()
}

#[derive(Clone, Debug, Eq, Hash, Ord, Parser, PartialEq, PartialOrd)]
#[clap(about, author, long_about = None, version)]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct CommandLineInterface {
    /// Network specific commands
    #[clap(subcommand)]
    cmd: Option<Command>,
    /// Provide an address to listen to
    #[clap(long)]
    listen: Option<Multiaddr>,
    /// Dial up another peer
    #[clap(long)]
    peer: Option<Multiaddr>,
    /// Fixed value to generate deterministic peer ID.
    #[clap(long)]
    seed: Option<u8>,
    /// Startup the network
    #[arg(action = ArgAction::SetTrue, long, short)]
    up: bool
}

impl CommandLineInterface {
    pub fn handle_seed(&self) -> Peer {
        if let Some(seed) = self.clone().seed() {
            seed.try_into().unwrap_or_default()
        } else {
            Peer::default()
        }
    }
    pub async fn handle(&self, client: &mut Client) -> NetResult {
        if let Some(cmd) = self.clone().cmd() {
            match cmd {
                Command::Get { .. } => {}
                Command::Provide { .. } => {}
            }
        };
        let _peer = self.handle_seed();
        // Handle the optional listening address
        if let Some(addr) = self.clone().listen() {
            client.start_listening(addr).await?;
        } else {
            client
                .start_listening(crate::DEFAULT_MULTIADDR.parse()?)
                .await?;
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

impl Default for CommandLineInterface {
    fn default() -> Self {
        Self::parse()
    }
}
