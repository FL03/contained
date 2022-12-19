/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::states::States;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Runtime {
    pub(crate) state: Receiver<Arc<States>>,
}

impl Runtime {
    pub fn new(state: Receiver<Arc<States>>) -> Self {
        Self { state }
    }
}
