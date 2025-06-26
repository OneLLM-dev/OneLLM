use crate::pricing::Model;
use serde::{Deserialize, Serialize};

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
    // Common fields
    pub model: Model,
    pub temperature: f64,
    pub stream: bool,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
    pub top_p: f64,
    pub stop_sequences: Vec<String>,
    pub tools: Vec<Tool>,

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
