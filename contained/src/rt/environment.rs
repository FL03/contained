/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A space describes the environment in-which wasm modules may be executed in;

*/
use crate::music::neo::triads::*;
use crate::prelude::{EnvId, Shared};
use scsys::prelude::{BsonOid, Timestamp};
use std::sync::{Arc, Mutex};
use wasmer::{AsStoreMut, FunctionEnv};

pub struct Snapshot {
    id: EnvId,
    surface: Surface,
    ts: i64,
}

impl Snapshot {
    pub fn new(env: &Environment) -> Self {
        Self {
            id: env.id.clone(),
            surface: env.surface.lock().unwrap().clone(),
            ts: Timestamp::default().into(),
        }
    }
    pub fn id(&self) -> &EnvId {
        &self.id
    }
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
    pub fn ts(&self) -> i64 {
        self.ts
    }
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
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self)
    }
    pub fn surface(&self) -> &Shared<Surface> {
        &self.surface
    }
}
