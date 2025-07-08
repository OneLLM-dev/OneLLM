use crate::requests::responseparser::common::{LlmUnifiedResponse, LlmUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepSeekResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<DeepSeekChoice>,
    pub usage: DeepSeekUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepSeekChoice {
    pub index: u32,
    pub message: DeepSeekMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepSeekMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeepSeekUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl From<DeepSeekResponse> for LlmUnifiedResponse {
    fn from(res: DeepSeekResponse) -> Self {
        let choice = res.choices.get(0);
        let (role, content, finish_reason) = if let Some(c) = choice {
            (
                Some(c.message.role.clone()),
                c.message.content.clone(),
                c.finish_reason.clone(),
            )
        } else {
            (None, String::new(), None)
        };

        LlmUnifiedResponse {
            provider: "DeepSeek".into(),
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
