use std::env;
use std::io;

use dotenv::dotenv;
use serde_json::{Value, json};

use crate::function_call::api::send_chat_completion;
use crate::function_call::types::{ChatResponse, JsonSchemaObject, ToolDefinition, ToolFunctionDefinition};
use crate::weather::tools::{
    tool_fetch_weather, tool_list_memos, tool_save_weather_memo, tool_search_memos,
};
use crate::weather::types::{FetchWeatherArgs, SaveMemoArgs, SearchMemosArgs};

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    println!("🌤️ 天気メモアプリを起動しました！");
    println!("何をしましょうか？（例: 東京の天気を教えて、メモを保存して、など）\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_text = input.trim();

    // ツール定義
    let tools = vec![
        // ツール1: fetch_weather
        ToolDefinition {
            r#type: "function",
            function: ToolFunctionDefinition {
                name: "fetch_weather",
                description: "指定された都市の現在の天気情報を取得する",
                parameters: JsonSchemaObject {
                    r#type: "object",
                    properties: json!({
                        "city": {
                            "type": "string",
                            "description": "都市名（日本語または英語）"
                        }
                    }),
                    required: vec!["city"],
                },
            },
        },
        // ツール2: save_weather_memo
        ToolDefinition {
            r#type: "function",
            function: ToolFunctionDefinition {
                name: "save_weather_memo",
                description: "天気情報とメモを保存する",
                parameters: JsonSchemaObject {
                    r#type: "object",
                    properties: json!({
                        "city": {
                            "type": "string",
                            "description": "都市名"
                        },
                        "weather": {
                            "type": "string",
                            "description": "天気の説明"
                        },
                        "temperature": {
                            "type": "number",
                            "description": "気温（摂氏）"
                        },
                        "note": {
                            "type": "string",
                            "description": "ユーザーのメモ"
                        }
                    }),
                    required: vec!["city", "weather", "temperature", "note"],
                },
            },
        },
        // ツール3: list_memos
        ToolDefinition {
            r#type: "function",
            function: ToolFunctionDefinition {
                name: "list_memos",
                description: "保存されている全ての天気メモを一覧表示する",
                parameters: JsonSchemaObject {
                    r#type: "object",
                    properties: json!({}),
                    required: vec![],
                },
            },
        },
        // ツール4: search_memos
        ToolDefinition {
            r#type: "function",
            function: ToolFunctionDefinition {
                name: "search_memos",
                description: "特定の都市のメモを検索する",
                parameters: JsonSchemaObject {
                    r#type: "object",
                    properties: json!({
                        "city": {
                            "type": "string",
                            "description": "検索する都市名"
                        }
                    }),
                    required: vec!["city"],
                },
            },
        },
    ];

    // メッセージ履歴
    let mut messages: Vec<Value> = vec![
        json!({"role": "system", "content": "あなたは天気情報を提供し、ユーザーのメモ管理を手伝うアシスタントです。必要に応じて適切なツールを使用してください。"}),
        json!({"role": "user", "content": user_text}),
    ];

    let client = reqwest::Client::new();
    let model = "gpt-4o-mini";
    const MAX_ITER: usize = 5;

    for iter in 0..MAX_ITER {
        let req_body = json!({
            "model": model,
            "messages": messages,
            "tools": tools,
            "temperature": 0.3,
        });

        let raw = match send_chat_completion(&client, &api_key, &req_body).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("❌ API 呼び出し失敗(iter={iter}): {e}");
                break;
            }
        };

        let parsed: ChatResponse = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("❌ JSON パース失敗(iter={iter}): {e}\nraw: {raw}");
                break;
            }
        };

        let choice = match parsed.choices.first() {
            Some(c) => c,
            None => {
                eprintln!("❌ choices 空(iter={iter})");
                break;
            }
        };
        let msg = &choice.message;

        // tool_calls がない = 最終回答
        if msg.tool_calls.is_empty() {
            println!("\n💬 {}", msg.content.clone().unwrap_or_default());
            break;
        }

        // assistant メッセージを履歴に追加
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
                "content": msg.content.clone().unwrap_or_default(),
                "tool_calls": tool_calls_json,
            })
        };
        messages.push(assistant_msg);

        // 各 tool_call を実行
        for tc in &msg.tool_calls {
            let func_name = &tc.function.name;
            let args_raw = &tc.function.arguments;

            // ツール実行
            let result = match func_name.as_str() {
                "fetch_weather" => {
                    match serde_json::from_str::<FetchWeatherArgs>(args_raw) {
                        Ok(args) => tool_fetch_weather(args).await,
                        Err(e) => Err(format!("引数パースエラー: {}", e)),
                    }
                }
                "save_weather_memo" => {
                    match serde_json::from_str::<SaveMemoArgs>(args_raw) {
                        Ok(args) => tool_save_weather_memo(args).await,
                        Err(e) => Err(format!("引数パースエラー: {}", e)),
                    }
                }
                "list_memos" => tool_list_memos().await,
                "search_memos" => {
                    match serde_json::from_str::<SearchMemosArgs>(args_raw) {
                        Ok(args) => tool_search_memos(args).await,
                        Err(e) => Err(format!("引数パースエラー: {}", e)),
                    }
                }
                _ => Err(format!("未知のツール: {}", func_name)),
            };

            // 結果を tool メッセージとして追加
            let content = match result {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("❌ ツールエラー詳細: {}", e);
                    format!("❌ エラー: {}", e)
                }
            };

            messages.push(json!({
                "role": "tool",
                "tool_call_id": tc.id,
                "content": content,
            }));
        }

        if iter == MAX_ITER - 1 {
            eprintln!("⚠️ 最大反復 {MAX_ITER} に到達。打ち切り");
        }
    }

    Ok(())
}
