/*
    Appellation: stack <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The stack is a collection of modules and environments that are availible to the agent.
*/
//! # Stack
//!
//! The stack is a collection of modules and environments that are availible to the agent.
use crate::prelude::hash_module;
use decanter::prelude::H256;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use wasmer::Module;

#[derive(Clone)]
pub struct Stack {
    modules: Arc<RwLock<HashMap<H256, Module>>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn modules(&self) -> &Arc<RwLock<HashMap<H256, Module>>> {
        &self.modules
    }

    pub fn add_module(&self, module: Module) -> H256 {
        let hash = hash_module(&module);
        self.modules.write().unwrap().insert(hash.clone(), module);
        hash
    }
}
