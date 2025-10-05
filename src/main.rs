mod openai;

pub mod function_call {
    pub mod api;
    pub mod errors;
    pub mod play;
    pub mod tools;
    pub mod types;
}

pub mod memo;
pub mod weather;

fn main() {
    // メモアプリを起動
    if let Err(e) = memo::cli::run() {
        eprintln!("エラー: {}", e);
        std::process::exit(1);
    }

    // 天気アプリを起動（コメントアウトで切り替え可能）
    // let _ = weather::play::play();

    // 計算アプリを起動する場合はこちら
    // let _ = function_call::play::play();
}
