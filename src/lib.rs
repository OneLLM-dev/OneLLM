#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub endpoint: String,
    pub data: String,
    pub ai_provider: String,
}

impl Input {
    pub async fn get(&self, apikey: String) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let resp = client
            .post(self.endpoint.as_str())
            .bearer_auth(apikey.as_str())
            .json(&self.data)
            .send()
            .await?
            .text()
            .await;

        resp
    }

    pub fn parse_input(input: String) -> Result<Input, serde_json::Error> {
        let result: Result<Input, serde_json::Error> = serde_json::from_str(input.as_str());
        result
    }
}
