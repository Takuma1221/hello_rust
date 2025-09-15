use std::collections::HashMap;
use std::env;
use std::io;

use dotenv::dotenv;

use crate::function_call::api::send_chat_completion; // 以前の build_* 系は使わず自前で messages ベクタを構築
use crate::function_call::tools::NumFunc; // execute_first_tool_call は単発用なのでループ版では不使用
use crate::function_call::types::{ToolDefinition, ToolFunctionDefinition, JsonSchemaObject, ChatResponse, SumArgs};
use serde_json::{json, Value};

pub fn calc_sum(a: f64, b: f64) -> f64 { a + b }
pub fn calc_times(a: f64, b: f64) -> f64 { a * b }

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    println!("足し算したい内容を自然文で入力してください (例: 3と8を足して)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_text = input.trim();

    // calc_sum 用ツール定義
    let tool_schema_sum = ToolDefinition {
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
    // calc_times 用ツール定義 (積)
    let tool_schema_times = ToolDefinition {
        r#type: "function",
        function: ToolFunctionDefinition {
            name: "calc_times",
            description: "2つの数 a と b の積 (掛け算) を計算して返す",
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

    let mut registry: HashMap<String, NumFunc> = HashMap::new();
    registry.insert("calc_sum".into(), calc_sum as NumFunc);
    registry.insert("calc_times".into(), calc_times as NumFunc);

    // --- ここから複数回 tool 呼び出し対応ループ版 ---
    // OpenAI へ渡す完全な履歴 (tool 呼び出しを含む) を Value で保持
    let mut messages: Vec<Value> = vec![
        json!({"role": "system", "content": "あなたは計算を手伝うアシスタントです。足し算は calc_sum、掛け算は calc_times を使い、必要なら順に複数ツールを呼び出して最終的な式と答えを示してください。"}),
        json!({"role": "user", "content": user_text}),
    ];

    let tools = vec![tool_schema_sum, tool_schema_times];
    let client = reqwest::Client::new();
    let model = "gpt-4o-mini";
    const MAX_ITER: usize = 5; // 無限ループ安全弁

    for iter in 0..MAX_ITER {
        let req_body = json!({
            "model": model,
            "messages": messages,
            "tools": tools,
        });

        let raw = match send_chat_completion(&client, &api_key, &req_body).await {
            Ok(t) => t,
            Err(e) => { eprintln!("API 呼び出し失敗(iter={iter}): {e}"); break; }
        };

        let parsed: ChatResponse = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => { eprintln!("JSON パース失敗(iter={iter}): {e}\nraw: {raw}"); break; }
        };

        let choice = match parsed.choices.first() {
            Some(c) => c,
            None => { eprintln!("choices 空(iter={iter})"); break; }
        };
        let msg = &choice.message;

        if msg.tool_calls.is_empty() {
            // これが最終回答
            println!("最終回答: {}", msg.content.clone().unwrap_or_default());
            break;
        }

        // assistant メッセージ (tool_calls 含む) を履歴に追加
        let assistant_msg = {
            let mut tool_calls_json = Vec::new();
            for tc in &msg.tool_calls {
                tool_calls_json.push(json!({
                    "id": tc.id,
                    "type": "function",
                    "function": {"name": tc.function.name, "arguments": tc.function.arguments}
                }));
            }
            json!({
                "role": "assistant",
                // content があればそのまま残す (思考説明など)
                "content": msg.content.clone().unwrap_or_default(),
                "tool_calls": tool_calls_json,
            })
        };
        messages.push(assistant_msg);

        // 各 tool_call をローカル実行し tool メッセージを追加
        for tc in &msg.tool_calls {
            let func_name = &tc.function.name;
            let Some(func) = registry.get(func_name) else {
                eprintln!("未知のツール呼び出し: {func_name}");
                continue; // 未知ならスキップ (本来は break でも良い)
            };
            // 今は a,b の2引数数値関数のみ対応
            match serde_json::from_str::<SumArgs>(&tc.function.arguments) {
                Ok(args) => {
                    let result = func(args.a, args.b);
                    println!("実行: {func_name}({} , {}) = {result}", args.a, args.b);
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": tc.id,
                        "content": result.to_string(),
                    }));
                }
                Err(e) => {
                    eprintln!("引数パース失敗 tool_call_id={} error={e} raw={}", tc.id, tc.function.arguments);
                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": tc.id,
                        "content": format!("引数エラー: {e}"),
                    }));
                }
            }
        }

        // ループ継続→ 次 API 呼び出しでこれらの tool 結果を踏まえて次の指示または最終回答を得る
        if iter == MAX_ITER - 1 {
            eprintln!("最大反復 {MAX_ITER} に到達。打ち切り");
        }
    }

    Ok(())
}
