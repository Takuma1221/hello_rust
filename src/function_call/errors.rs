use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ToolCallOutcome {
    NoToolCall,
    Executed {
        tool_name: String,
        result_number: f64,
        tool_call_id: String,
        raw_arguments: String,
    },
}

#[derive(Debug)]
pub enum ToolCallError {
    EmptyChoices,
    UnknownTool(String),
    ArgParse(String),
}

impl Display for ToolCallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolCallError::EmptyChoices => write!(f, "choices が空"),
            ToolCallError::UnknownTool(n) => write!(f, "未知のツール: {n}"),
            ToolCallError::ArgParse(e) => write!(f, "引数パース失敗: {e}"),
        }
    }
}
impl std::error::Error for ToolCallError {}

#[derive(Debug)]
pub enum OpenAiCallError {
    Http(String),
    JsonBuild(String),
    JsonParse(String),
}
impl Display for OpenAiCallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAiCallError::Http(e) => write!(f, "HTTPエラー: {e}"),
            OpenAiCallError::JsonBuild(e) => write!(f, "JSON生成失敗: {e}"),
            OpenAiCallError::JsonParse(e) => write!(f, "JSONパース失敗: {e}"),
        }
    }
}
impl std::error::Error for OpenAiCallError {}
