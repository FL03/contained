/*
    Appellation: runtime <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::{State, StatePack};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Runtime<S: StatePack> {
    pub(crate) state: Receiver<Arc<State<S>>>,
}

impl<S: StatePack> Runtime<S> {
    pub fn new(state: Receiver<Arc<State<S>>>) -> Self {
        Self { state }
    }
}

impl<S: StatePack> Default for Runtime<S> {
    fn default() -> Self {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
        Self::new(rx)
    }
}
