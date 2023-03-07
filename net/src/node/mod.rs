/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{client::*, context::*};

pub(crate) mod client;
pub(crate) mod context;

pub mod cli;
pub mod rt;

use crate::NetResult;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node {
    cli: cli::CommandLineInterface,
}

impl Node {
    pub fn new() -> Self {
        let cli = cli::new();
        Self { cli }
    }
    pub async fn start(&self, ctx: Context) -> NetResult {
        ctx.start(self.cli.clone()).await
    }
}
