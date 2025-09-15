use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

use crate::function_call::errors::OpenAiCallError;
use crate::function_call::types::{OutgoingMessage, ToolDefinition};
use serde_json::Value;

pub fn build_request_json<'a>(
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

// 1 回目返信の tool_call を受けローカルで実行した結果を tool ロールとして差し込み、
// その続きの最終回答を得るための payload を構築する。
// messages_original: 最初に送った system+user (今後履歴伸ばすなら Vec<Value> で保持しても良い)
pub fn build_second_payload<'a>(
    model: &str,
    original_messages: &[OutgoingMessage<'a>],
    tool_call_id: &str,
    tool_name: &str,
    raw_arguments: &str,
    result_number: f64,
) -> Value {
    // assistant (tool_call 宣言) と tool (結果) を追加する簡易形。
    // 将来: 最初のレスポンス JSON からそのまま assistant メッセージを再利用する形に変えても良い。
    let mut msgs: Vec<Value> = Vec::new();
    for m in original_messages {
        msgs.push(json!({"role": m.role, "content": m.content}));
    }
    // assistant: tool_calls 宣言 (今回は 1 件想定)
    msgs.push(json!({
        "role": "assistant",
        "tool_calls": [{
            "id": tool_call_id,
            "type": "function",
            "function": {"name": tool_name, "arguments": raw_arguments}
        }]
    }));
    // tool: 実行結果
    msgs.push(json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "content": result_number.to_string()
    }));

    json!({
        "model": model,
        "messages": msgs
    })
}

pub async fn send_chat_completion(
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
