#![allow(non_camel_case_types)]
use crate::requests::requests::AIProvider;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "GPT-4.1")]
    Gpt4_1,
    #[serde(rename = "GPT-4.1-Mini")]
    Gpt4_1Mini,
    #[serde(rename = "GPT-4.1-Nano")]
    Gpt4_1Nano,
    #[serde(rename = "GPT-o3")]
    GptO3,
    #[serde(rename = "GPT-o4-mini")]
    GptO4Mini,
    #[serde(rename = "Opus-4")]
    ClaudeOpus4,
    #[serde(rename = "Sonnet-4")]
    ClaudeSonnet4,
    #[serde(rename = "Haiku-3.5")]
    ClaudeHaiku3_5,
    #[serde(rename = "Opus-3")]
    ClaudeOpus3,
    #[serde(rename = "Sonnet-3.7")]
    ClaudeSonnet3_7,
    #[serde(rename = "Haiku-3")]
    ClaudeHaiku3,
    #[serde(rename = "DeepSeek-Reasoner")]
    DeepSeekR1,
    #[serde(rename = "DeepSeek-Chat")]
    DeepSeekV3,
    Gemini25FlashPreview,
    #[serde(rename = "2.5-Pro-preview")]
    Gemini25ProPreview,
    #[serde(rename = "2.0-Flash")]
    Gemini20Flash,
    #[serde(rename = "2.0-Flash-lite")]
    Gemini20FlashLite,
    #[serde(rename = "1.5-Flash")]
    Gemini15Flash,
    #[serde(rename = "1.5-Flash-8B")]
    Gemini15Flash8B,
    #[serde(rename = "1.5-Pro")]
    Gemini15Pro,
}

impl Model {
    pub async fn price(&self) -> f32 {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1 => 10.4,
            Model::Gpt4_1Mini => 2.08,
            Model::Gpt4_1Nano => 0.52,
            Model::GptO3 => 10.4,
            Model::GptO4Mini => 5.72,

            // ==== Anthropic ====
            Model::ClaudeOpus4 => 93.6,
            Model::ClaudeSonnet4 => 18.72,
            Model::ClaudeHaiku3_5 => 4.992,
            Model::ClaudeOpus3 => 93.6,
            Model::ClaudeSonnet3_7 => 18.72,
            Model::ClaudeHaiku3 => 1.82,

            // ==== DeepSeek ====
            Model::DeepSeekR1 => 1.4248,
            Model::DeepSeekV3 => 2.4232,

            // ==== Gemini (Google) ====
            Model::Gemini25FlashPreview => 3.796,
            Model::Gemini25ProPreview => 18.2,
            Model::Gemini20Flash => 0.52,
            Model::Gemini20FlashLite => 0.39,
            Model::Gemini15Flash => 0.78,
            Model::Gemini15Flash8B => 0.39,
            Model::Gemini15Pro => 13.0,
        }
    }

    pub async fn provider(&self) -> AIProvider {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1
            | Model::Gpt4_1Mini
            | Model::Gpt4_1Nano
            | Model::GptO3
            | Model::GptO4Mini => AIProvider::OpenAI,

            // ==== Anthropic ====
            Model::ClaudeOpus4
            | Model::ClaudeSonnet4
            | Model::ClaudeHaiku3_5
            | Model::ClaudeOpus3
            | Model::ClaudeSonnet3_7
            | Model::ClaudeHaiku3 => AIProvider::Anthropic,

            // ==== Gemini (Google) ====
            Model::Gemini25FlashPreview
            | Model::Gemini25ProPreview
            | Model::Gemini20Flash
            | Model::Gemini20FlashLite
            | Model::Gemini15Flash
            | Model::Gemini15Flash8B
            | Model::Gemini15Pro => AIProvider::Gemini,

            // ==== DeepSeek ====
            Model::DeepSeekR1 | Model::DeepSeekV3 => AIProvider::DeepSeek,
        }
    }
}
