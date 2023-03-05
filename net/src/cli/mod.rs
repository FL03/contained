/*
    Appellation: cli <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::cmds::*;

pub(crate) mod cmds;

pub mod args;

use crate::NetResult;
use clap::Parser;
use libp2p::Multiaddr;

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
    addr: Option<Multiaddr>,
    #[clap(long)]
    peer: Option<Multiaddr>,
    /// Fixed value to generate deterministic peer ID.
    #[clap(long)]
    seed: Option<u8>,
}

impl CommandLineInterface {
    pub async fn handle(&self) -> NetResult {
        if let Some(cmd) = self.clone().cmd() {
            cmd.handle()?;
        };
        Ok(())
    }
    pub fn cmd(self) -> Option<Command> {
        self.cmd
    }
    pub fn listen(self) -> Option<Multiaddr> {
        self.addr
    }
    pub fn peer(self) -> Option<Multiaddr> {
        self.peer
    }
    pub fn seed(self) -> Option<u8> {
        self.seed
    }
}
