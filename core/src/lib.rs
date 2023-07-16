/*
    Appellation: actors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{errors::*, primitives::*, specs::*, utils::*};

mod errors;
mod primitives;
mod specs;
mod utils;

pub mod compute;
pub mod connect;
pub mod delay;
pub mod epoch;
pub mod states;
pub mod tasks;

pub mod prelude {
    pub use super::errors::*;
    pub use super::primitives::*;
    pub use super::specs::*;
    pub use super::utils::*;
    pub use super::{
        compute::*, connect::*, delay::*, epoch::*, errors::*, primitives::*, specs::*, states::*,
        tasks::*, utils::*,
    };
}
