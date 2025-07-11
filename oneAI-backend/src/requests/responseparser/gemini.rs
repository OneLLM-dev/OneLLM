#![allow(non_snake_case)]
use crate::requests::responseparser::common::LlmUnifiedResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
    pub promptFeedback: GeminiPromptFeedback,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: Option<GeminiUsageMetadata>,
}

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
pub struct GeminiCandidate {
    pub content: GeminiContent,
    pub finishReason: String,
    pub index: u32,
    pub safetyRatings: Vec<GeminiSafetyRating>,
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

        let usage = res.usage_metadata.map(|u| crate::requests::responseparser::common::LlmUsage {
            input_tokens: Some(u.prompt_token_count),
            output_tokens: Some(u.candidates_token_count),
            total_tokens: Some(u.total_token_count),
        });

        LlmUnifiedResponse {
            provider: "Gemini".into(),
            model: "gemini".into(), // Gemini API doesn't return model in response, can inject manually
            role,
            content,
            usage, // Gemini's response usually doesn't include token usage
            finish_reason,
        }
    }
}
