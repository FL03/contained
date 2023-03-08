/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::Settings;
use crate::peers::Peer;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Context {
    cnf: Settings,
    pub peer: Peer,
}

impl Context {
    pub fn new(cnf: Settings, peer: Peer) -> Self {
        Self { cnf, peer }
    }
    pub fn peer(self) -> Peer {
        self.peer
    }
}
