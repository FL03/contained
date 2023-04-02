/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A space describes the environment in-which wasm modules may be executed in;

*/
use crate::music::neo::triads::*;
use crate::prelude::{EnvId, Shared};
use decanter::prelude::{Hashable, H256};
use scsys::prelude::{BsonOid, Timestamp};
use std::sync::{Arc, Mutex};
use wasmer::{AsStoreMut, FunctionEnv};

pub struct Snapshot {
    hash: H256,
    id: EnvId,
    surface: Surface,
    ts: i64,
}

impl Snapshot {
    pub fn new(env: &Environment) -> Self {
        Self {
            hash: H256::generate(),
            id: env.id.clone(),
            surface: env.surface.lock().unwrap().clone(),
            ts: Timestamp::default().into(),
        }
    }
    pub fn id(&self) -> EnvId {
        self.id.clone()
    }
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
    pub fn ts(&self) -> i64 {
        self.ts
    }
}

#[derive(Clone, Debug, Hashable)]
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
    /// Returns a snapshot of the environment.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot::new(self)
    }
    /// Returns the environment's surface; a surface is a stateful topological representation of the environment
    pub fn surface(&self) -> &Shared<Surface> {
        &self.surface
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(Triad::default())
    }
}

impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!({
            "id": self.id,
            "surface": self.surface.lock().unwrap().clone().to_string()
        });
        write!(f, "{}", msg)
    }
}
