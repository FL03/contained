/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Settings;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: Settings
}

impl Context {
    pub fn new() -> Self {
        let cnf = Settings::default();
        Self { cnf }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
    pub fn set_settings(&mut self, cnf: Settings) -> &Self {
        self.cnf = cnf;
        self
    }
}

impl From<Settings> for Context {
    fn from(data: Settings) -> Self {
        Self { cnf: data }
    }
}
