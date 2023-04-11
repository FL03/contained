/*
    Appellation: tasks <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{manager::*, registry::*};

mod manager;
mod registry;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Task {
    group: String,
    name: String,
}

impl Task {
    pub fn new(group: impl ToString, name: impl ToString) -> Self {
        Self {
            group: group.to_string(),
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.group, self.name)
    }
}
