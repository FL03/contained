/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::Settings;
use scsys::prelude::Contextual;
use serde::{Deserialize, Serialize};
use std::{convert::From, path::PathBuf};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Services {
    #[default]
    None = 0,
    Authenticator = 1,
}

#[derive(Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Context {
    pub cnf: Settings,
    pub services: Vec<Services>,
    pub workdir: PathBuf,
}

impl Context {
    pub fn new(
        cnf: Option<Settings>,
        services: Option<Vec<Services>>,
        workdir: Option<PathBuf>,
    ) -> Self {
        Self {
            cnf: cnf.unwrap_or_default(),
            services: services.unwrap_or(vec![Default::default()]),
            workdir: workdir.unwrap_or_else(scsys::project_root),
        }
    }
    pub fn settings(&self) -> &Settings {
        &self.cnf
    }
    pub fn set_settings(&mut self, cnf: Settings) -> &Self {
        self.cnf = cnf;
        self
    }
    pub fn workdir(&self) -> &PathBuf {
        &self.workdir
    }
}

impl Contextual for Context {
    type Cnf = Settings;

    type Ctx = Self;

    fn context(&self) -> &Self::Ctx {
        self
    }
}

impl From<Settings> for Context {
    fn from(data: Settings) -> Self {
        Self::new(Some(data), None, None)
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            scsys::fnl_remove(serde_json::to_string_pretty(self).unwrap())
        )
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            scsys::fnl_remove(serde_json::to_string(self).unwrap())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let a = Context::default();
        let b = a.clone();
        assert_eq!(a, b)
    }
}
