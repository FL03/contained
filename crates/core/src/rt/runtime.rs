/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::states::State;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Runtime {
    pub(crate) state: Receiver<Arc<State>>,
}

impl Runtime {
    pub fn new(state: Receiver<Arc<State>>) -> Self {
        Self { state }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Self::new(rx)
    }
}
