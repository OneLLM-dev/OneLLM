// This was never added... and will always be code, that was never finished and added
//
//
//
//
use anyhow::Result;
use onellm::input::{APIInput, Message, Model, ResponseFormat};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(JsonSchema, Deserialize, Serialize)]
pub enum Style {
    Agile,
    Waterfall,
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct PromptSlice {
    pub model: Model,
    pub prompt: String,
}

#[derive(JsonSchema, Deserialize, Serialize)]
pub struct PromptSplit {
    pub style: Style,
    pub prompts: Vec<PromptSlice>,
}

pub async fn split_prompt(input: &str) -> Result<PromptSplit> {
    let endpoint = "https://api.openai.com/v1/responses".to_string();
    let apikey = std::env::var("ONELLM")?;
    let system_message = Message {
        role: "system".to_string(),
        content: include_str!("prompt.txt").to_string(),
    };

    let model = Model::Gpt4o;
    let prompt = Message {
        role: "user".to_string(),
        content: input.to_string(),
    };
    let messages = vec![prompt, system_message];
    let mut query = APIInput::new(endpoint, model, messages, 1000);

    let schema = schema_for!(PromptSplit);

    query.response_format = Some(ResponseFormat {
        r#type: serde_json::to_string_pretty(&schema).unwrap(),
    });

    let response_str = serde_json::to_string_pretty(&query.send(apikey).await?)?;

    Ok(serde_json::from_str(&response_str)?)
}
