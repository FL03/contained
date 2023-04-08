/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Settings;
use crate::net::peers::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    pub cnf: Settings,
}

impl Context {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }
    pub fn peer(&self) -> Peer {
        if let Some(seed) = self.settings().network.subnet.seed {
            Peer::try_from(seed).unwrap_or_default()
        } else {
            Peer::default()
        }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
}
