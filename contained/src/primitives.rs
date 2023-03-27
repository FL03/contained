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
    use std::sync::{Arc, Mutex};

    /// A type alias for a boxed `dyn` `std::error::Error` + `Send` + `Sync`.
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// A type alias for a `Result` with the error type `AsyncError`.
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
    /// A type alias for a thread-safe [Vec] of [Mutex]es.
    pub type Sharded<T> = Arc<Vec<Mutex<T>>>;
    /// A type alias for an thread-safe [Mutex].
    pub type Shared<T> = Arc<Mutex<T>>;
    /// A type alias for a workload ID.
    pub type WorkloadId = String;
}
