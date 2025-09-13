//! Stage1 Step1-2: 最初のリクエスト + tool_calls 有無判定のみ
//! ここでは calc_sum の実行や 2 回目 API 呼び出しは行わない。

use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::env;
use std::io;

// ===== レスポンス用 (tool_calls 判定) =====
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: IncomingMessage,
}

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    role: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: String,
    function: CalledFunction,
}

#[derive(Debug, Deserialize)]
struct CalledFunction {
    name: String,
    // OpenAI は arguments を文字列(JSON)で返す
    arguments: String,
}

// ===== リクエスト用 (送信する tools と messages) =====
#[derive(Debug, Serialize)]
struct OutgoingMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct ToolDefinition<'a> {
    #[serde(rename = "type")]
    r#type: &'a str, // "function"
    function: ToolFunctionDefinition<'a>,
}

#[derive(Debug, Serialize)]
struct ToolFunctionDefinition<'a> {
    name: &'a str,
    description: &'a str,
    parameters: JsonSchemaObject<'a>,
}

#[derive(Debug, Serialize)]
struct JsonSchemaObject<'a> {
    #[serde(rename = "type")]
    r#type: &'a str, // "object"
    properties: Value,
    required: Vec<&'a str>,
}

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    println!("足し算したい内容を自然文で入力してください (例: 3と8を足して)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_text = input.trim();

    // ツール定義 (型で組み立て → Serialize)
    let tool_schema = ToolDefinition {
        r#type: "function",
        function: ToolFunctionDefinition {
            name: "calc_sum",
            description: "2つの数 a と b の合計を計算して返す",
            parameters: JsonSchemaObject {
                r#type: "object",
                properties: json!({
                    "a": { "type": "number", "description": "最初の数" },
                    "b": { "type": "number", "description": "次の数" }
                }),
                required: vec!["a", "b"],
            },
        },
    };

    // 送信メッセージ (こちらは content 必須)
    let messages = vec![
        OutgoingMessage {
            role: "system",
            content: "あなたは計算を手伝うアシスタントです",
        },
        OutgoingMessage {
            role: "user",
            content: user_text,
        },
    ];

    // let tool_schema = json!({
    //     "type": "function",
    //     "function": {
    //         "name": "calc_sum",
    //         "description": "2つの数 a と b の合計を計算して返す",
    //         "parameters": {
    //             "type": "object",
    //             "properties": {
    //                 "a": {"type": "number", "description": "最初の数"},
    //                 "b": {"type": "number", "description": "次の数"}
    //             },
    //             "required": ["a", "b"]
    //         }
    //     }
    // });

    let client = reqwest::Client::new();
    let first = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": messages,
            "tools": [tool_schema]
        }))
        .send()
        .await?;

    let status = first.status();
    let body_text = first.text().await?;
    if !status.is_success() {
        eprintln!("API Error: {}\n{}", status, body_text);
        return Ok(());
    }

    let parsed: ChatResponse = match serde_json::from_str(&body_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON パース失敗: {}\nraw: {}", e, body_text);
            return Ok(());
        }
    };

    if let Some(first_choice) = parsed.choices.first() {
        let msg = &first_choice.message;
        if !msg.tool_calls.is_empty() {
            let tc = &msg.tool_calls[0];
            println!(
                "tool_calls 検出: {} (arguments raw JSON string)",
                tc.function.name
            );
            println!("← ここまでが Stage1 ステップ2。次は引数 JSON をパースして実行します。");
        } else {
            println!("通常回答: {}", msg.content.clone().unwrap_or_default());
        }
    } else {
        println!("choices が空です");
    }

    Ok(())
}
