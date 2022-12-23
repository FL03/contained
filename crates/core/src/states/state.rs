/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{Message, Timestamp};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumString, EnumVariantNames};

pub trait Stateful {
    fn name(self) -> String;
    fn state(self) -> Self;
    fn timestamp(self) -> i64;
}

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States<T: Default + Display = Message> {
    Complete(T),
    Derive(T),
    Process(T),
    Request(T),
    Response(T),
    Idle,
}

impl<T: Default + Display> States<T> {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl<T: Default + Display> Default for States<T> {
    fn default() -> Self {
        Self::idle()
    }
}

impl<T: Default + Display> std::fmt::Display for States<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct State<T: Default + Display> {
    pub message: States<T>,
    pub timestamp: Timestamp,
}

impl<T: Default + Display> State<T> {
    pub fn new(message: States<T>) -> Self {
        let timestamp = Timestamp::default();
        Self { message, timestamp }
    }
}

impl<T: Default + Display> Default for State<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
