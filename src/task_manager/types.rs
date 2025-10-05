// タスク管理アプリの型定義
// ここに Task, TaskList, Priority を実装してください

use chrono::Local;
use serde::{Deserialize, Serialize};

// TODO: Priority 列挙型を定義
// ヒント: High, Medium, Low の3つ
// ヒント: Serialize, Deserialize を derive

// TODO: Task 構造体を定義
// フィールド:
//   - id: u32
//   - title: String
//   - due_date: String  (例: "2025-10-06")
//   - priority: Priority
//   - completed: bool
//   - created_at: String

// TODO: TaskList 構造体を定義
// フィールド:
//   - tasks: Vec<Task>

// TODO: TaskList の impl ブロック
// 実装するメソッド:
//   - new() -> Self
//   - add_task(&mut self, title: String, due_date: String, priority: Priority)
//   - find_task(&self, title: &str) -> Option<&Task>
//   - find_task_mut(&mut self, title: &str) -> Option<&mut Task>
//   - complete_task(&mut self, title: &str) -> bool
//   - delete_task(&mut self, title: &str) -> bool
//   - list_by_filter(&self, filter: &str) -> Vec<&Task>

// ヒント: weather や memo の types.rs を参考に！
// ヒント: メソッドは段階的に実装していけばOK
