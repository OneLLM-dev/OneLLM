#![allow(non_snake_case)]
use crate::requests::responseparser::common::LlmUnifiedResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiUsageMetadata {
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: u32,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
    #[serde(rename = "promptFeedback")]
    pub prompt_feedback: Option<GeminiPromptFeedback>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<GeminiUsageMetadata>,
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String>,
    #[serde(rename = "responseId")]
    pub response_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiContent,
    pub finishReason: String,
    pub index: Option<u32>, // optional now
    pub safetyRatings: Vec<GeminiSafetyRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgLogprobs: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPromptFeedback {
    pub safetyRatings: Vec<GeminiSafetyRating>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiSafetyRating {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<String>,
}
impl From<GeminiResponse> for LlmUnifiedResponse {
    fn from(res: GeminiResponse) -> Self {
        let candidate = res.candidates.get(0);
        let (role, content, finish_reason) = if let Some(c) = candidate {
            let text = c
                .content
                .parts
                .iter()
                .map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("\n");
            (
                Some(c.content.role.clone()),
                text,
                Some(c.finishReason.clone()),
            )
        } else {
            (None, String::new(), None)
        };

        LlmUnifiedResponse {
            provider: "Gemini".into(),
            model: "gemini".into(), // Gemini API doesn't return model in response, can inject manually
            role,
            content,
            usage: None, // Gemini's response usually doesn't include token usage
            finish_reason,
        }
    }
}
