/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {}

mod types {
    use crate::Error;
    use std::sync::{Arc, Mutex};

    /// A type alias for a boxed `dyn` `std::error::Error` + `Send` + `Sync`.
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// A type alias for a `Result` with the error type `AsyncError`.
    pub type AsyncResult<T = ()> = std::result::Result<T, AsyncError>;
    /// Type alias for a [Result]
    pub type Resultant<T = (), E = Error> = Result<T, E>;
    /// A type alias for a thread-safe [Vec] of [Mutex]es.
    pub type Sharded<T> = Arc<Vec<Mutex<T>>>;
    /// A type alias for an thread-safe [Mutex].
    pub type Shared<T> = Arc<Mutex<T>>;
}
