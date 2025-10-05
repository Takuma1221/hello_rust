# Phase 3: CLI 実装ガイド

## 目標

コマンドライン引数を解析して、メモの追加・一覧表示・削除を実行できるようにする。

## 実装する機能

```bash
# メモを追加
cargo run -- memo add "買い物に行く"

# メモ一覧を表示
cargo run -- memo list

# メモを削除
cargo run -- memo delete 1
```

## 学ぶこと

1. **コマンドライン引数の取得**: `std::env::args()`
2. **パターンマッチング**: `match` 式でサブコマンド処理
3. **Vec の要素アクセス**: `.get()` と `.iter().skip().collect()`
4. **文字列のパース**: `.parse::<u32>()`

## 実装する関数

### `src/memo/cli.rs`

```rust
use crate::memo::storage;
use crate::memo::types::MemoList;

/// メモアプリの CLI エントリーポイント
pub fn run() -> Result<(), String> {
    // 1. コマンドライン引数を取得
    let args: Vec<String> = std::env::args().collect();
    
    // 2. 引数が足りない場合はヘルプを表示
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    // 3. サブコマンドで分岐
    match args[1].as_str() {
        "add" => handle_add(&args[2..]),
        "list" => handle_list(),
        "delete" => handle_delete(&args[2..]),
        _ => {
            println!("❌ 不明なコマンド: {}", args[1]);
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("📝 メモアプリ");
    println!("使い方:");
    println!("  cargo run -- memo add <内容>    - メモを追加");
    println!("  cargo run -- memo list           - メモ一覧を表示");
    println!("  cargo run -- memo delete <id>    - メモを削除");
}

fn handle_add(args: &[String]) -> Result<(), String> {
    // TODO: 実装する
    // 1. args が空なら "内容を指定してください" エラー
    // 2. args を全部つなげてメモ内容にする（args.join(" ")）
    // 3. load_memos() で読み込み
    // 4. add_memo() で追加
    // 5. save_memos() で保存
    // 6. "✅ メモを追加しました" と表示
    todo!()
}

fn handle_list() -> Result<(), String> {
    // TODO: 実装する
    // 1. list_memos_formatted() を呼ぶ
    // 2. 結果を println! で表示
    todo!()
}

fn handle_delete(args: &[String]) -> Result<(), String> {
    // TODO: 実装する
    // 1. args が空なら "ID を指定してください" エラー
    // 2. args[0] を u32 にパース（.parse::<u32>()）
    // 3. load_memos() で読み込み
    // 4. remove_memo() で削除（成功したか確認）
    // 5. save_memos() で保存
    // 6. "✅ メモを削除しました" または "❌ メモが見つかりません" と表示
    todo!()
}
```

## ヒント

### コマンドライン引数の取得

```rust
let args: Vec<String> = std::env::args().collect();
// args[0] = プログラム名（"target/debug/hello_rust"）
// args[1] = 第1引数（"memo"）
// args[2] = 第2引数（"add"）
// args[3..] = 残りの引数
```

### スライスの使い方

```rust
let args = vec!["add", "買い物", "する"];
let sub_args = &args[1..];  // ["買い物", "する"]
let joined = sub_args.join(" ");  // "買い物 する"
```

### 文字列のパース

```rust
let id_str = "42";
let id: u32 = id_str.parse()
    .map_err(|_| "数値に変換できません".to_string())?;
```

### MemoList のメソッド呼び出し

```rust
let mut memo_list = storage::load_memos()?;
memo_list.add_memo("買い物".to_string());
storage::save_memos(&memo_list)?;
```

## 次のステップ

1. **cli.rs を作成**して、上記の TODO を実装
2. **mod.rs を更新**して、`pub mod cli;` を追加
3. **main.rs を更新**して、`memo::cli::run()` を呼び出す
4. **動作確認**: `cargo run -- memo add "テスト"`

頑張ってください！🚀
