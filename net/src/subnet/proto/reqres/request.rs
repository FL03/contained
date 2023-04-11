/*
    Appellation: request <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

pub enum Method {
    Get,
    Post,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Request {
    pub payload: String,
}

impl Request {
    pub fn new(data: impl ToString) -> Self {
        Self {
            payload: data.to_string(),
        }
    }
}

impl AsRef<[u8]> for Request {
    fn as_ref(&self) -> &[u8] {
        self.payload.as_bytes()
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.payload)
    }
}
