use crate::requests::responseparser::common::{LlmUnifiedResponse, LlmUsage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<MistralChoice>,
    pub usage: Option<MistralUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralChoice {
    pub index: u32,
    pub message: MistralMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

impl From<MistralResponse> for LlmUnifiedResponse {
    fn from(res: MistralResponse) -> Self {
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
            provider: "Mistral".into(),
            model: res.model,
            role,
            content,
            usage: res.usage.map(|u| LlmUsage {
                input_tokens: Some(u.prompt_tokens),
                output_tokens: Some(u.completion_tokens),
                total_tokens: Some(u.total_tokens),
            }),
            finish_reason,
        }
    }
}
