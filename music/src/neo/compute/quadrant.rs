/*
    Appellation: quadrant <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: each surface is broken down into equal parts called quadrants
*/
use crate::intervals::Interval;
use decanter::prelude::H256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Quadrant {
    interval: Interval,
    hash: H256,
}
