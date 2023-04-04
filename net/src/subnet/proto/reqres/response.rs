/*
    Appellation: response <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Response {
    pub data: String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }
}

impl AsRef<[u8]> for Response {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!({
            "payload": self.data
        });
        write!(f, "{}", msg)
    }
}
