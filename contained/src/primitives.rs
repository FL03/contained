/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    pub const CONFIG_FNAME_PATTERN: &str = "*.config.toml";
}

pub(crate) mod types {}
