/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the client for engaging with an actor
*/
use super::layer::Command;
use crate::prelude::BoxedWasmValue;
use decanter::prelude::H256;
use scsys::prelude::AsyncResult;
use tokio::sync::mpsc;

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
    ) -> AsyncResult {
        self.cmd
            .send(Command::execute(module, function, args, imports))
            .await?;
        Ok(())
    }
    pub async fn include(&mut self, bytes: Vec<u8>) -> AsyncResult {
        self.cmd.send(Command::include(bytes)).await?;
        Ok(())
    }
}
