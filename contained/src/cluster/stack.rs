/*
    Appellation: stack <cluster>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The stack is a collection of modules and environments that are availible to the cluster
*/
use crate::agents::VirtualEnv;
use crate::prelude::Shared;
use decanter::prelude::H256;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmer::Module;

pub struct Stack {
    pub envs: Shared<HashMap<H256, VirtualEnv>>,
    pub modules: Shared<HashMap<H256, Module>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            envs: Arc::new(Mutex::new(HashMap::new())),
            modules: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn envs(&self) -> &Shared<HashMap<H256, VirtualEnv>> {
        &self.envs
    }
    pub fn modules(&self) -> &Shared<HashMap<H256, Module>> {
        &self.modules
    }
}
