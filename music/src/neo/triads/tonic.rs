/*
    Appellation: tonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A tonic is used to des
*/
use super::Triad;
use crate::{Notable, Note};
use contained_core::states::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Tonic<N: Notable = Note> {
    epoch: std::time::Duration,
    state: State,
    triad: Triad<N>,
}
