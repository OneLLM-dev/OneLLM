#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::requests::parseapi::{APIInput, Input};

#[derive(Serialize, Deserialize, Debug)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
}

impl APIInput {
    pub async fn get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut endpoint = self.endpoint.clone();
        let apikey = match self.model.provider() {
            AIProvider::OpenAI => std::env::var("OPENAI").expect("Error getting OPENAI apikey"),
            AIProvider::Anthropic => std::env::var("CLAUDE").expect("Error getting CLAUDE apikey"),
            AIProvider::Gemini => {
                let key = std::env::var("GEMINI").expect("Error getting GEMINI apikey");
                endpoint += &format!("?key={}", key);
                key
            }
            AIProvider::DeepSeek => std::env::var("DEEPSEEK").expect("Error getting DS apikey"),
        };
        let client = reqwest::Client::new();

        match self.model.provider() {
            AIProvider::Gemini => {
                let resp = client
                    .post(endpoint)
                    .json(&self.clone().into_provider_request().await)
                    .send()
                    .await?
                    .text()
                    .await;
                return Ok(resp?);
            }
            _ => {
                let resp = client
                    .post(endpoint)
                    .bearer_auth(apikey)
                    .json(&self.clone().into_provider_request().await)
                    .send()
                    .await?
                    .text()
                    .await;

                return Ok(resp?);
            }
        }
    }
}
