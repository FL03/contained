/*
    Appellation: session <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    pub timestamp: i64,
}

impl Session {
    pub fn new() -> Self {
        let timestamp = Timestamp::default().into();
        Self { timestamp }
    }
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).unwrap().to_lowercase()
        )
    }
}
