/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the client for engaging with an actor
*/
pub use self::connect::*;

mod connect;

use super::layer::Command;
use crate::music::neo::LPR;
use crate::prelude::BoxedWasmValue;
use decanter::prelude::H256;
use scsys::prelude::AsyncResult;
use tokio::sync::{mpsc, oneshot};

pub struct Client {
    cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(cmd: mpsc::Sender<Command>) -> Self {
        Self { cmd }
    }
    pub async fn execute(
        &mut self,
        module: H256,
        function: String,
        args: BoxedWasmValue,
        imports: Option<wasmer::Imports>,
    ) -> AsyncResult<BoxedWasmValue> {
        let (tx, rx) = oneshot::channel();
        self.cmd
            .send(Command::execute(module, function, args, imports, tx))
            .await?;
        rx.await?
    }
    pub async fn include(&mut self, bytes: Vec<u8>) -> AsyncResult<H256> {
        let (tx, rx) = oneshot::channel();
        self.cmd.send(Command::include(bytes, tx)).await?;
        rx.await?
    }
    pub async fn transform(&mut self, id: H256, dirac: LPR) -> AsyncResult {
        let (tx, rx) = oneshot::channel();
        self.cmd.send(Command::transform(id, dirac, tx)).await?;
        rx.await?
    }
}
