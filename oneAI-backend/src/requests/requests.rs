#![allow(non_snake_case)]
#[allow(unused)]
use crate::{
    auth::basicauth::update_bal,
    requests::responseparser::{
        anthropic::ClaudeMessageResponse, common::LlmUnifiedResponse, deepseek::DeepSeekResponse,
        gemini::GeminiResponse, openai::OpenAIResponse,
    },
};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{requests::parseapi::APIInput, utils::User};

#[derive(Serialize, Deserialize, Debug)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
}

impl APIInput {
    pub async fn get(
        &self,
        onellm_apikey: String,
    ) -> Result<LlmUnifiedResponse, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
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

        let user = User::get_row_api(onellm_apikey).await?;

        if user.balance <= 1000000 {
            return Err(
                "Insufficient balance, please topup your balance to continue using OneLLM".into(),
            );
        }

        let price = self.model.price();
        let max_allowed = (user.balance as u64) * (1000000 / price as u64);

        if max_allowed < max_tokens as u64 {
            max_tokens = max_allowed as u32;
        }

        let request = &self.clone().into_provider_request(max_tokens).await;

        let resp = client.post(endpoint).json(request);

        let output: Result<String, Error>;

        match self.model.provider() {
            AIProvider::Gemini => {
                //                output = resp.send().await?.text().await;
                return Err("Gemini isn't available at this moment (OneLLM's response)".into());
            }
            AIProvider::Anthropic => {
                output = resp.header("x-api-key", apikey).send().await?.text().await;
            }
            _ => {
                output = resp.bearer_auth(apikey).send().await?.text().await;
            }
        }

        let total: u32;
        let unified_response: LlmUnifiedResponse = match self.model.provider() {
            AIProvider::OpenAI => {
                let openai: OpenAIResponse = from_str(&output?)?;
                total = openai.usage.total_tokens;
                openai.into()
            }
            AIProvider::Anthropic => {
                let claude: ClaudeMessageResponse = from_str(&output?)?;
                total = claude.usage.input_tokens + claude.usage.output_tokens;
                claude.into()
            }
            AIProvider::Gemini => {
                return Err("Gemini isn't available at this moment (OneLLM's response)".into());

                //                let gemini: GeminiResponse = from_str(&output?)?;
                //                let total = gemini
                //                    .usage_metadata
                //                    .as_ref()
                //                    .map(|u| u.total_token_count)
                //                    .unwrap_or(0);
                //
                //                gemini.into()
            }
            AIProvider::DeepSeek => {
                let deepseek: DeepSeekResponse = from_str(&output?)?;
                total = deepseek.usage.total_tokens;
                deepseek.into()
            }
        };
        match update_bal(user.email, -1 * (price * total) as i32).await {
            Some(_) => return Ok(unified_response),
            None => return Err("An Unexpected error occurred".into()),
        }
    }
}
