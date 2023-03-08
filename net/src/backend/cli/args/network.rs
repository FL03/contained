/*
    Appellation: network <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use clap::Args;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Provide {
    #[]
}
