use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: IncomingMessage,
}

#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: CalledFunction,
}

#[derive(Debug, Deserialize)]
pub struct CalledFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize)]
pub struct OutgoingMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ToolDefinition<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub function: ToolFunctionDefinition<'a>,
}

#[derive(Debug, Serialize)]
pub struct ToolFunctionDefinition<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub parameters: JsonSchemaObject<'a>,
}

#[derive(Debug, Serialize)]
pub struct JsonSchemaObject<'a> {
    #[serde(rename = "type")]
    pub r#type: &'a str,
    pub properties: Value,
    pub required: Vec<&'a str>,
}

#[derive(serde::Deserialize)]
pub struct SumArgs {
    pub a: f64,
    pub b: f64,
}
