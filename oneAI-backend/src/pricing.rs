#![allow(non_camel_case_types)]
use crate::requests::requests::AIProvider;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Model {
    // ==== OpenAI ====
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
    #[serde(rename = "GPT-o3-pro")]
    GptO3Pro,
    #[serde(rename = "GPT-4o")]
    Gpt4o,
    #[serde(rename = "GPT-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "GPT-o1")]
    GptO1,
    #[serde(rename = "GPT-o3-DeepResearch")]
    GptO3DeepResearch,
    #[serde(rename = "GPT-o3-Mini")]
    GptO3Mini,
    #[serde(rename = "GPT-o1-Mini")]
    GptO1Mini,

    // ==== Anthropic ====
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

    // ==== DeepSeek ====
    #[serde(rename = "DeepSeek-Reasoner")]
    DeepSeekR1,
    #[serde(rename = "DeepSeek-Chat")]
    DeepSeekV3,

    // ==== Gemini (Google) ====
    #[serde(rename = "2.5-Flash-preview")]
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
    pub fn name(&self) -> &str {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1 => "GPT-4.1",
            Model::Gpt4_1Mini => "GPT-4.1-Mini",
            Model::Gpt4_1Nano => "GPT-4.1-Nano",
            Model::GptO3 => "GPT-o3",
            Model::GptO4Mini => "GPT-o4-mini",
            Model::GptO3Pro => "GPT-o3-pro",
            Model::Gpt4o => "GPT-4o",
            Model::Gpt4oMini => "GPT-4o-mini",
            Model::GptO1 => "GPT-o1",
            Model::GptO3DeepResearch => "GPT-o3-DeepResearch",
            Model::GptO3Mini => "GPT-o3-Mini",
            Model::GptO1Mini => "GPT-o1-Mini",

            // ==== Anthropic ====
            Model::ClaudeOpus4 => "Opus-4",
            Model::ClaudeSonnet4 => "Sonnet-4",
            Model::ClaudeHaiku3_5 => "Haiku-3.5",
            Model::ClaudeOpus3 => "Opus-3",
            Model::ClaudeSonnet3_7 => "Sonnet-3.7",
            Model::ClaudeHaiku3 => "Haiku-3",

            // ==== DeepSeek ====
            Model::DeepSeekR1 => "DeepSeek-Reasoner",
            Model::DeepSeekV3 => "DeepSeek-Chat",

            // ==== Gemini (Google) ====
            Model::Gemini25FlashPreview => "2.5-Flash-preview",
            Model::Gemini25ProPreview => "2.5-Pro-preview",
            Model::Gemini20Flash => "2.0-Flash",
            Model::Gemini20FlashLite => "2.0-Flash-lite",
            Model::Gemini15Flash => "1.5-Flash",
            Model::Gemini15Flash8B => "1.5-Flash-8B",
            Model::Gemini15Pro => "1.5-Pro",
        }
    }

    pub fn price(&self) -> u32 {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1 => 1040,
            Model::Gpt4_1Mini => 208,
            Model::Gpt4_1Nano => 52,
            Model::GptO3 => 1040,
            Model::GptO4Mini => 572,
            Model::GptO3Pro => 10400,
            Model::Gpt4o => 1300,
            Model::Gpt4oMini => 78,
            Model::GptO1 => 7800,
            Model::GptO3DeepResearch => 5200,
            Model::GptO3Mini => 572,
            Model::GptO1Mini => 572,

            // ==== Anthropic ====
            Model::ClaudeOpus4 => 9360,
            Model::ClaudeSonnet4 => 1872,
            Model::ClaudeHaiku3_5 => 499,
            Model::ClaudeOpus3 => 9360,
            Model::ClaudeSonnet3_7 => 1872,
            Model::ClaudeHaiku3 => 182,

            // ==== DeepSeek ====
            Model::DeepSeekR1 => 142,
            Model::DeepSeekV3 => 242,

            // ==== Gemini (Google) ====
            Model::Gemini25FlashPreview => 380,
            Model::Gemini25ProPreview => 1820,
            Model::Gemini20Flash => 52,
            Model::Gemini20FlashLite => 39,
            Model::Gemini15Flash => 78,
            Model::Gemini15Flash8B => 39,
            Model::Gemini15Pro => 1300,
        }
    }

    pub fn provider(&self) -> AIProvider {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1
            | Model::Gpt4_1Mini
            | Model::Gpt4_1Nano
            | Model::GptO3
            | Model::GptO4Mini
            | Model::GptO3Pro
            | Model::Gpt4o
            | Model::Gpt4oMini
            | Model::GptO1
            | Model::GptO3DeepResearch
            | Model::GptO3Mini
            | Model::GptO1Mini => AIProvider::OpenAI,

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
