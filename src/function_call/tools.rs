use std::collections::HashMap;

use crate::function_call::errors::{ToolCallError, ToolCallOutcome};
use crate::function_call::types::{ChatResponse, SumArgs};

pub type NumFunc = fn(f64, f64) -> f64;

pub fn execute_first_tool_call(
    parsed: &ChatResponse,
    registry: &HashMap<String, NumFunc>,
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
    tool_call_id: tc.id.clone(),
    raw_arguments: raw.clone(),
    })
}
