/*
    Appellation: turing <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Turing
pub use self::{exec::*, primitives::*, programs::*, specs::*, tape::*, utils::*};

pub(crate) use contained_core as core;

mod exec;
mod primitives;

mod programs;
mod specs;
mod tape;
mod utils;

pub mod errors;
pub mod instructions;
pub mod machine;

pub mod prelude {
    pub use super::errors::*;
    pub use super::exec::*;
    pub use super::instructions::*;
    pub use super::machine::*;
    pub use super::primitives::*;
    pub use super::programs::*;
    pub use super::specs::*;
    pub use super::tape::*;
    pub use super::utils::*;
}
