#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::{requests::parseapi::APIInput, utils::User};

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

        let mut max_tokens = self.max_tokens;

        let user = User::get_row_api(apikey.clone()).await?;

        let max_allowed = (user.balance as f32) * (1000000.0 / self.model.price());

        if max_allowed < max_tokens as f32 {
            max_tokens = max_allowed as u32;
        }

        let request = &self.clone().into_provider_request(max_tokens).await;

        let resp = client.post(endpoint).json(request);

        match self.model.provider() {
            AIProvider::Gemini => {
                let output = resp.send().await?.text().await;
                return Ok(output?);
            }
            _ => {
                let output = resp.bearer_auth(apikey).send().await?.text().await;
                return Ok(output?);
            }
        }
    }
}
