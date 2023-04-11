/*
    Appellation: args <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{auto::*, build::*, setup::*, test::*};

mod auto;
mod build;
mod setup;
mod test;

use clap::{Args, ArgAction};
use serde::{Deserialize, Serialize};

#[derive(
    Args, Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]

pub struct Cargo {
    
    #[clap(long, short)]
    pub package: Option<String>,
}
