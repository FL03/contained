/*
    Appellation: machine <fsm>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An implementation of a finite state machine
*/

pub enum Event {
    Process,
    Start,
    Stop,
    Transition,
}

pub enum State {
    Invalid,
    Valid
}

pub struct Machine {
    state: State,
    states: Vec<State>,
    transitions: Vec<Box<dyn Fn(&State) -> State>>,
}
