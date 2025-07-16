use crate::requests::responseparser::common::{LlmUnifiedResponse, LlmUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    #[serde(rename = "created")]
    pub created_at: u64,
    pub model: String,
    pub choices: Vec<OpenAIOutput>,
    pub usage: OpenAIUsage,
    // Optional fields for completeness
    pub service_tier: Option<String>,
    pub system_fingerprint: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIOutput {
    pub index: u32,
    pub finish_reason: Option<String>,
    pub message: OpenAIMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    #[serde(default)]
    pub prompt_tokens_details: OpenAIInputTokensDetails,
    #[serde(default)]
    pub completion_tokens_details: OpenAIOutputTokensDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
    pub refusal: Option<serde_json::Value>,
    pub function_call: Option<serde_json::Value>,
    pub tool_calls: Option<Vec<serde_json::Value>>,
    pub parsed: Option<serde_json::Value>,
    #[serde(default)]
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIContent {
    pub r#type: String,
    pub text: String,
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAIReasoning {
    pub effort: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAITextField {
    pub format: OpenAIFormat,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAIFormat {
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAIInputTokensDetails {
    pub cached_tokens: u32,
    pub audio_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenAIOutputTokensDetails {
    pub reasoning_tokens: u32,
    pub audio_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMetadata {} // Empty object

impl From<OpenAIResponse> for LlmUnifiedResponse {
    fn from(res: OpenAIResponse) -> Self {
        let first = res.choices.get(0);

        let content = first
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        let role = first
            .map(|c| c.message.role.clone());

        let finish_reason = first
            .and_then(|c| c.finish_reason.clone());

        LlmUnifiedResponse {
            provider: "OpenAI".into(),
            model: res.model,
            role,
            content,
            usage: Some(LlmUsage {
                input_tokens: Some(res.usage.prompt_tokens),
                output_tokens: Some(res.usage.completion_tokens),
                total_tokens: Some(res.usage.total_tokens),
            }),
            finish_reason,
        }
    }
}
