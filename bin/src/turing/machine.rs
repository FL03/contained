/*
    Appellation: machine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::State;
use std::sync::Arc;


pub struct TuringMachine {
    pub state: Arc<State>,
}
