#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub endpoint: String,
    pub data: String,
    pub ai_provider: AIProvider,
}

impl Input {
    #[allow(unused)]
    pub async fn get(&self) -> Result<String, reqwest::Error> {
        let apikey: String;
        match self.ai_provider {
            AIProvider::OpenAI => {
                apikey = std::env::var("OPENAI").expect("Error getting OPENAI apikey")
            }
            AIProvider::Anthropic => {
                apikey = std::env::var("CLAUDE").expect("Error getting CLAUDE apikey")
            }
            AIProvider::Gemini => {
                apikey = std::env::var("GEMINI").expect("Error getting GEMINI apikey")
            }
        }
        let client = reqwest::Client::new();
        let resp = client
            .post(self.endpoint.clone())
            .bearer_auth(apikey)
            .json(&self.data)
            .send()
            .await?
            .text()
            .await;

        resp
    }

    #[allow(unused)]
    pub fn parse_input(input_str: &str) -> Result<Input, serde_json::Error> {
        let result: Result<Input, serde_json::Error> = serde_json::from_str(input_str);

        let input = match result {
            Ok(a) => a,
            Err(e) => return Err(e),
        };

        Ok(input)
    }
}
