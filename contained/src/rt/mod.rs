/*
    Appellation: rt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{environment::*, runtime::*, space::*, workload::*};

mod environment;
mod runtime;
mod space;
mod workload;

pub mod layer;

use crate::prelude::{EnvId, Resultant, SpaceId, WorkloadId};
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::oneshot;

pub struct Stack {
    pub envs: RwLock<HashMap<EnvId, oneshot::Sender<Resultant>>>,
    pub spaces: RwLock<HashMap<SpaceId, Space>>,
    pub workloads: RwLock<HashMap<WorkloadId, Workload>>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            envs: RwLock::new(HashMap::new()),
            spaces: RwLock::new(HashMap::new()),
            workloads: RwLock::new(HashMap::new()),
        }
    }
}
