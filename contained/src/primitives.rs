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
    /// A type alias for a space identifier.
    pub type SpaceId = String;
    /// A type alias for a workload ID.
    pub type WorkloadId = String;
}
