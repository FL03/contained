/*
    Appellation: stack <cluster>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The stack is a collection of modules and environments that are availible to the cluster
*/
use crate::agents::VirtualEnv;
use decanter::prelude::H256;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use wasmer::Module;

pub struct Stack {
    pub envs: Arc<RwLock<HashMap<H256, VirtualEnv>>>,
    pub modules: Arc<RwLock<HashMap<H256, Module>>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            envs: Arc::new(RwLock::new(HashMap::new())),
            modules: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn envs(&self) -> &Arc<RwLock<HashMap<H256, VirtualEnv>>> {
        &self.envs
    }
    pub fn modules(&self) -> &Arc<RwLock<HashMap<H256, Module>>> {
        &self.modules
    }
}
