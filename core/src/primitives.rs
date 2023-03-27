/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

mod constants {}

mod types {
    use crate::Error;

    /// Type alias for a [Result]
    pub type Resultant<T = (), E = Error> = Result<T, E>;
}
