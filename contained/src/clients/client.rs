/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::prelude::{Error, SpaceId, WorkloadId};
use crate::rt::layer::Command;

use tokio::sync::{mpsc, oneshot};

pub struct Client {
    pub cmd: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(buffer: usize) -> Self {
        Self {
            cmd: mpsc::channel(buffer).0,
        }
    }

    pub async fn add_triad(&mut self, id: SpaceId, value: u32) -> Result<(), Error> {
        self.cmd
            .send(Command::add_triad(id, value, oneshot::channel().0))
            .await?;
        Ok(())
    }

    pub async fn add_workload(&mut self, id: WorkloadId, module: u32) -> Result<(), Error> {
        self.cmd
            .send(Command::add_workload(id, module, oneshot::channel().0))
            .await?;
        Ok(())
    }

    pub async fn remove_triad(&mut self, id: SpaceId) -> Result<(), Error> {
        self.cmd
            .send(Command::remove_triad(id, oneshot::channel().0))
            .await?;
        Ok(())
    }

    pub async fn remove_workload(&mut self, id: WorkloadId) -> Result<(), Error> {
        self.cmd
            .send(Command::remove_workload(id, oneshot::channel().0))
            .await?;
        Ok(())
    }

    pub async fn run_workload(
        &mut self,
        triad_id: SpaceId,
        workload_id: WorkloadId,
    ) -> Result<(), Error> {
        self.cmd
            .send(Command::run_workload(
                triad_id,
                workload_id,
                oneshot::channel().0,
            ))
            .await?;
        Ok(())
    }

    pub async fn send(&mut self, cmd: Command) -> Result<(), Error> {
        self.cmd.send(cmd).await?;
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
