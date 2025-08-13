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

use crate::{
    database::init_pool,
    requests::{parseapi::APIInput, responseparser::mistral::MistralResponse},
    utils::User,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
    DeepSeek,
    Mistral,
}

impl APIInput {
    pub async fn get(
        &self,
        onellm_apikey: String,
    ) -> Result<LlmUnifiedResponse, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let pool = init_pool().await?;

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
            AIProvider::Mistral => std::env::var("MISTRAL").expect("Error getting Mistral APIKEY"),
        };
        let client = reqwest::Client::new();

        let mut max_tokens = self.max_tokens;

        let user = User::get_row_api(Some(pool.clone()), onellm_apikey).await?;

        if user.balance <= 1000000 {
            return Err(
                "Insufficient balance, please topup your balance to continue using OneLLM".into(),
            );
        }

        let price = self.model.output_price();
        let max_allowed = (user.balance as u64) * (1000000 / price as u64);

        if max_allowed < max_tokens as u64 {
            max_tokens = max_allowed as u32;
        }

        let request = &self.clone().into_provider_request(max_tokens).await;

        let resp = client.post(endpoint).json(request);

        let output: Result<String, Error>;

        match self.model.provider() {
            AIProvider::Gemini => {
                output = resp.send().await?.text().await;
                //                return Err("Gemini isn't available at this moment (OneLLM's response)".into());
            }
            AIProvider::Anthropic => {
                output = resp
                    .header("x-api-key", apikey)
                    .header("anthropic-version", "2023-06-01")
                    .send()
                    .await?
                    .text()
                    .await;
            }
            _ => {
                output = resp.bearer_auth(apikey).send().await?.text().await;
                dbg!(&output);
            }
        }

        let unified_response: LlmUnifiedResponse = match self.model.provider() {
            AIProvider::OpenAI => {
                let openai: OpenAIResponse = from_str(&output?)?;
                openai.into()
            }
            AIProvider::Anthropic => {
                let claude: ClaudeMessageResponse = from_str(&output?)?;
                claude.into()
            }
            AIProvider::Mistral => {
                let mistral: MistralResponse = from_str(&output?)?;
                mistral.into()
            }
            AIProvider::Gemini => {
                let gemini: GeminiResponse = from_str(&output?)?;

                gemini.into()
            }
            AIProvider::DeepSeek => {
                let deepseek: DeepSeekResponse = from_str(&output?)?;
                deepseek.into()
            }
        };
        let usage = unified_response.usage.as_ref().unwrap();

        let input_cost = self.model.input_price() * usage.input_tokens.unwrap();
        let output_cost = self.model.output_price() * usage.output_tokens.unwrap();

        let total_cost = input_cost + output_cost;

        // This cast is safe only if total_cost <= i32::MAX
        let update_val = -(total_cost as i32);
        match update_bal(Some(pool), user.email, update_val).await {
            Some(_) => Ok(unified_response),
            None => Err("An Unexpected error occurred".into()),
        }
    }
}
