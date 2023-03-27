/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    pub const CONFIG_FNAME_PATTERN: &str = "*.config.toml";
}

pub(crate) mod types {
    /// A type alias for a boxed `dyn` `std::error::Error` + `Send` + `Sync`.
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// A type alias for a `Result` with the error type `AsyncError`.
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
}
