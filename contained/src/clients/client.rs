/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::prelude::{EnvId, Error, WorkloadId};
use crate::rt::{layer::Command, Environment, Workload};

use tokio::sync::mpsc;

pub struct Client {
    pub cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(buffer: usize) -> Self {
        Self {
            cmd: mpsc::channel(buffer).0,
        }
    }
    /// Add a workload to the runtime.
    pub async fn add_workload(&mut self, workload: Workload) -> Result<(), Error> {
        self.cmd.send(Command::add_workload(workload)).await?;
        Ok(())
    }
    /// Register an environment (triad) with the runtime.
    pub async fn register(&mut self, env: Environment) -> Result<(), Error> {
        self.cmd.send(Command::register(env)).await?;
        Ok(())
    }
    /// Remove a workload from the runtime.
    pub async fn remove_workload(&mut self, id: WorkloadId) -> Result<(), Error> {
        self.cmd.send(Command::remove_workload(id)).await?;
        Ok(())
    }
    /// Run some workload on some machine in the runtime.
    pub async fn run(&mut self, triad_id: EnvId, workload_id: WorkloadId) -> Result<(), Error> {
        self.cmd.send(Command::run(triad_id, workload_id)).await?;
        Ok(())
    }
    /// Unregister an environment (triad) from the runtime.
    pub async fn unregister(&mut self, id: EnvId) -> Result<(), Error> {
        self.cmd.send(Command::unregister(id)).await?;
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::from(1)
    }
}

impl From<usize> for Client {
    fn from(buffer: usize) -> Self {
        Self {
            cmd: mpsc::channel(buffer).0,
        }
    }
}
