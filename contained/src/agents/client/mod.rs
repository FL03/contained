/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the client for engaging with an actor
*/
//! # Client
use super::layer::Command;
use crate::music::neo::LPR;
use crate::prelude::{AsyncResult, BoxedWasmValue};
use decanter::prelude::H256;
use tokio::sync::{mpsc, oneshot};

#[async_trait::async_trait]
pub trait AgentManager: Send + Sync {
    fn sender(&self) -> &mpsc::Sender<Command>;
    async fn execute(
        &mut self,
        module: H256,
        function: String,
        args: BoxedWasmValue,
        imports: Option<wasmer::Imports>,
    ) -> AsyncResult<BoxedWasmValue> {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::execute(module, function, args, imports, tx))
            .await?;
        rx.await?
    }
    async fn include(&mut self, bytes: Vec<u8>) -> AsyncResult<H256> {
        let (tx, rx) = oneshot::channel();
        self.sender().send(Command::include(bytes, tx)).await?;
        rx.await?
    }
    async fn transform(&mut self, id: H256, dirac: LPR) -> AsyncResult {
        let (tx, rx) = oneshot::channel();
        self.sender()
            .send(Command::transform(id, dirac, tx))
            .await?;
        rx.await?
    }
}

impl AgentManager for mpsc::Sender<Command> {
    fn sender(&self) -> &mpsc::Sender<Command> {
        self
    }
}

#[derive(Debug)]
pub struct Client {
    cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(cmd: mpsc::Sender<Command>) -> Self {
        Self { cmd }
    }
    pub fn with_capacity(capacity: usize) -> (Self, mpsc::Receiver<Command>) {
        let (cmd, rx) = mpsc::channel(capacity);
        (Self::new(cmd), rx)
    }
}

impl AgentManager for Client {
    fn sender(&self) -> &mpsc::Sender<Command> {
        &self.cmd
    }
}
