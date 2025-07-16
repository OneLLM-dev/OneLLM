use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub code: u16,
    pub output: LlmUnifiedResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmUnifiedResponse {
    pub role: Option<String>,
    #[serde(alias = "output")]
    pub content: String,
    pub usage: Option<LlmUsage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmUsage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}
