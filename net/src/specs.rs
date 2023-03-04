/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use libp2p::{identity::PublicKey, PeerId};

pub trait Peerable {
    fn pk(&self) -> PublicKey;
    fn pid(&self) -> PeerId {
        PeerId::from(self.pk())
    }
}
