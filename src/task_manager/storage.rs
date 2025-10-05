// タスクの保存・読み込み
// memo/storage.rs を参考に実装してください

use std::fs;
use std::path::Path;

use crate::task_manager::types::TaskList;

const TASK_FILE: &str = "data/tasks.json";

// TODO: load_tasks を実装
// ヒント: memo の load_memos とほぼ同じ
// 戻り値: Result<TaskList, String>

// TODO: save_tasks を実装
// ヒント: memo の save_memos とほぼ同じ
// 引数: task_list: &TaskList
// 戻り値: Result<(), String>

// TODO: list_tasks_formatted を実装（オプション）
// タスク一覧を整形して文字列で返す
// ヒント: 優先度、期限、完了状態を表示
