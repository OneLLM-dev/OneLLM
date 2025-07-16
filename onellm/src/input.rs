#![allow(non_camel_case_types)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Model {
    // ==== OpenAI ====
    #[serde(rename = "GPT-4.1")]
    Gpt4_1,
    #[serde(rename = "GPT-4.1-Mini")]
    Gpt4_1Mini,
    #[serde(rename = "GPT-4.1-Nano")]
    Gpt4_1Nano,
    #[serde(rename = "GPT-o3")]
    GptO3,
    #[serde(rename = "GPT-o4-mini")]
    GptO4Mini,
    #[serde(rename = "GPT-o3-pro")]
    GptO3Pro,
    #[serde(rename = "GPT-4o")]
    Gpt4o,
    #[serde(rename = "GPT-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "GPT-o1")]
    GptO1,
    #[serde(rename = "GPT-o3-DeepResearch")]
    GptO3DeepResearch,
    #[serde(rename = "GPT-o3-Mini")]
    GptO3Mini,
    #[serde(rename = "GPT-o1-Mini")]
    GptO1Mini,

    // ==== Anthropic ====
    #[serde(rename = "Opus-4")]
    ClaudeOpus4,
    #[serde(rename = "Sonnet-4")]
    ClaudeSonnet4,
    #[serde(rename = "Haiku-3.5")]
    ClaudeHaiku3_5,
    #[serde(rename = "Opus-3")]
    ClaudeOpus3,
    #[serde(rename = "Sonnet-3.7")]
    ClaudeSonnet3_7,
    #[serde(rename = "Haiku-3")]
    ClaudeHaiku3,

    // ==== DeepSeek ====
    #[serde(rename = "DeepSeek-Reasoner")]
    DeepSeekR1,
    #[serde(rename = "DeepSeek-Chat")]
    DeepSeekV3,

    // ==== Gemini (Google) ====
    #[serde(rename = "2.5-Flash-preview")]
    Gemini25FlashPreview,
    #[serde(rename = "2.5-Pro-preview")]
    Gemini25ProPreview,
    #[serde(rename = "2.0-Flash")]
    Gemini20Flash,
    #[serde(rename = "2.0-Flash-lite")]
    Gemini20FlashLite,
    #[serde(rename = "1.5-Flash")]
    Gemini15Flash,
    #[serde(rename = "1.5-Flash-8B")]
    Gemini15Flash8B,
    #[serde(rename = "1.5-Pro")]
    Gemini15Pro,

    // ==== Mistral ====
    #[serde(rename = "Mistral-Medium-3")]
    MistralMedium3,
    #[serde(rename = "Magistral-Medium")]
    MagistralMedium,
    #[serde(rename = "Codestral")]
    Codestral,
    #[serde(rename = "Devstral-Medium")]
    DevstralMedium,
    #[serde(rename = "Mistral-Saba")]
    MistralSaba,
    #[serde(rename = "Mistral-Large")]
    MistralLarge,
    #[serde(rename = "Pixtral-Large")]
    PixtralLarge,
    #[serde(rename = "Ministral-8B-24.10")]
    Ministral8B_24_10,
    #[serde(rename = "Ministral-3B-24.10")]
    Ministral3B_24_10,
    #[serde(rename = "Mistral-Small-3.2")]
    MistralSmall3_2,
    #[serde(rename = "Magistral-Small")]
    MagistralSmall,
    #[serde(rename = "Devstral-Small")]
    DevstralSmall,
    #[serde(rename = "Pixtral-12B")]
    Pixtral12B,
    #[serde(rename = "Mistral-NeMo")]
    MistralNemo,
    #[serde(rename = "Mistral-7B")]
    Mistral7B,
    #[serde(rename = "Mixtral-8x7B")]
    Mixtral8x7B,
    #[serde(rename = "Mixtral-8x22B")]
    Mixtral8x22B,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    pub role: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    pub r#type: String,
    pub function: Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SafetySetting {
    pub category: String,
    pub threshold: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfig {
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: u32,
    pub candidate_count: u32,
    pub max_output_tokens: u32,
    pub stop_sequences: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseFormat {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIInput {
    pub endpoint: String,
    // Common fields
    pub model: Model,
    pub temperature: Option<f64>,
    pub stream: Option<bool>,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub top_p: f64,
    pub stop_sequences: Option<Vec<String>>,
    pub tools: Option<Vec<Tool>>,

    // Gemini
    #[serde(rename = "contents")]
    pub contents: Option<Vec<Content>>,
    #[serde(rename = "safety_settings")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(rename = "generation_config")]
    pub generation_config: Option<GenerationConfig>,

    // OpenAI, DeepSeek
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f64>,

    // OpenAI
    pub n: Option<u32>,
    #[serde(rename = "response_format")]
    pub response_format: Option<ResponseFormat>,
    pub seed: Option<u32>,
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    pub user: Option<String>,

    // DeepSeek
    pub logprobs: Option<bool>,
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<u32>,

    // Claude
    pub system: Option<String>,
    #[serde(rename = "top_k")]
    pub top_k: Option<u32>,
}

impl APIInput {
    pub fn new(endpoint: String, model: Model, messages: Vec<Message>, max_tokens: u32) -> Self {
        Self {
            endpoint,
            model,
            messages,
            max_tokens,
            temperature: Some(1.0),
            stream: Some(false),
            top_p: 1.0,
            stop_sequences: None,
            tools: None,
            contents: None,
            safety_settings: None,
            generation_config: None,
            frequency_penalty: None,
            presence_penalty: None,
            n: None,
            response_format: None,
            seed: None,
            tool_choice: None,
            user: None,
            logprobs: None,
            top_logprobs: None,
            system: None,
            top_k: None,
        }
    }
    //    pub fn temperature(&mut self, temp: f64) {
    //        self.temperature = Some(temp);
    //    }
    //
    //    pub fn stop_sequences(&mut self, stop_sequences: Vec<String>) {
    //        self.stop_sequences = Some(stop_sequences);
    //    }

    pub async fn send(self, apikey: String) -> anyhow::Result<crate::output::ApiResponse> {
        let client = reqwest::Client::new();
        let response = client
            .post("https://onellm.dev/api")
            .json(&self)
            .bearer_auth(apikey)
            .send()
            .await?;
        let text = response.text().await?;
        let output = serde_json::from_str(&text)?;

        Ok(output)
    }
}
