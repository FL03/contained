/*
    Appellation: workload <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::WorkloadId;
use scsys::prelude::BsonOid;
use wasmer::Module;

#[derive(Clone, Debug)]
pub struct Workload {
    id: WorkloadId,
    module: Module,
}

impl Workload {
    pub fn new(module: Module) -> Self {
        let id = BsonOid::new().to_hex();
        Self { id, module }
    }

    pub fn id(&self) -> &WorkloadId {
        &self.id
    }

    pub fn module(&self) -> &Module {
        &self.module
    }
}

impl From<Module> for Workload {
    fn from(module: Module) -> Self {
        Self::new(module)
    }
}
