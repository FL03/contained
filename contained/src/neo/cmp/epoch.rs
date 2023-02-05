/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use scsys::prelude::Timestamp;
use serde::{Deserialize, Serialize};

/// An [Epoch] consists of a start time and optionally, a duration (seconds). If None, the system assumes an infinite duration
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Epoch(i64, Option<i64>);

impl Epoch {
    pub fn new(ts: i64, duration: Option<i64>) -> Self {
        Self(ts, duration)
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new(Timestamp::default().into(), Some(1000))
    }
}
