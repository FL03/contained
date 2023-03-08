/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{constants::*, types::*};

pub(crate) mod constants {
    /// Default address for [libp2p::Multiaddr]
    pub const DEFAULT_MULTIADDR: &str = "/ip4/0.0.0.0/tcp/0";
}

pub(crate) mod types {
    use libp2p::core::{muxing::StreamMuxerBox, transport::Boxed};
    use libp2p::PeerId;

    /// Type alias for a [Boxed] two-tuple, ([PeerId], [StreamMuxerBox])
    pub type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;
    /// Type alias for a [std::error::Error] with [Send] and [Sync] flags enabled
    pub type NetError = Box<dyn std::error::Error + Send + Sync>;
    /// Type alias for a [Result] for a given type with a [NetError]
    pub type NetResult<T = ()> = Result<T, NetError>;
}
