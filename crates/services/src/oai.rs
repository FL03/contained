/*
    Appellation: oai <module>
    Contrib: FL03 <j3mccain@gmail.com> (https://github.com/FL03)
    Description: ... Summary ...
*/
use async_openai as openai;

use openai::{
    types::{CreateCompletionRequest, CreateCompletionResponse},
    Completion,
};
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub struct OpenAI {
    secret_key: String,
}

impl OpenAI {
    pub fn new(secret_key: Option<String>) -> Self {
        let secret_key = secret_key.unwrap_or_default();
        Self { secret_key }
    }
    pub fn from_env(secret_key: Option<&str>) -> Self {
        let secret_key = match std::env::var(secret_key.unwrap_or("OPENAI_API_KEY")) {
            Err(_) => None,
            Ok(v) => Some(v),
        };
        Self::new(secret_key)
    }
    pub fn is_auth(&self) -> bool {
        self.secret_key.len() > 1 && self.secret_key.starts_with("sk-")
    }
    pub fn client(&self) -> openai::Client {
        openai::Client::new().with_api_key(self.secret_key.as_str())
    }
    pub fn create_request(&self, prompt: &str) -> CreateCompletionRequest {
        CreateCompletionRequest {
            model: "text-davinci-003".to_owned(),
            prompt: Some(prompt.to_string()),
            ..Default::default()
        }
    }
    pub async fn response(
        &self,
        req: CreateCompletionRequest,
    ) -> AsyncResult<CreateCompletionResponse> {
        let res = Completion::create(&self.client(), req).await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_default() {
        let a = OpenAI::from_env(Some("OPENAI_SECRET_KEY"));
        assert!(a.is_auth())
    }

    #[tokio::test]
    async fn test_openai_completion() {
        let oai = OpenAI::from_env(Some("OPENAI_SECRET_KEY"));
        let req = oai.create_request("What is music theory?");
        let res = oai.response(req).await;
        assert!(res.is_ok())
    }
}
