/*
    Appellation: face <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::core::states::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Face<T>(T, State);
