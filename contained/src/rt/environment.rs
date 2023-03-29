/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A space describes the environment in-which wasm modules may be executed in;

*/
use crate::music::neo::triads::*;
use crate::prelude::{EnvId, Shared, State};
use scsys::prelude::BsonOid;
use std::sync::{Arc, Mutex};
use wasmer::{AsStoreMut, FunctionEnv};

pub struct Snapshot {
    id: EnvId,
    surface: Surface,
    ts: i64,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub id: EnvId,
    pub surface: Shared<Surface>,
}

impl Environment {
    pub fn new(triad: Triad) -> Self {
        Self {
            id: BsonOid::new().to_hex(),
            surface: Arc::new(Mutex::new(Surface::new(triad))),
        }
    }
    pub fn function_env(&self, store: &mut impl AsStoreMut) -> FunctionEnv<Self> {
        FunctionEnv::new(store, self.clone())
    }
    pub fn surface(&self) -> &Shared<Surface> {
        &self.surface
    }
}
