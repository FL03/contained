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
    pub fn new(data: impl ToString) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}

impl AsRef<[u8]> for Response {
    fn as_ref(&self) -> &[u8] {
        self.data.as_bytes()
    }
}
