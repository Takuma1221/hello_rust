# 段階 1: OpenAI Function Calling 基礎 (Rust)

## ゴール

- OpenAI Chat Completions API に `tools` (function schema) を添付して呼び出す
- モデルが返す `tool_calls` を検出し、引数(JSON 文字列)をパース
- 対応するローカル関数 (`calc_sum`) を実行し、その結果を再送して最終回答を得る
- エラー(ツール未使用 / パース失敗) を分類して扱う

## 何が新しいか

通常の Chat 呼び出し: 単に user/system メッセージを送る。
Function Calling: モデルに「呼べる関数の仕様(= JSON Schema)」を与え、モデルが _必要だと判断したときに_ 適切な JSON 引数を組み立てて `tool_calls` として返す。

## 用語整理

| 用語                  | 意味                                                                |
| --------------------- | ------------------------------------------------------------------- |
| tool(function) schema | 関数の引数構造を JSON Schema で表したもの (`parameters`)            |
| tool_calls            | モデルが「この関数をこういう引数で呼んで」と返す領域                |
| arguments             | JSON 文字列 (→ serde_json でパース)                                 |
| tool メッセージ       | 実際に関数を実行した結果をモデルに返すための message (role: "tool") |

## リクエスト構造(例)

```jsonc
{
  "model": "gpt-4o-mini",
  "messages": [
    { "role": "system", "content": "あなたは数値計算を手伝うアシスタントです" },
    { "role": "user", "content": "3と8を足して" }
  ],
  "tools": [
    {
      "type": "function",
      "function": {
        "name": "calc_sum",
        "description": "2つの数 a と b の合計を計算して返す",
        "parameters": {
          "type": "object",
          "properties": {
            "a": { "type": "number", "description": "最初の数" },
            "b": { "type": "number", "description": "次の数" }
          },
          "required": ["a", "b"]
        }
      }
    }
  ]
}
```

## 返ってくる想定レスポンス(抜粋)

```jsonc
{
  "choices": [
    {
      "message": {
        "role": "assistant",
        "tool_calls": [
          {
            "id": "call_abc123",
            "type": "function",
            "function": {
              "name": "calc_sum",
              "arguments": "{\"a\":3,\"b\":8}"
            }
          }
        ]
      }
    }
  ]
}
```

## 処理フロー

1. 1 回目呼び出し: tools を含めて送信
2. tool_calls があるか判定
   - 無ければ通常回答として content を表示し終了
3. tool_calls[0] を取り出し
   - name が `calc_sum` かチェック
   - arguments(JSON 文字列) を struct にデシリアライズ
4. ローカル関数で計算 (a + b)
5. メッセージ列を拡張:
   - (a) 1 回目 assistant の tool_calls を含むメッセージそのまま追加
   - (b) `role: "tool"`, `tool_call_id: <id>`, `content: "結果数値"` のメッセージ追加
6. 2 回目呼び出し: 拡張済 messages を送り最終回答を取得
7. 最終回答 content を表示

## Rust サンプル (`src/openai_function_call.rs` 想定)

> 実際に配置する際は必要に応じて `mod` 等を調整してください。エラーハンドリング簡略版です。

```rust
use dotenv::dotenv;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::io;

#[derive(Debug, Deserialize)]
struct ToolCallFunction {
    name: String,
    arguments: String, // JSON 文字列
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    #[serde(rename = "type")] // "function"
    call_type: String,
    function: ToolCallFunction,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    role: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Deserialize)]
struct ChoiceWrapper {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChoiceWrapper>,
}

#[derive(Debug, Deserialize)]
struct SumArgs { a: f64, b: f64 }

fn calc_sum(a: f64, b: f64) -> f64 { a + b }

pub async fn run_function_call_demo() -> anyhow::Result<()> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY")?;

    println!("足し算したい2つの数 (例: 3 8):");
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let user_input = line.trim();

    // 単純にユーザーの文章とする (自然文でもOK)
    let messages = vec![
        json!({"role": "system", "content": "あなたは計算を手伝うアシスタントです"}),
        json!({"role": "user", "content": user_input}),
    ];

    let tool_schema = json!({
        "type": "function",
        "function": {
            "name": "calc_sum",
            "description": "2つの数 a と b の合計を計算して返す",
            "parameters": {
                "type": "object",
                "properties": {
                    "a": {"type": "number", "description": "最初の数"},
                    "b": {"type": "number", "description": "次の数"}
                },
                "required": ["a", "b"]
            }
        }
    });

    let client = reqwest::Client::new();

    // 1回目: ツール付き
    let first = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": messages,
            "tools": [tool_schema],
        }))
        .send()
        .await?;

    let status = first.status();
    let body_text = first.text().await?;
    if !status.is_success() { anyhow::bail!("API error: {}\n{}", status, body_text); }

    let parsed: ChatResponse = serde_json::from_str(&body_text)?;
    let first_msg = &parsed.choices[0].message;

    if first_msg.tool_calls.is_empty() {
        // 普通の返答
        println!("モデル回答: {}", first_msg.content.clone().unwrap_or_default());
        return Ok(());
    }

    // 今回は最初の tool_call のみ
    let tc = &first_msg.tool_calls[0];
    if tc.function.name != "calc_sum" {
        println!("未知のツール要求: {}", tc.function.name);
        return Ok(());
    }

    // arguments をパース
    let args: SumArgs = match serde_json::from_str(&tc.function.arguments) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("引数パース失敗: {}\nraw: {}", e, tc.function.arguments);
            return Ok(());
        }
    };
    let result_value = calc_sum(args.a, args.b);

    // 2回目呼び出し用の messages を構築
    let mut second_messages: Vec<Value> = vec![
        json!({"role": "system", "content": "あなたは計算を手伝うアシスタントです"}),
        json!({"role": "user", "content": user_input}),
        // assistant が tool_calls を提示したメッセージ
        json!({
            "role": "assistant",
            "tool_calls": first_msg.tool_calls.iter().map(|t| json!({
                "id": t.id,
                "type": "function",
                "function": {"name": t.function.name, "arguments": t.function.arguments}
            })).collect::<Vec<_>>()
        }),
        // 実行結果を tool role で返す
        json!({
            "role": "tool",
            "tool_call_id": tc.id,
            "content": result_value.to_string()
        })
    ];

    let second = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&json!({
            "model": "gpt-4o-mini",
            "messages": second_messages,
        }))
        .send()
        .await?;

    let second_body = second.text().await?;
    let parsed2: ChatResponse = serde_json::from_str(&second_body)?;
    let answer = &parsed2.choices[0].message.content.clone().unwrap_or_default();
    println!("最終回答: {}", answer);

    Ok(())
}
```

## 最低限のエラー分類 (提案)

```rust
enum FcErrorKind { NoToolCall, UnknownTool, ArgParse }
```

段階 1 ではログ出力のみで十分。

## 実行例 (想定)

```
$ cargo run --bin hello_rust_function_call   # (または main 内で呼び出し)
足し算したい2つの数 (例: 3 8):
3 8
最終回答: 3 と 8 の合計は 11 です。
```

## 練習課題

1. arguments の JSON に余計なキー `{ "a":1, "b":2, "c":999 }` が来たらどうなるか試す
2. a/b が文字列として渡された場合を強制発生させエラー表示を改善
3. `calc_sum` を `calc_sum_or_concat` に変えて、a,b のどちらかが文字列なら連結する仕様に拡張

## 次の段階への布石

- 複数ツールにしたい場合: `tools: [tool_schema1, tool_schema2, ...]`
- 2 個以上の tool_calls 返却をループ処理するオーケストレータに抽象化
- テスト: 引数パース関数だけ先に unit test 可能

---

質問があればこのファイルを参照しつつ、次は段階 2(複数ツール)へ。
