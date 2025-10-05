# シンプルメモ帳 CLI - 学習ガイド

## 🎯 このプロジェクトで学ぶこと

- Rust の基本的なデータ型
- ファイル I/O（JSON 読み書き）
- CLI 引数の扱い方
- エラーハンドリング

---

## 📝 実装する順番

### Phase 1: データ型定義（types.rs）← 今ここから！

### Phase 2: ファイル操作（storage.rs）

### Phase 3: CLI 処理（cli.rs）

### Phase 4: 動作確認

---

## 🏗️ Phase 1: types.rs を書こう

### 課題 1-1: Memo 構造体を定義する

**あなたがやること:**
`src/memo/types.rs` を作成して以下を実装してください。

#### ヒント

- メモには以下の情報が必要です：

  - `id`: u32（メモの番号）
  - `content`: String（メモの内容）
  - `created_at`: String（作成日時）

- 必要な derive:
  - `Debug`: デバッグ表示用
  - `Clone`: 複製可能にする
  - `Serialize`: JSON 保存用
  - `Deserialize`: JSON 読み込み用

#### テンプレート

```rust
use serde::{Deserialize, Serialize};

/// 1つのメモを表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memo {
    // TODO: ここにフィールドを追加
}
```

---

### 課題 1-2: MemoList 構造体を定義する

**あなたがやること:**
`Memo` の配列を保持する構造体を作る

#### ヒント

- JSON ファイル全体の構造:

```json
{
  "memos": [...]
}
```

#### テンプレート

```rust
/// メモのリスト全体
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoList {
    // TODO: memos フィールドを追加（Vec<Memo>型）
}
```

---

### 課題 1-3: メソッドを実装する

**あなたがやること:**
`MemoList` に以下のメソッドを実装

1. `new()` - 空のリストを作る
2. `add_memo(&mut self, content: String)` - メモを追加
3. `remove_memo(&mut self, id: u32)` - ID でメモを削除
4. `next_id(&self)` - 次の ID を取得（最大 ID + 1）

#### ヒント

```rust
impl MemoList {
    /// 空のメモリストを作成
    pub fn new() -> Self {
        // TODO: 実装
    }

    /// 新しいメモを追加
    pub fn add_memo(&mut self, content: String) {
        // TODO:
        // 1. next_id() で新しい ID を取得
        // 2. 現在時刻を取得（chrono::Local::now()）
        // 3. Memo を作成
        // 4. self.memos に追加
    }

    /// メモを削除
    pub fn remove_memo(&mut self, id: u32) -> bool {
        // TODO:
        // self.memos.retain() を使って id が一致しないものだけ残す
        // 削除できたら true, できなかったら false
    }

    /// 次の ID を計算
    fn next_id(&self) -> u32 {
        // TODO:
        // self.memos が空なら 1
        // そうでなければ最大の id + 1
    }
}
```

---

## 💡 わからないことがあったら

**すぐに聞いてください！** 以下のような質問で OK：

- 「Vec ってどう使うの？」
- 「retain って何？」
- 「時刻の取得方法は？」
- 「これで合ってる？」

---

## 🚀 準備ができたら

`src/memo/types.rs` を開いて、書き始めてください！

わからない部分は **一緒に考えましょう** 💪
