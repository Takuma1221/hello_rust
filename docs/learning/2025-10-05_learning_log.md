# 2025-10-05 学習ログ

## 2025-10-05

### [Rust] 参照と所有権の基礎

- `&` は参照（reference）= 値を借りるだけ
- `&mut` は可変参照 = 値を変更できる借用
- 所有権を渡さないので、呼び出し側でも使い続けられる

**重要ポイント**:

- `&MemoList` で読み取り専用の借用
- `&mut MemoList` で変更可能な借用
- 所有権移動 vs 借用の使い分けが重要
- デフォルトは不変（immutable）が Rust の哲学

```rust
// 読み取り専用
pub fn save_memos(memo_list: &MemoList) -> Result<(), String> {
    // memo_list を読むだけ、変更しない
}

// 変更可能
pub fn add_memo(memo_list: &mut MemoList, content: String) {
    memo_list.add_memo(content); // 変更する
}
```

**用語リマインド**:

- borrow（借用）: 参照を使って一時的に値を使う
- ownership（所有権）: 値の責任を持つこと
- mutable（可変）: 変更できる状態

---

### [Rust] let, let mut, const の違い

- `let`: 不変変数（デフォルト）、再代入不可
- `let mut`: 可変変数、再代入可能
- `const`: コンパイル時定数、型注釈必須、大文字スネークケース

**重要ポイント**:

- Rust は安全性優先でデフォルト不変
- `mut` をつけることで「変更する」意図を明示
- `const` は実行時の計算不可（コンパイル時に確定）
- シャドーイング（同名で再宣言）は `let` で可能

```rust
let x = 5;        // 不変
let mut y = 10;   // 可変
const MAX: u32 = 100;  // 定数

y += 1;  // OK
x += 1;  // エラー！
```

**次アクション**:

- 所有権の詳細ガイド作成（今後）
- ライフタイムの学習（将来）

---

### [Rust] エラーハンドリング: .map_err() と ? 演算子

- `Result<T, E>` 型で成功（Ok）と失敗（Err）を表現
- `.map_err(|e| ...)` でエラー型を変換
- `?` 演算子で早期リターン（Err なら即座に return）

**重要ポイント**:

- `io::Error` → `String` への変換で統一したエラー型
- `?` は `match` の糖衣構文
- 複数の `?` を連鎖できる
- 関数の戻り値が `Result` の場合のみ使える

```rust
// .map_err() でエラー型変換
let content = fs::read_to_string(path)
    .map_err(|e| format!("ファイル読み込みエラー: {}", e))?;

// ? 演算子の展開イメージ
match fs::read_to_string(path) {
    Ok(content) => content,
    Err(e) => return Err(format!("エラー: {}", e)),
}
```

**用語リマインド**:

- `Result<T, E>`: 成功（Ok(T)）または失敗（Err(E)）
- error propagation: エラーの伝播
- early return: 早期リターン

---

### [Rust] イテレータと .collect()

- `.iter()` でイテレータに変換
- `.map()` で各要素を変換（遅延評価）
- `.collect()` でイテレータを Vec などに集める

**重要ポイント**:

- `.map()` は遅延評価（lazy）→ `.collect()` で初めて実行
- 型推論が賢い（戻り値の型から自動判断）
- `Vec`, `HashSet`, `String` など様々な型に変換可能
- 関数型プログラミングスタイル

```rust
let lines: Vec<String> = memo_list.memos
    .iter()
    .map(|memo| format!("[id:{}] {}", memo.id, memo.content))
    .collect();
```

**関連機能**:

- `.filter()`: 条件に合う要素のみ
- `.find()`: 最初の一致要素
- `.fold()`: 畳み込み

---

### [Rust] format! マクロと文字列操作

- `format!()` は `println!()` と同じ書式で String を返す
- `&format!()` は短命な参照（通常は `String` を返す）
- `.join()` で Vec を区切り文字でつなげる

**重要ポイント**:

- `println!` は画面出力、`format!` は String 作成
- 埋め込みは `{}` で変数を展開
- 改行は `\n`（format! 内に含められる）
- `&format!()` は寿命が短いので注意

```rust
let message = format!("Hello, {}!", name);
let lines = vec!["a", "b", "c"];
let joined = lines.join("\n");  // "a\nb\nc"
```

---

### [Rust] todo!() マクロ

- 「まだ実装してない」をマークするマクロ
- コンパイルは通るが、実行するとパニック
- 関数の雛形を作る時に便利

**重要ポイント**:

- 型チェックは通る（戻り値の型が合っていれば OK）
- 実装完了後は `Ok(())` などに置き換える
- `unimplemented!()` と似ているが、より「TODO」の意味が強い

```rust
fn my_function() -> Result<(), String> {
    todo!()  // 後で実装する
}
```

**関連マクロ**:

- `unimplemented!()`: 未実装を明示
- `unreachable!()`: 到達しないはずのコード

---

### [Rust] テストコードの書き方

- `#[cfg(test)]` でテストモジュールを定義
- `#[test]` でテスト関数をマーク
- `assert_eq!`, `assert!` で検証

**重要ポイント**:

- ユニットテストは各ファイル内に書く
- AAA パターン（Arrange, Act, Assert）を推奨
- 1 つのテストで 1 つのことを検証
- `cargo test` で全テスト実行

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_memo() {
        // Arrange
        let mut list = MemoList::new();

        // Act
        list.add_memo("test".to_string());

        // Assert
        assert_eq!(list.memos.len(), 1);
    }
}
```

**ベストプラクティス**:

- テスト名は具体的に（`test_xxx_success`, `test_xxx_failure`）
- 副作用を残さない（cleanup 関数で後始末）
- テストの分離（本番データに影響しない）

**実行コマンド**:

```bash
cargo test                  # 全テスト
cargo test test_add        # 名前で絞り込み
cargo test -- --nocapture  # 出力を表示
```

---

### [CLI] コマンドライン引数パース

- `std::env::args()` で引数を取得
- `match` でサブコマンド分岐
- `&args[2..]` でスライスを取得

**重要ポイント**:

- `args[0]` はプログラム名
- `args[1]` が第 1 引数（サブコマンド）
- `.join(" ")` で複数引数を結合
- `.parse::<u32>()` で文字列を数値に変換

```rust
let args: Vec<String> = std::env::args().collect();

match args[1].as_str() {
    "add" => handle_add(&args[2..]),
    "list" => handle_list(),
    _ => print_help(),
}
```

---

### [Concept] ドキュメント整理のベストプラクティス

プロジェクトが増えてきたので、ドキュメントを整理した。

**新構造**:

```
docs/
├── projects/         # プロジェクト別（function_calling, weather, memo）
├── learning/         # 学習ログ（日付別）
├── guides/          # 汎用技術ガイド（testing など）
└── prompts/         # Copilot 設定
```

**重要ポイント**:

- プロジェクトごとに README を作成
- 各フォルダの目的を明確に
- 汎用的な内容は guides/ に
- 学習記録は learning/ に日付で

**次アクション**:

- guides/ に所有権ガイドを追加予定
- 新プロジェクトは projects/ に追加

---

## 実装完了プロジェクト

### メモ帳 CLI アプリ（memo）

**Phase 1-3 完了** ✅

**実装内容**:

- ✅ types.rs: Memo, MemoList 構造体
- ✅ storage.rs: ファイル I/O（load, save, format）
- ✅ cli.rs: CLI 引数パース（add, list, delete）
- ✅ テスト: 11 個すべてパス

**学んだ機能**:

- 構造体とメソッド
- ファイル I/O（fs::read_to_string, fs::write）
- JSON シリアライズ（serde）
- エラーハンドリング（Result, ?, .map_err()）
- CLI 引数パース（std::env::args）
- ユニットテスト（#[test], assert_eq!）

**動作確認**:

```bash
cargo run -- add "買い物に行く"     # ✅
cargo run -- list                   # ✅
cargo run -- delete 1               # ✅
cargo test                          # ✅ 11 passed
```

---

## 次回プロジェクト準備

### タスク管理 AI アシスタント（task_manager）

**準備完了** 📝

**プロジェクト概要**:

- AI に自然言語でタスク管理を依頼
- OpenAI Function Calling を使用
- 4 つのツール（add, list, complete, delete）

**雛形作成済み**:

- `src/task_manager/` ディレクトリ
- types.rs, storage.rs, tools.rs, play.rs（TODO コメント付き）
- `docs/projects/task_manager/README.md`（仕様書）
- `docs/projects/task_manager/implementation_guide.md`（ヒント）

**学習目標**:

- AI 関数呼び出しの実践
- 日付操作（chrono）
- 複雑なフィルタリングロジック
- **ヒント少なめで自力実装**

**次回やること**:

1. types.rs から実装開始
2. memo/weather を参考に段階的に実装
3. つまずいたら質問（でもまず自分で考える）

---

## チェックポイント状況

### 完了済み

- [x] Function Calling Stage1 完了（計算ツール）
- [x] Weather アプリ完成（天気取得、メモ保存）
- [x] Memo アプリ Phase 1（types.rs）
- [x] Memo アプリ Phase 2（storage.rs）
- [x] Memo アプリ Phase 3（cli.rs）
- [x] Memo アプリ テスト実装（11 個）
- [x] ドキュメント整理完了

### 進行中

- [ ] Task Manager アプリ（次回開始）

### 今後の予定

- [ ] Task Manager 完成
- [ ] guides/ に所有権ガイド追加
- [ ] guides/ にエラーハンドリングガイド追加
- [ ] 新しい AI アプリ検討

---

## Q&A ログへの追加

本日の以下の質問を `qna_log.md` に転記済み:

- 参照（`&`）と所有権の違い
- `let` vs `let mut` vs `const`
- `.map_err()` と `?` 演算子の仕組み
- `.collect()` の役割
- `format!` と `&format!()` の違い
- `todo!()` マクロの使い方

---

**今日の学習時間**: 約 3-4 時間
**実装行数**: 約 300 行（types.rs, storage.rs, cli.rs, テスト含む）
**習得した概念**: 8 つ（参照、変数、エラー処理、イテレータ、文字列、マクロ、テスト、CLI）
