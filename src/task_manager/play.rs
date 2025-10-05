// メインループ
// weather/play.rs を参考に実装してください

use reqwest::Client;
use serde_json::{json, Value};
use std::io::{self, Write};

use crate::task_manager::tools;

const OPENAI_API_KEY: &str = env!("OPENAI_API_KEY");
const MAX_ITER: u32 = 5;

// TODO: play 関数を実装
// 処理の流れ:
// 1. ユーザー入力を受け取る
// 2. OpenAI API にリクエスト
// 3. tool_calls があれば実行
// 4. 結果を会話履歴に追加
// 5. 再度 API 呼び出し（最大 MAX_ITER 回）
// 6. AI の返答を表示

// ヒント: weather/play.rs とほぼ同じ構造
// ヒント: get_tool_definitions で定義したツールを使う
// ヒント: match でツール名によって処理を分岐

pub async fn play() {
    println!("🤖 タスク管理 AI アシスタント");
    println!("何か話しかけてみてください（例: 明日までにレポート書く）\n");

    // TODO: ここに実装
    // let client = Client::new();
    // let mut messages: Vec<Value> = vec![];
    
    // loop {
    //     // ユーザー入力
    //     // OpenAI 呼び出し
    //     // ツール実行
    //     // 結果表示
    // }
    
    println!("実装してください！");
}
