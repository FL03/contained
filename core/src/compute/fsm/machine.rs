/*
    Appellation: machine <fsm>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: An implementation of a finite state machine
*/

pub enum States {
    Invalid,
    Valid,
}

impl States {
    pub fn all() -> Vec<States> {
        vec![States::Invalid, States::Valid]
    }
}

pub struct State<T = String> {
    id: String,
    msg: Option<T>,
    state: States,
    ts: i64,
}

pub struct Machine<T> {
    state: State<T>,
    states: Vec<State<T>>,
    transitions: Vec<Box<dyn Fn(&State) -> State>>,
}
