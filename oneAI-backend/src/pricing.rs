#[allow(unused)]
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum OpenAIModel {
    Gpt4_1,
    Gpt4_1Mini,
    Gpt4_1Nano,
    GptO3,
    GptO4Mini,
}

#[derive(Deserialize, Serialize)]
pub enum ClaudeModel {
    Opus4,
    Sonnet4,
    Haiku3_5,
    Opus3,
    Sonnet3_7,
    Haiku3,
}

#[derive(Deserialize, Serialize)]
pub enum GeminiModel {
    Gem25_FlashPrev,
    Gem25ProPrev,
    Gem25FlashNativeAudio,
}

#[derive(Deserialize, Serialize)]
pub enum MistralModel {}

#[derive(Deserialize, Serialize)]
pub enum DeepSeekModel {
    Dsv3,
    DsR1,
}

pub trait Model {
    /// The prcing for the models per 1 million tokens
    async fn price(&self) -> f32;
}

impl Model for OpenAIModel {
    async fn price(&self) -> f32 {
        match self {
            OpenAIModel::Gpt4_1 => return 11.0,
            OpenAIModel::Gpt4_1Mini => return 2.5,
            OpenAIModel::Gpt4_1Nano => return 0.6,
            OpenAIModel::GptO3 => return 50.0,
            OpenAIModel::GptO4Mini => return 6.0,
        }
    }
}

impl Model for DeepSeekModel {
    /// Returns $3.0 for R1 model (reasoner)
    /// Returns $1.5 for V3 model (chat)
    async fn price(&self) -> f32 {
        match self {
            DeepSeekModel::DsR1 => return 3.0,
            DeepSeekModel::Dsv3 => return 1.5,
        }
    }
}

impl Model for ClaudeModel {
    async fn price(&self) -> f32 {
        match self {
            ClaudeModel::Opus4 => return 91.0,
            ClaudeModel::Sonnet4 => return 19.0,
            ClaudeModel::Haiku3_5 => return 5.0,
            ClaudeModel::Opus3 => return 91.0,
            ClaudeModel::Sonnet3_7 => return 18.5,
            ClaudeModel::Haiku3 => return 2.0,
        }
    }
}

impl Model for GeminiModel {
    async fn price(&self) -> f32 {
        match self {
            _ => {}
        }
        0.0
    }
}
