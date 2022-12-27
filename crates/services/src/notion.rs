/*
    Appellation: notin <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use notion::NotionApi;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct Notion {
    api_key: String,
}

impl Notion {
    pub fn new(api_key: Option<String>) -> Self {
        let api_key = api_key.unwrap_or_default();
        Self { api_key }
    }
    pub fn from_env(api_key: Option<&str>) -> Self {
        let api_key = match std::env::var(api_key.unwrap_or("NOTION_API_KEY")) {
            Err(_) => None,
            Ok(v) => Some(v),
        };
        Self::new(api_key)
    }
    pub fn client(&self) -> AsyncResult<NotionApi> {
        let client = NotionApi::new(self.api_key.clone())?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_default() {
        let a = Notion::from_env(None);
        assert!(a.client().is_ok())
    }

}
