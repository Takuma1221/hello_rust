mod openai;
pub mod function_call {
    pub mod types;
    pub mod errors;
    pub mod api;
    pub mod tools;
    pub mod play;
}

fn main() {
    // play は #[tokio::main] 付きなので直接呼ぶと再ランタイム生成になる。
    // ここでは単純に別バイナリ化を避け現状維持: ユーザは openai_function_call.rs を将来削除して整理。
    let _ = function_call::play::play();
}
