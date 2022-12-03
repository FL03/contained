/*
    Appellation: appstate <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Appstate<T: Default + Display = Message> {
    Process(T),
    ReqRes(T),
    Idle,
}

impl<T: Default + Display> Appstate<T> {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl<T: Default + Display> Default for Appstate<T> {
    fn default() -> Self {
        Self::idle()
    }
}

impl<T: Default + Display> std::fmt::Display for Appstate<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
