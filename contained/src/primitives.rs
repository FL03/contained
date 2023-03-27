/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {
    pub const CONFIG_FNAME_PATTERN: &str = "*.config.toml";
}

mod types {
    /// A type alias for a boxed `dyn` `std::error::Error` + `Send` + `Sync`.
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// A type alias for a `Result` with the error type `AsyncError`.
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
    /// A type alias for a workload ID.
    pub type WorkloadId = String;
}
