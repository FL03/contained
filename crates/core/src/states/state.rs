/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{Id, Message, Timestamp};
use serde::{Deserialize, Serialize};

use strum::{EnumString, EnumVariantNames};

pub trait Stateful<S> {
    type Msg;
    fn message(self) -> Self::Msg;
    fn state(self) -> S;
    fn timestamp(self) -> i64;
}

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    Idle = 1,
    Complete = 2,
    Derive = 3,
    Process = 4,
    Request = 5,
    Response = 6,
}

impl States {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl Default for States {
    fn default() -> Self {
        Self::idle()
    }
}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            crate::fnl_remove(serde_json::to_string(&self).unwrap())
        )
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<T: Default = String> {
    pub id: Id,
    pub message: Message<T>,
    pub state: States,
    pub timestamp: i64,
}

impl<T: Default> State<T> {
    pub fn new(message: Message<T>, state: States) -> Self {
        let id = Id::default();
        let timestamp = Timestamp::default().into();
        Self {
            id,
            message,
            state,
            timestamp,
        }
    }
}

impl<T: Default> Stateful<States> for State<T> {
    type Msg = Message<T>;

    fn message(self) -> Self::Msg {
        self.message
    }

    fn state(self) -> States {
        self.state
    }

    fn timestamp(self) -> i64 {
        self.timestamp
    }
}

impl<T: Default> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<T: Default> std::fmt::Display for State<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl<T: Default> From<States> for State<T> {
    fn from(data: States) -> Self {
        Self::new(Default::default(), data)
    }
}

impl<T: Default> From<State<T>> for States {
    fn from(data: State<T>) -> Self {
        data.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        let a = State::<serde_json::Value>::default();
        let b = a.clone();

        assert_eq!(a, b)
    }
}
