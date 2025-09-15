//! Stage1 Step1-2: 最初のリクエストと tool_calls が付いたかどうかを見るところまで。
//! ここではまだ calc_sum を実際に動かしたり 2 回目のリクエスト送信はしません。

use dotenv::dotenv; // .env から OPENAI_API_KEY を読む
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE}; // 認証用の定数
use serde::{Deserialize, Serialize}; // 型 <-> JSON 変換 (derive)
use serde_json::{Value, json}; // json! マクロや動的値
use std::collections::HashMap; // 関数名→実行関数のレジストリ用
use std::env; // 環境変数参照
use std::io; // 標準入力読み込み

// ===== モデルから「受け取る」形 =====
// リクエストで送った形とは微妙に構造が違うため分離しています。
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
struct OutgoingMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct ToolDefinition<'a> {
    #[serde(rename = "type")]
    r#type: &'a str,
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
    r#type: &'a str,
    properties: Value,
    required: Vec<&'a str>,
}

#[derive(serde::Deserialize)]
struct SumArgs {
    a: f64,
    b: f64,
}

// ツール呼び出し結果の意味を Option ではなく列挙型で表現。
enum ToolCallOutcome {
    NoToolCall, // tool_calls が無かった
    Executed {
        tool_name: String,
        result_number: f64, // 今回は数値のみ (Stage1 想定)
    },
}

// 失敗理由 (簡易)。必要に応じて発展させる (ParseError など細分化)。
#[derive(Debug)]
enum ToolCallError {
    EmptyChoices,
    UnknownTool(String),
    ArgParse(String),
}

impl std::fmt::Display for ToolCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCallError::EmptyChoices => write!(f, "choices が空"),
            ToolCallError::UnknownTool(n) => write!(f, "未知のツール: {n}"),
            ToolCallError::ArgParse(e) => write!(f, "引数パース失敗: {e}"),
        }
    }
}

impl std::error::Error for ToolCallError {}

// Chat API 呼び出し用の汎用エラー (簡易版)
#[derive(Debug)]
enum OpenAiCallError {
    Http(String),
    JsonBuild(String),
    JsonParse(String),
}

impl std::fmt::Display for OpenAiCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAiCallError::Http(e) => write!(f, "HTTPエラー: {e}"),
            OpenAiCallError::JsonBuild(e) => write!(f, "JSON生成失敗: {e}"),
            OpenAiCallError::JsonParse(e) => write!(f, "JSONパース失敗: {e}"),
        }
    }
}
impl std::error::Error for OpenAiCallError {}

// 送信用メッセージ型 (今は OutgoingMessage を再利用)

// リクエスト JSON を組み立てる (tools は引数 Vec で柔軟性を残す)
fn build_request_json<'a>(
    model: &str,
    messages: &[OutgoingMessage<'a>],
    tools: &[ToolDefinition<'a>],
) -> serde_json::Value {
    json!({
        "model": model,
        "messages": messages,
        "tools": tools
    })
}

// 実際に送信し文字列ボディを返す
async fn send_chat_completion(
    client: &reqwest::Client,
    api_key: &str,
    body: &serde_json::Value,
) -> Result<String, OpenAiCallError> {
    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(body)
        .send()
        .await
        .map_err(|e| OpenAiCallError::Http(e.to_string()))?;
    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| OpenAiCallError::Http(e.to_string()))?;
    if !status.is_success() {
        return Err(OpenAiCallError::Http(format!(
            "status={} body={}",
            status, text
        )));
    }
    Ok(text)
}

// (今後 Step4 で複数ツールを扱うなら enum やマップ化を検討するが、Stage1 は 1 個に固定)

pub fn calc_sum(a: f64, b: f64) -> f64 {
    return a + b;
}

pub fn calc_times(a: f64, b: f64) -> f64 {
    return a * b;
}

// ChatResponse から最初の tool_call (calc_sum) を拾って実行する小さなヘルパ。
// 見つからなければ Ok(None)。失敗理由は Err(String) で簡易返却。
// 関数名→ (a,b)->f64 のシンプルな関数ポインタを登録しておき、選択された関数を実行。
// 戻り値: Ok(Some(result)) 実行成功 / Ok(None) tool_call 無し / Err(...) エラー。

// OKの型(None, Some)ではなく、専用の型で返す
// AIにsendする別関数を作る　引数：プロンプト、関数一覧、会話履歴（Toolの結果を含めて）
fn execute_first_tool_call(
    parsed: &ChatResponse,
    registry: &HashMap<String, fn(f64, f64) -> f64>,
) -> Result<ToolCallOutcome, ToolCallError> {
    let first_choice = parsed.choices.first().ok_or(ToolCallError::EmptyChoices)?;
    let msg = &first_choice.message;
    if msg.tool_calls.is_empty() {
        return Ok(ToolCallOutcome::NoToolCall);
    }
    let tc = &msg.tool_calls[0];
    println!(
        "tool_calls 検出: {} (arguments raw JSON string)",
        tc.function.name
    );
    let func_name = &tc.function.name;
    let func = registry
        .get(func_name)
        .ok_or_else(|| ToolCallError::UnknownTool(func_name.clone()))?;
    let raw = &tc.function.arguments;
    let args: SumArgs = serde_json::from_str(raw)
        .map_err(|e| ToolCallError::ArgParse(format!("{e}; raw: {raw}")))?;
    let result = func(args.a, args.b);
    Ok(ToolCallOutcome::Executed {
        tool_name: func_name.clone(),
        result_number: result,
    })
}

#[tokio::main]
pub async fn play() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // .env が無くてもエラーにはしない
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

    // ===== レジストリ (増えたらここに追加) =====
    let mut registry: HashMap<String, fn(f64, f64) -> f64> = HashMap::new();
    registry.insert("calc_sum".into(), calc_sum as fn(f64, f64) -> f64);
    registry.insert("calc_times".into(), calc_times as fn(f64, f64) -> f64); // まだ schema 未公開

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
    // ===== 1 回目の呼び出し (組み立て → 送信) =====
    let request_json = build_request_json("gpt-4o-mini", &messages, &[tool_schema]);
    let body_text = match send_chat_completion(&client, &api_key, &request_json).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("API 呼び出し失敗: {e}");
            return Ok(());
        }
    };

    // JSON 文字列 -> 型 (tool_calls が付いたか見るため)
    let parsed: ChatResponse = match serde_json::from_str(&body_text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("JSON パース失敗: {}\nraw: {}", e, body_text);
            return Ok(());
        }
    };

    // ===== 最初の候補を簡潔に処理 =====
    match execute_first_tool_call(&parsed, &registry) {
        Ok(ToolCallOutcome::Executed {
            tool_name,
            result_number,
        }) => {
            println!("ツール {tool_name} 実行結果: {result_number}");
        }
        Ok(ToolCallOutcome::NoToolCall) => {
            if let Some(first) = parsed.choices.first() {
                println!(
                    "通常回答: {}",
                    first.message.content.clone().unwrap_or_default()
                );
            }
        }
        Err(err) => eprintln!("tool 処理エラー: {err}"),
    }

    Ok(())
}
