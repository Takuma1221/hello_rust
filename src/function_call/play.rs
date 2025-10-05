use std::collections::HashMap;
use std::env;
use std::io;

use dotenv::dotenv;

use crate::function_call::api::send_chat_completion; // 以前の build_* 系は使わず自前で messages ベクタを構築
use crate::function_call::tools::NumFunc; // execute_first_tool_call は単発用なのでループ版では不使用
use crate::function_call::types::{
    ChatResponse, JsonSchemaObject, SumArgs, ToolDefinition, ToolFunctionDefinition,
};
use serde_json::{Value, json};

pub fn calc_sum(a: f64, b: f64) -> f64 {
    a + b
}
pub fn calc_times(a: f64, b: f64) -> f64 {
    a * b
}
pub fn calc_pow(a: f64, b: f64) -> f64 {
    // b は 0 以上の整数として扱う (小数部があれば切り捨て)
    if b < 0.0 {
        return f64::NAN;
    }
    let mut exp = b.floor() as u32;
    let mut result = 1.0;
    let mut base = a;
    // 簡単で十分: 2 分累乗 (高速 & 安定)
    while exp > 0 {
        if exp & 1 == 1 {
            result *= base;
        }
        base *= base;
        exp >>= 1;
    }
    result
}

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

    // calc_pow 用ツール定義 (べき乗: a^b, b は非負整数想定)
    let tool_schema_pow = ToolDefinition {
        r#type: "function",
        function: ToolFunctionDefinition {
            name: "calc_pow",
            description: "a の b 乗 (b は 0 以上の整数として解釈) を計算して返す",
            parameters: JsonSchemaObject {
                r#type: "object",
                properties: json!({
                    "a": { "type": "number", "description": "底 (base)" },
                    "b": { "type": "number", "description": "指数 (0 以上 の整数と見なす)" }
                }),
                required: vec!["a", "b"],
            },
        },
    };

    let mut registry: HashMap<String, NumFunc> = HashMap::new();
    registry.insert("calc_sum".into(), calc_sum as NumFunc);
    registry.insert("calc_times".into(), calc_times as NumFunc);
    registry.insert("calc_pow".into(), calc_pow as NumFunc);

    // --- ここから複数回 tool 呼び出し対応ループ版 ---
    // OpenAI へ渡す完全な履歴 (tool 呼び出しを含む) を Value で保持
    let mut messages: Vec<Value> = vec![
        json!({"role": "system", "content": "あなたは計算を手伝う厳密モードのアシスタントです。ルール: (1) 不要な 0 との加算や 1 との乗算は行わない。 (2) 必要なら足し算は calc_sum、掛け算は calc_times、べき乗は calc_pow を使う。 (3) べき乗が直接表現できる場合は calc_pow で 1 回で求める。 (4) 途中の説明は簡潔に。"}),
        json!({"role": "user", "content": user_text}),
    ];

    let tools = vec![tool_schema_sum, tool_schema_times, tool_schema_pow];
    let client = reqwest::Client::new();
    let model = "gpt-4o-mini";
    const MAX_ITER: usize = 5; // 無限ループ安全弁

    for iter in 0..MAX_ITER {
        let req_body = json!({
            "model": model,
            "messages": messages,
            "tools": tools,
            "temperature": 0.0,
        });

        let raw = match send_chat_completion(&client, &api_key, &req_body).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("API 呼び出し失敗(iter={iter}): {e}");
                break;
            }
        };

        let parsed: ChatResponse = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("JSON パース失敗(iter={iter}): {e}\nraw: {raw}");
                break;
            }
        };

        let choice = match parsed.choices.first() {
            Some(c) => c,
            None => {
                eprintln!("choices 空(iter={iter})");
                break;
            }
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
                    eprintln!(
                        "引数パース失敗 tool_call_id={} error={e} raw={}",
                        tc.id, tc.function.arguments
                    );
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
