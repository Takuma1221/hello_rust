# 段階 1: OpenAI Function Calling 基礎 (Rust)

## ゴール

この段階では「モデルに関数を教えて、必要なら呼んでもらい、その結果を返して最終回答を受け取る」最小の流れを体に入れます。

- Chat Completions に `tools` (関数定義) を付けて送る
- 返ってきたメッセージで `tool_calls` があるか確かめる
- 引数 (JSON 文字列) を安全に構造体へ変換
- ローカル関数 (`calc_sum`) を実行し結果を tool メッセージとして再送
- 「ツール使われなかった」「引数がおかしい」など基本エラーを区別

## 何が新しいか

通常の Chat は文章のやり取りだけです。Function Calling では「こういう名前の関数があり、こういう形の引数を受け取れますよ」と宣言しておくと、モデルが「これはコードで処理した方が良い」と判断した場面で `tool_calls` にその関数名と JSON 形式の引数を入れて返してくれます。こちらはその JSON をパースして実行し、結果を戻すことで最終回答をより正確にさせられます。

## 用語整理

| 用語                  | 説明                                                                 |
| --------------------- | -------------------------------------------------------------------- |
| tool(function) schema | 関数の引数の形 (型・必須項目) を JSON Schema で書いた宣言            |
| tool_calls            | モデル側が「この関数をこの引数で実行してほしい」と示す配列          |
| arguments             | その関数に渡す引数を表す JSON 文字列 (後で構造体にパース)           |
| tool メッセージ       | こちらが実行結果を返すためのメッセージ (role が "tool")             |

## リクエスト構造 (例)

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

## 返ってくる予想レスポンス (抜粋)

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

1. 1 回目: tools を付けて送る
2. `tool_calls` が空か確認 (空なら普通の回答なので終了)
3. 最初の `tool_call` を取り出す (今回は 1 件前提で簡略)
4. 関数名が期待どおりか確認 (`calc_sum`)
5. `arguments` (JSON文字列) を `SumArgs` にパース
6. 関数を実行 (a + b)
7. 2 回目用メッセージ: (a) 1 回目 assistant メッセージ (tool_calls 付き) (b) 実行結果の tool メッセージ
8. 2 回目呼び出し → 最終回答の content を表示

## Rust サンプル (`src/openai_function_call.rs` 抜粋)

主要な流れに集中するためエラー処理は最小に絞っています。細かな改善 (再試行や分類) は後の段階で検討します。

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

## 最低限のエラー分類 (案)

```rust
enum FcErrorKind { NoToolCall, UnknownTool, ArgParse }
```

今は型だけメモしておき、実際の分岐はログ出力で代用。後続で Result へ昇格させても良いです。

## 実行例 (想定)

```
$ cargo run --bin hello_rust_function_call   # (または main 内で呼び出し)
足し算したい2つの数 (例: 3 8):
3 8
最終回答: 3 と 8 の合計は 11 です。
```

## 練習課題

1. 余計なキー (c など) が来たらパースはどう振る舞うか確認
2. a / b を文字列にしてエラーメッセージの改善案を考える
3. 文字列が混ざったら連結、それ以外は加算する関数に拡張

## 次の段階への布石

- ツールを複数に増やす (配列へ足すだけで拡張可能)
- 複数 `tool_calls` を順に処理するループ設計
- 引数パース部分だけを切り出して単体テスト

---

不明点があればいつでも戻ってこれます。次は段階 2 (複数ツール) へ進みます。
