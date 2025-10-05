// AI が呼び出すツール関数
// weather/tools.rs を参考に実装してください

use serde_json::{json, Value};

use crate::task_manager::storage;
use crate::task_manager::types::Priority;

// TODO: tool_add_task を実装
// 引数: args: &Value (JSON から title, due_date, priority を取得)
// 処理: storage から読み込み → add_task → 保存
// 戻り値: Result<String, String>

// TODO: tool_list_tasks を実装
// 引数: args: &Value (JSON から filter を取得)
// 処理: storage から読み込み → フィルター → 整形
// 戻り値: Result<String, String>

// TODO: tool_complete_task を実装
// 引数: args: &Value (JSON から title を取得)
// 処理: storage から読み込み → complete_task → 保存
// 戻り値: Result<String, String>

// TODO: tool_delete_task を実装
// 引数: args: &Value (JSON から title を取得)
// 処理: storage から読み込み → delete_task → 保存
// 戻り値: Result<String, String>

// TODO: get_tool_definitions を実装
// OpenAI に渡すツール定義の JSON を作成
// ヒント: weather の get_tool_definitions を参考に
// 戻り値: Vec<Value>

// ヒント: 引数の取り出し方
// let title = args["title"].as_str().unwrap_or("");

// ヒント: Priority の変換
// let priority = match priority_str {
//     "high" => Priority::High,
//     "low" => Priority::Low,
//     _ => Priority::Medium,
// };
