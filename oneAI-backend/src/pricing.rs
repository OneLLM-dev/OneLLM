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

    // ==== Mistral ====
    #[serde(rename = "Mistral-Medium-3")]
    MistralMedium3,
    #[serde(rename = "Magistral-Medium")]
    MagistralMedium,
    #[serde(rename = "Codestral")]
    Codestral,
    #[serde(rename = "Devstral-Medium")]
    DevstralMedium,
    #[serde(rename = "Mistral-Large")]
    MistralLarge,
    #[serde(rename = "Pixtral-Large")]
    PixtralLarge,
    #[serde(rename = "Ministral-8B-24.10")]
    Ministral8B_24_10,
    #[serde(rename = "Ministral-3B-24.10")]
    Ministral3B_24_10,
    #[serde(rename = "Mistral-Small-3.2")]
    MistralSmall3_2,
    #[serde(rename = "Magistral-Small")]
    MagistralSmall,
    #[serde(rename = "Devstral-Small")]
    DevstralSmall,
    #[serde(rename = "Pixtral-12B")]
    Pixtral12B,
    #[serde(rename = "Mistral-NeMo")]
    MistralNemo,
}

impl Model {
    pub fn name(&self) -> &str {
        match self {
            // ==== OpenAI ====
            Model::Gpt4_1 => "gpt-4.1",
            Model::Gpt4_1Mini => "gpt-4.1-mini",
            Model::Gpt4_1Nano => "4.1-nano",
            Model::GptO3 => "o3",
            Model::GptO4Mini => "o4-mini",
            Model::GptO3Pro => "o3-pro",
            Model::Gpt4o => "gpt-4o",
            Model::Gpt4oMini => "gpt-4o-mini",
            Model::GptO1 => "o1",
            Model::GptO3DeepResearch => "o3-DeepResearch",
            Model::GptO3Mini => "o3-mini",
            Model::GptO1Mini => "o1-mini",

            // ==== Anthropic ====
            Model::ClaudeOpus4 => "claude-opus-4-20250514",
            Model::ClaudeSonnet4 => "claude-sonnet-4-20250514",
            Model::ClaudeHaiku3_5 => "claude-3-5-haiku-latest",
            Model::ClaudeOpus3 => "claude-3-opus-20240229",
            Model::ClaudeSonnet3_7 => "claude-3-7-sonnet-latest",
            Model::ClaudeHaiku3 => "claude-3-haiku-20240307",

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

            // ==== Mistral ====
            Model::MistralMedium3 => "Mistral-Medium-2505",
            Model::MagistralMedium => "Magistral-Medium-2506",
            Model::Codestral => "Codestral-2501",
            Model::DevstralMedium => "Devstral-Medium-2507",
            Model::MistralLarge => "Mistral-Large-2411",
            Model::PixtralLarge => "Pixtral-Large-2411",
            Model::Ministral8B_24_10 => "Ministral-8B-2410",
            Model::Ministral3B_24_10 => "Ministral-3B-2410",
            Model::MistralSmall3_2 => "Mistral-Small-2506",
            Model::MagistralSmall => "Magistral-Small-2506",
            Model::DevstralSmall => "Devstral-Small-2507",
            Model::Pixtral12B => "Pixtral-12B-2409",
            Model::MistralNemo => "open-Mistral-NeMo",
        }
    }
pub fn input_price(&self) -> u32 {
    match self {
        // ==== OpenAI ====
        Model::Gpt4_1 => 208,
        Model::Gpt4_1Mini => 42,
        Model::Gpt4_1Nano => 10,
        Model::Gpt4o => 260,
        Model::Gpt4oMini => 16,
        Model::GptO3 => 208,
        Model::GptO3Pro => 2080,
        Model::GptO3DeepResearch => 1040,
        Model::GptO4Mini => 114,
        Model::GptO3Mini => 114,
        Model::GptO1 => 1560,
        Model::GptO1Mini => 114,

        // ==== Anthropic ====
        Model::ClaudeOpus4 => 1560,
        Model::ClaudeSonnet4 => 312,
        Model::ClaudeHaiku3_5 => 83,
        Model::ClaudeOpus3 => 1560,
        Model::ClaudeSonnet3_7 => 312,
        Model::ClaudeHaiku3 => 26,

        // ==== Mistral ====
        Model::MistralMedium3 => 42,
        Model::MagistralMedium => 208,
        Model::MistralLarge => 208,
        Model::DevstralMedium => 42,
        Model::MistralSmall3_2 => 10,
        Model::MagistralSmall => 52,
        Model::Codestral => 31,
        Model::DevstralSmall => 10,
        Model::MistralNemo => 16,
        Model::Pixtral12B => 16,
        Model::PixtralLarge => 208,
        Model::Ministral8B_24_10 => 10,
        Self::Ministral3B_24_10 => 4,

        // ==== DeepSeek ====
        Model::DeepSeekR1 => 57,
        Model::DeepSeekV3 => 28,

        // ==== Gemini (Google) ====
        Model::Gemini25ProPreview => 130,
        Model::Gemini25FlashPreview => 31,
        Model::Gemini20Flash => 10,
        Model::Gemini20FlashLite => 7,
        Model::Gemini15Flash => 7,
        Model::Gemini15Flash8B => 3,
        Model::Gemini15Pro => 130,
    }
}

pub fn output_price(&self) -> u32 {
    match self {
        // ==== OpenAI ====
        Model::Gpt4_1 => 832,
        Model::Gpt4_1Mini => 166,
        Model::Gpt4_1Nano => 42,
        Model::Gpt4o => 1040,
        Model::Gpt4oMini => 62,
        Model::GptO3 => 832,
        Model::GptO3Pro => 8320,
        Model::GptO3DeepResearch => 4160,
        Model::GptO4Mini => 458,
        Model::GptO3Mini => 458,
        Model::GptO1 => 6240,
        Model::GptO1Mini => 458,

        // ==== Anthropic ====
        Model::ClaudeOpus4 => 7800,
        Model::ClaudeSonnet4 => 1560,
        Model::ClaudeHaiku3_5 => 416,
        Model::ClaudeOpus3 => 7800,
        Model::ClaudeSonnet3_7 => 1560,
        Model::ClaudeHaiku3 => 130,

        // ==== Gemini (Google) ====
        Model::Gemini25ProPreview => 1040,
        Model::Gemini25FlashPreview => 260,
        Model::Gemini20Flash => 42,
        Model::Gemini20FlashLite => 31,
        Model::Gemini15Flash => 31,
        Model::Gemini15Flash8B => 16,
        Model::Gemini15Pro => 520,

        // ==== Mistral ====
        Model::MistralMedium3 => 208,
        Model::MagistralMedium => 520,
        Model::MistralLarge => 624,
        Model::DevstralMedium => 208,
        Model::MistralSmall3_2 => 31,
        Model::MagistralSmall => 156,
        Model::Codestral => 94,
        Model::DevstralSmall => 31,
        Model::MistralNemo => 16,
        Model::PixtralLarge => 624,
        Model::Pixtral12B => 16,
        Model::Ministral8B_24_10 => 104,
        Model::Ministral3B_24_10 => 4,

        Model::DeepSeekR1 => 228,
        Model::DeepSeekV3 => 114,
    }
}

    //    pub fn price(&self) -> u32 {
    //        match self {
    //            // ==== OpenAI ====
    //            Model::Gpt4_1 => 1040,
    //            Model::Gpt4_1Mini => 208,
    //            Model::Gpt4_1Nano => 52,
    //            Model::GptO3 => 1040,
    //            Model::GptO4Mini => 572,
    //            Model::GptO3Pro => 10400,
    //            Model::Gpt4o => 1300,
    //            Model::Gpt4oMini => 78,
    //            Model::GptO1 => 7800,
    //            Model::GptO3DeepResearch => 5200,
    //            Model::GptO3Mini => 572,
    //            Model::GptO1Mini => 572,
    //
    //            // ==== Anthropic ====
    //            Model::ClaudeOpus4 => 9360,
    //            Model::ClaudeSonnet4 => 1872,
    //            Model::ClaudeHaiku3_5 => 499,
    //            Model::ClaudeOpus3 => 9360,
    //            Model::ClaudeSonnet3_7 => 1872,
    //            Model::ClaudeHaiku3 => 182,
    //
    //            // ==== DeepSeek ====
    //            Model::DeepSeekR1 => 142,
    //            Model::DeepSeekV3 => 242,
    //
    //            // ==== Gemini (Google) ====
    //            Model::Gemini25FlashPreview => 380,
    //            Model::Gemini25ProPreview => 1820,
    //            Model::Gemini20Flash => 52,
    //            Model::Gemini20FlashLite => 39,
    //            Model::Gemini15Flash => 78,
    //            Model::Gemini15Flash8B => 39,
    //            Model::Gemini15Pro => 1300,
    //
    //            // ==== Mistral ====
    //            Model::MistralMedium3 => 2496,
    //            Model::MagistralMedium => 7280,
    //            Model::Codestral => 1248,
    //            Model::DevstralMedium => 2496,
    //            Model::MistralLarge => 8320,
    //            Model::PixtralLarge => 8320,
    //            Model::Ministral8B_24_10 => 208,
    //            Model::Ministral3B_24_10 => 83,
    //            Model::MistralSmall3_2 => 416,
    //            Model::MagistralSmall => 2080,
    //            Model::DevstralSmall => 416,
    //            Model::Pixtral12B => 312,
    //            Model::MistralNemo => 312,
    //        }
    //    }
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

            // ==== Gemini ====
            Model::Gemini25FlashPreview
            | Model::Gemini25ProPreview
            | Model::Gemini20Flash
            | Model::Gemini20FlashLite
            | Model::Gemini15Flash
            | Model::Gemini15Flash8B
            | Model::Gemini15Pro => AIProvider::Gemini,

            // ==== DeepSeek ====
            Model::DeepSeekR1 | Model::DeepSeekV3 => AIProvider::DeepSeek,

            // ==== Mistral ====
            Model::MistralMedium3
            | Model::MagistralMedium
            | Model::Codestral
            | Model::DevstralMedium
            | Model::MistralLarge
            | Model::PixtralLarge
            | Model::Ministral8B_24_10
            | Model::Ministral3B_24_10
            | Model::MagistralSmall
            | Model::DevstralSmall
            | Model::Pixtral12B
            | Model::MistralNemo
            | Model::MistralSmall3_2 => AIProvider::Mistral,
        }
    }
}
