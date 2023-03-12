/*
    Appellation: surface <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A surface is a sub-graph, whose depth is the hash of its data and header
        The data stored on the tonnetz is a block of sorts, built with the hash of the two connected machines.
*/
use crate::intervals::Interval;
use decanter::prelude::{hasher, Hashable, H256};
use scsys::prelude::{SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn edge_hash(interval: Interval, ts: i64) -> H256 {
    hasher(&json!({
        "interval": interval,
        "ts": ts
    }))
    .into()
}

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, SerdeDisplay, Serialize,
)]
pub struct Link {
    interval: Interval,
    hash: H256,
    ts: i64,
}

impl Link {
    pub fn new(interval: Interval) -> Self {
        let ts: i64 = Timestamp::default().into();
        let hash: H256 = edge_hash(interval.clone(), ts);
        Self { interval, hash, ts }
    }
}

impl Hashable for Link {
    fn hash(&self) -> H256 {
        self.hash.clone()
    }
}
