/*
    Appellation: quadrant <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: each surface is broken down into equal parts called quadrants
*/
use crate::intervals::Interval;
use decanter::prelude::{hasher, H256, Hashable};

pub struct Quadrant {
    interval: Interval,
    hash: H256
}
