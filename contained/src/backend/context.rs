/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use super::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    cnf: Settings,
}

impl Context {
    pub fn new(cnf: Settings) -> Self {
        Self { cnf }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
}
