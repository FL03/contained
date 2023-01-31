/*
    Appellation: Contained <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::states::*;

pub(crate) mod states;

pub mod turing;

pub type StrResult<T = ()> = Result<T, String>;

pub trait Turing {
    type Symbol: turing::Symbolic;

    fn tape(&self) -> &turing::Tape<Self::Symbol>;
}
