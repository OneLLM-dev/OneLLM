use crate::pricing::Model;
use crate::requests::requests::AIProvider;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    pub async fn into_provider_request(self, maxtoken: u32) -> serde_json::Value {
        match self.model.provider() {
            AIProvider::OpenAI => {
                json!({
                    "model": self.model.name(),
                    "messages": self.messages,
                    "temperature": self.temperature,
                    "max_tokens": maxtoken,
                    "top_p": self.top_p,
                    "stop": self.stop_sequences,
                    "stream": self.stream,
                    "frequency_penalty": self.frequency_penalty,
                    "presence_penalty": self.presence_penalty,
                    "n": self.n,
                    "response_format": self.response_format,
                    "seed": self.seed,
                    "tool_choice": self.tool_choice,
                    "tools": self.tools,
                    "user": self.user,
                })
            }
            AIProvider::Anthropic => {
                let messages: Vec<serde_json::Value> = self
                    .messages
                    .into_iter()
                    .map(|msg| {
                        json!({
                            "role": msg.role,
                            "content": msg.content,
                        })
                    })
                    .collect();

                json!({
                    "model": self.model.name(),
                    "messages": messages,
                    "temperature": self.temperature,
                    "max_tokens": maxtoken,
                    "top_p": self.top_p,
                    "stop_sequences": self.stop_sequences,
                    "stream": self.stream,
                    "system": self.system,
                    "top_k": self.top_k,
                })
            }
            AIProvider::Gemini => {
                let contents: Vec<serde_json::Value> = self
                    .messages
                    .into_iter()
                    .map(|msg| {
                        json!({
                            "role": msg.role,
                            "parts": [
                                { "text": msg.content }
                            ]
                        })
                    })
                    .collect();

                json!({
                    "model": self.model.name(),
                    "contents": contents,
                    "safety_settings": self.safety_settings,
                    "generation_config": {
                        "temperature": self.temperature,
                        "top_p": self.top_p,
                        "top_k": self.generation_config.as_ref().map(|cfg| cfg.top_k),
                        "candidate_count": self.generation_config.as_ref().map(|cfg| cfg.candidate_count),
                        "max_output_tokens": maxtoken,
                        "stop_sequences": self.stop_sequences,
                    },
                    "tools": self.tools,
                })
            }
            AIProvider::DeepSeek => {
                let model = match self.model {
                    Model::DeepSeekR1 => "deepseek-reasoner",
                    Model::DeepSeekV3 => "deepseek-chat",
                    _ => panic!("This shouldn't be possible"),
                };
                json!({
                    "model": model,
                    "messages": self.messages,
                    "temperature": self.temperature,
                    "max_tokens": maxtoken,
                    "top_p": self.top_p,
                    "stop": self.stop_sequences,
                    "stream": self.stream,
                    "frequency_penalty": self.frequency_penalty,
                    "presence_penalty": self.presence_penalty,
                    "logprobs": self.logprobs,
                    "top_logprobs": self.top_logprobs,
                    "tools": self.tools,
                })
            }
        }
    }
}
