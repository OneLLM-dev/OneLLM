#![allow(unused)]
#![allow(non_camel_case_types)]
use crate::{requests::AIProvider, *};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Model {
    Gpt4_1,
    Gpt4_1Mini,
    Gpt4_1Nano,
    GptO3,
    GptO4Mini,
    ClaudeOpus4,
    ClaudeSonnet4,
    ClaudeHaiku3_5,
    ClaudeOpus3,
    ClaudeSonnet3_7,
    ClaudeHaiku3,
    DeepSeekR1,
    DeepSeekV3,
    Gemini25FlashPreview,
    Gemini25ProPreview,
    Gemini20Flash,
    Gemini20FlashLite,
    Gemini15Flash,
    Gemini15Flash8B,
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
