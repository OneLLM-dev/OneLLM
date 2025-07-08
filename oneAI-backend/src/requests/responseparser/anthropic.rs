use crate::requests::responseparser::common::{LlmUnifiedResponse, LlmUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeMessageResponse {
    pub content: Vec<ClaudeContent>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub message_type: String,
    pub usage: ClaudeUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

impl From<ClaudeMessageResponse> for LlmUnifiedResponse {
    fn from(res: ClaudeMessageResponse) -> Self {
        let content = res
            .content
            .iter()
            .map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        LlmUnifiedResponse {
            provider: "Claude".into(),
            model: res.model,
            role: Some(res.role),
            content,
            usage: Some(LlmUsage {
                input_tokens: Some(res.usage.input_tokens),
                output_tokens: Some(res.usage.output_tokens),
                total_tokens: Some(res.usage.input_tokens + res.usage.output_tokens),
            }),
            finish_reason: res.stop_reason,
        }
    }
}
