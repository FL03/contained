/*
    Appellation: core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{errors::*, primitives::*, specs::*, utils::*};

mod errors;
mod primitives;
mod specs;
mod utils;

pub mod actors;
pub mod compute;
pub mod connect;
pub mod delay;
pub mod epoch;
pub mod states;
pub mod tasks;

pub mod prelude {
    pub use super::actors::*;
    pub use super::compute::*;
    pub use super::connect::*;
    pub use super::delay::*;
    pub use super::epoch::*;
    pub use super::states::*;
    pub use super::tasks::*;

    pub use super::{errors::*, primitives::*, specs::*, utils::*};
}
