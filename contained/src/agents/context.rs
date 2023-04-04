/*
    Appellation: context <agents>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: The context of an agent provides all of the necessary information and resources required for an agent to execute.
*/
use super::Settings;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Context {
    pub cnf: Settings,
}
