use crate::requests::responseparser::common::{LlmUnifiedResponse, LlmUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub status: String,
    pub error: Option<serde_json::Value>,
    pub incomplete_details: Option<serde_json::Value>,
    pub instructions: Option<serde_json::Value>,
    pub max_output_tokens: Option<serde_json::Value>,
    pub model: String,
    pub output: Vec<OpenAIOutput>,
    pub parallel_tool_calls: bool,
    pub previous_response_id: Option<String>,
    pub reasoning: OpenAIReasoning,
    pub store: bool,
    pub temperature: f64,
    pub text: OpenAITextField,
    pub tool_choice: String,
    pub tools: Vec<serde_json::Value>,
    pub top_p: f64,
    pub truncation: String,
    pub usage: OpenAIUsage,
    pub user: Option<serde_json::Value>,
    pub metadata: OpenAIMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIOutput {
    pub r#type: String,
    pub id: String,
    pub status: String,
    pub role: String,
    pub content: Vec<OpenAIContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIContent {
    pub r#type: String,
    pub text: String,
    pub annotations: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIReasoning {
    pub effort: Option<serde_json::Value>,
    pub summary: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAITextField {
    pub format: OpenAIFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIFormat {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIUsage {
    pub input_tokens: u32,
    pub input_tokens_details: OpenAIInputTokensDetails,
    pub output_tokens: u32,
    pub output_tokens_details: OpenAIOutputTokensDetails,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIInputTokensDetails {
    pub cached_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIOutputTokensDetails {
    pub reasoning_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMetadata {} // Empty object

impl From<OpenAIResponse> for LlmUnifiedResponse {
    fn from(res: OpenAIResponse) -> Self {
        let content = res
            .output
            .iter()
            .flat_map(|out| out.content.iter().map(|c| c.text.clone()))
            .collect::<Vec<_>>()
            .join("\n");

        LlmUnifiedResponse {
            provider: "OpenAI".into(),
            model: res.model,
            role: res.output.get(0).map(|o| o.role.clone()),
            content,
            usage: Some(LlmUsage {
                input_tokens: Some(res.usage.input_tokens),
                output_tokens: Some(res.usage.output_tokens),
                total_tokens: Some(res.usage.total_tokens),
            }),
            finish_reason: Some(res.status), // e.g. "completed"
        }
    }
}
