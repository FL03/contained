/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::prelude::BoxedWasmValue;
use scsys::prelude::{AsyncResult, BsonOid};
use std::collections::HashMap;
use tokio::sync::mpsc;
use wasmer::Module;

#[derive(Debug)]
pub struct Client {
    pub cache: HashMap<String, BoxedWasmValue>,
    pub program: mpsc::Sender<Module>,
    pub results: mpsc::Receiver<BoxedWasmValue>,
    pub transform: mpsc::Sender<String>,
}

impl Client {
    pub fn new(
        program: mpsc::Sender<Module>,
        results: mpsc::Receiver<BoxedWasmValue>,
        transform: mpsc::Sender<String>,
    ) -> Self {
        Self {
            cache: HashMap::new(),
            program,
            results,
            transform,
        }
    }
    pub async fn add_workload(&mut self, module: Module) -> AsyncResult {
        self.program.send(module).await?;
        Ok(())
    }
    pub async fn cache_results(&mut self) -> AsyncResult<&HashMap<String, BoxedWasmValue>> {
        while let Some(res) = self.results.recv().await {
            self.cache.insert(BsonOid::new().to_hex(), res);
        }
        Ok(&self.cache)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(mpsc::channel(1).0, mpsc::channel(1).1, mpsc::channel(1).0)
    }
}
