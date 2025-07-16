# OneLLM API Client

This is a Rust client for interacting with the OneLLM API.

## Usage

Add this to your `Cargo.toml`:

```toml
onellm = "1.0.0"
```

## Example

```rust
use onellm::input::Message;

mod input;
mod output;

#[tokio::main]
async fn main() {
    let output = input::APIInput::new(
        "https://api.deepseek.com/chat/completions".to_string(),
        input::Model::DeepSeekV3,
        vec![Message {
            role: "user".to_string(),
            content: "hi there!".to_string(),
        }],
        200,
    )
    .send(String::from("YOUR API KEY HERE"))
    .await
    .expect("Error obtaining result");
    println!("Output: {output:#?}");
}
```
