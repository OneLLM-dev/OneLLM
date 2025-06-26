
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeApiInput {
    pub model: String,
    pub messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ClaudeTool>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: Vec<ClaudeContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClaudeContent {
    Text {
        #[serde(rename = "type")]
        content_type: String,
        text: String,
    },
    Image {
        #[serde(rename = "type")]
        content_type: String,
        source: ClaudeImageSource,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeTool {
    pub name: String,
    pub description: String,
    pub input_schema: ClaudeToolInputSchema,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeToolInputSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    pub properties: HashMap<String, ClaudeToolProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeToolProperty {
    #[serde(rename = "type")]
    pub property_type: String,
    pub description: String,
}
