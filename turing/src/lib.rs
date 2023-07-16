/*
    Appellation: turing <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # Turing
pub use self::{exec::*, primitives::*, programs::*, specs::*, tape::*, utils::*};

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

    pub use super::{
        exec::*, instructions::*, machine::*, primitives::*, programs::*, specs::*, tape::*,
        utils::*,
    };
}
