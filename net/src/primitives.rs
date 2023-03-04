/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {}

pub(crate) mod types {
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};

    /// Type alias for a boxed transport
    pub type BoxedTransport = Boxed<(libp2p::PeerId, StreamMuxerBox)>;

    /// Type alias for a [Result]
    pub type Resultant<T = (), E = String> = Result<T, E>;
}
