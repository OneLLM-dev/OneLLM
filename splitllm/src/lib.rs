use anyhow::Result;
use onellm::input::{APIInput, Message, Model, ResponseFormat};
use schemars::{JsonSchema, schema_for};

#[derive(JsonSchema)]
pub enum Style {
    Agile,
    Waterfall,
}

#[derive(JsonSchema)]
pub enum Tier {
    High,
    Medium,
    Low,
}

impl Tier {
    pub fn tier_from_model(model: Model) -> Tier {
        let price = model.price();
        match price {
            0..=499 => Tier::Low,
            500..=1999 => Tier::Medium,
            _ => Tier::High,
        }
    }
}

#[derive(JsonSchema)]
pub struct PromptSplit {
    pub style: Style,
    pub prompts: Vec<String>,
}

pub async fn split_prompt(input: &str) -> Result<()> {
    let endpoint = "https://api.openai.com/v1/responses".to_string();
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

    Ok(())
}
