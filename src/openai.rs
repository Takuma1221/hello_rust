use dotenv::dotenv;
// use openai::openai_function_call;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::io;

#[derive(Debug, Deserialize)]
struct AiResponse {
    _id: String,
    _object: String,
    _model: String,
    choices: Vec<Choice>,
    _usage: Usage,
    _service_tier: String,
    _system_fingerprint: String,
}

#[derive(Debug, Deserialize)]
struct Choice {
    _index: u32,
    message: Message,
    _finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    _role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    _prompt_tokens: u32,
    _completion_tokens: u32,
    _total_tokens: u32,
}

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // .env を読み込む
    let api_key = env::var("OPENAI_API_KEY")?;

    println!("伝えたいことを入力してください");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "system", "content": "あなたは親切なアシスタントです"},
                {"role": "user", "content": input}
            ]
        }))
        .send()
        .await?;

    let text = response.text().await?;
    let json_value: AiResponse = serde_json::from_str(&text)?;
    // contentを安全に取り出す
    if let Some(first_choice) = json_value.choices.first() {
        println!("content: {}", first_choice.message.content);
    } else {
        println!("choices が空です");
    }
    // println!("レスポンス: {}", message);
    // println!("{}", text);

    Ok(())
}
