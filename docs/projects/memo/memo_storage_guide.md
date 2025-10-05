# Phase 2: storage.rs を書こう

## 🎯 このフェーズの目標

ファイルの読み書き（ファイル I/O）を実装する

---

## 📋 実装する関数

### 1. `load_memos()` - ファイルから読み込み

### 2. `save_memos()` - ファイルに保存

### 3. `list_memos_formatted()` - 一覧を整形表示（オプション）

---

## 📝 課題 2-1: 定数とインポート

**あなたがやること:**
`src/memo/storage.rs` を作成して、必要な import と定数を定義

### ヒント

```rust
use std::fs;
use std::path::Path;

use crate::memo::types::MemoList;

/// メモファイルのパス
const MEMO_FILE: &str = "data/memos.json";
```

**必要な import:**

- `std::fs` - ファイル操作用
- `std::path::Path` - パス操作用
- `crate::memo::types::MemoList` - 自作の型

---

## 📝 課題 2-2: load_memos() を実装

**あなたがやること:**
ファイルから MemoList を読み込む関数

### ヒント

```rust
/// ファイルからメモリストを読み込む
/// ファイルがなければ空のリストを返す
pub fn load_memos() -> Result<MemoList, String> {
    let path = Path::new(MEMO_FILE);

    // TODO: ファイルが存在しない場合は空のリストを返す
    // if !path.exists() { ... }

    // TODO: ファイルを読み込む
    // fs::read_to_string(path) を使う
    // .map_err(|e| format!("...")) でエラーを String に変換

    // TODO: JSON をパースして MemoList に変換
    // serde_json::from_str(&content) を使う

    // TODO: Ok(memo_list) を返す
}
```

**weather/storage.rs の `load_memos()` を参考にしてください！**

---

## 📝 課題 2-3: save_memos() を実装

**あなたがやること:**
MemoList をファイルに保存する関数

### ヒント

```rust
/// メモリストをファイルに保存
pub fn save_memos(memo_list: &MemoList) -> Result<(), String> {
    let path = Path::new(MEMO_FILE);

    // TODO: data/ ディレクトリがなければ作成
    // if let Some(parent) = path.parent() { ... }
    // fs::create_dir_all(parent) を使う

    // TODO: MemoList を JSON 文字列に変換
    // serde_json::to_string_pretty(memo_list) を使う

    // TODO: ファイルに書き込む
    // fs::write(path, json) を使う

    // TODO: Ok(()) を返す
}
```

**weather/storage.rs の `save_memos()` を参考にしてください！**

---

## 📝 課題 2-4: 表示用関数（オプション）

**あなたがやること:**
メモ一覧を見やすく整形する関数

### ヒント

```rust
/// 全メモを整形して表示
pub fn list_memos_formatted() -> Result<String, String> {
    let memo_list = load_memos()?;

    if memo_list.memos.is_empty() {
        return Ok("📭 メモはありません".to_string());
    }

    let mut output = String::from("📝 メモ一覧:\n");
    for memo in &memo_list.memos {
        // TODO: format! でメモを整形
        // output.push_str(&format!("..."));
    }

    Ok(output)
}
```

---

## 🧪 動作確認

実装が終わったら、以下でビルド確認:

```bash
cargo build
```

エラーが出たら、エラーメッセージを見せてください！一緒に直します。

---

## 💡 困ったら

- **weather/storage.rs と見比べる**
- **わからない部分はすぐ質問する**
- **コンパイルエラーは恥ずかしくない！**

---

## 🚀 準備ができたら

`src/memo/storage.rs` を開いて書き始めてください！

**できたら「書けた」と教えてください** 😊
