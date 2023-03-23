/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is traditionally used to to describe the first degree (note) of the diatonic scale.
        For our purposes the tonic is used as an extension of the triad, implementing a complete UTM
*/
use super::Triad;
use contained_core::states::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tonic {
    epoch: std::time::Duration,
    state: State,
    triad: Triad,
}
