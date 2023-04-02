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
    pub data: String,
}

impl Request {
    pub fn new(data: impl ToString) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}

impl AsRef<[u8]> for Request {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
}
