//! Stage1 Step1-2: 最初のリクエストと tool_calls が付いたかどうかを見るところまで。
//! ここではまだ calc_sum を実際に動かしたり 2 回目のリクエスト送信はしません。

use dotenv::dotenv;                 // .env から OPENAI_API_KEY を読む
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE}; // 認証用の定数
use serde::{Deserialize, Serialize}; // 型 <-> JSON 変換 (derive)
use serde_json::{Value, json};       // json! マクロや動的値
use std::env;                        // 環境変数参照
use std::io;                         // 標準入力読み込み

// ===== モデルから「受け取る」形 =====
// リクエストで送った形とは微妙に構造が違うため分離しています。
#[derive(Debug, Deserialize)]
struct ChatResponse { choices: Vec<Choice> }

#[derive(Debug, Deserialize)]
struct Choice { message: IncomingMessage }

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    role: String,
    #[serde(default)]
    content: Option<String>, // 関数呼び出しが付く場合は空になることがある
    #[serde(default)]
    tool_calls: Vec<ToolCall>, // 無ければ空の配列
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
    name: String,      // 呼び出してほしい関数名
    arguments: String, // JSON 文字列 (Step3 で構造体に読み込む予定)
}

// ===== モデルへ「送る」形 =====
#[derive(Debug, Serialize)]
struct OutgoingMessage<'a> { role: &'a str, content: &'a str }

#[derive(Debug, Serialize)]
struct ToolDefinition<'a> { #[serde(rename = "type")] r#type: &'a str, function: ToolFunctionDefinition<'a> }

#[derive(Debug, Serialize)]
struct ToolFunctionDefinition<'a> { name: &'a str, description: &'a str, parameters: JsonSchemaObject<'a> }

#[derive(Debug, Serialize)]
struct JsonSchemaObject<'a> { #[serde(rename = "type")] r#type: &'a str, properties: Value, required: Vec<&'a str> }

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();                      // .env が無くてもエラーにはしない
    let api_key = env::var("OPENAI_API_KEY")?; // 未設定ならここで終了

    println!("足し算したい内容を自然文で入力してください (例: 3と8を足して)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_text = input.trim();

    // ===== ツール定義 =====
    // json! を都度ベタ書きするより、型でまとめると再利用や拡張がしやすい
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

    // ===== 送信メッセージ (system + user) =====
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

    let client = reqwest::Client::new();
    // ===== 1 回目の呼び出し =====
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

    let status = first.status(); // body 消費前に保持
    let body_text = first.text().await?;
    if !status.is_success() {
        eprintln!("API Error: {}\n{}", status, body_text);
        return Ok(());
    }

    // JSON 文字列 -> 型 (tool_calls が付いたか見るため)
    let parsed: ChatResponse = match serde_json::from_str(&body_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON パース失敗: {}\nraw: {}", e, body_text);
            return Ok(());
        }
    };

    // ===== 最初の候補を確認 (1件想定) =====
    if let Some(first_choice) = parsed.choices.first() {
        let msg = &first_choice.message;
        if !msg.tool_calls.is_empty() {
            let tc = &msg.tool_calls[0];
            println!(
                "tool_calls 検出: {} (arguments raw JSON string)",
                tc.function.name
            );
            println!("← Step2 完了。次は arguments (JSON 文字列) を構造体へ読み込み計算します。");
        } else {
            println!("通常回答: {}", msg.content.clone().unwrap_or_default());
        }
    } else {
        println!("choices が空です");
    }

    Ok(())
}
