# Rust テストコード完全ガイド

## 目次

1. [テストの基本](#テストの基本)
2. [テストの書き方](#テストの書き方)
3. [アサーション（検証）](#アサーション検証)
4. [テストの実行方法](#テストの実行方法)
5. [今回書いたテストコード](#今回書いたテストコード)
6. [ベストプラクティス](#ベストプラクティス)

---

## テストの基本

### テストとは？

- コードが期待通りに動くか**自動で確認**する仕組み
- バグを早期発見できる
- リファクタリング時の安全網になる
- ドキュメントとしても機能する

### Rust のテストの特徴

- **標準でサポート**（外部ライブラリ不要）
- **高速実行**（並列実行がデフォルト）
- **型安全**（コンパイル時にチェック）

---

## テストの書き方

### 基本構造

```rust
#[cfg(test)]  // テストビルド時のみコンパイル
mod tests {
    use super::*;  // 親モジュールの全てをインポート

    #[test]  // テスト関数であることを示す
    fn test_something() {
        // 1. 準備（Arrange）
        let x = 5;
        
        // 2. 実行（Act）
        let result = x + 3;
        
        // 3. 検証（Assert）
        assert_eq!(result, 8);
    }
}
```

### 重要なアトリビュート

| アトリビュート | 説明 |
|--------------|------|
| `#[cfg(test)]` | テストビルド時のみこのモジュールをコンパイル |
| `#[test]` | この関数がテストであることを示す |
| `#[should_panic]` | パニックすることを期待するテスト |
| `#[ignore]` | デフォルトではスキップするテスト |

---

## アサーション（検証）

### 基本的なアサーション

```rust
// 値が等しいか
assert_eq!(actual, expected);
assert_eq!(2 + 2, 4);

// 値が等しくないか
assert_ne!(actual, unexpected);
assert_ne!(2 + 2, 5);

// 条件が真か
assert!(condition);
assert!(5 > 3);

// 条件が偽か
assert!(!condition);
assert!(!(5 < 3));
```

### カスタムメッセージ

```rust
assert_eq!(
    result,
    expected,
    "計算結果が間違っています: 期待値 {} だが {} でした",
    expected,
    result
);
```

### Result 型の検証

```rust
let result = some_function();

// Ok か確認
assert!(result.is_ok());

// Err か確認
assert!(result.is_err());

// Ok の中身を検証
assert_eq!(result.unwrap(), expected_value);
```

### Option 型の検証

```rust
let option = Some(42);

// Some か確認
assert!(option.is_some());

// None か確認
assert!(option.is_none());

// 中身を検証
assert_eq!(option.unwrap(), 42);
```

### 文字列の検証

```rust
let text = "Hello, Rust!";

// 含まれているか
assert!(text.contains("Rust"));

// 開始するか
assert!(text.starts_with("Hello"));

// 終了するか
assert!(text.ends_with("!"));
```

---

## テストの実行方法

### 基本コマンド

```bash
# 全テストを実行
cargo test

# 特定のテストだけ実行（名前で絞り込み）
cargo test test_add

# 特定のモジュールのテストだけ実行
cargo test memo::types

# 詳細な出力を表示
cargo test -- --nocapture

# 並列実行を無効化（順次実行）
cargo test -- --test-threads=1

# 無視されたテストも実行
cargo test -- --ignored
```

### テスト結果の見方

```
running 11 tests
test memo::storage::tests::test_list_memos_formatted_empty ... ok
test memo::storage::tests::test_load_memos_no_file ... ok
test memo::types::tests::test_new_memo_list ... ok
...

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- **ok**: テスト成功 ✅
- **FAILED**: テスト失敗 ❌
- **ignored**: スキップされたテスト
- **filtered out**: 絞り込みで除外されたテスト

---

## 今回書いたテストコード

### types.rs のテスト（7個）

#### 1. 空のリスト作成

```rust
#[test]
fn test_new_memo_list() {
    let list = MemoList::new();
    assert_eq!(list.memos.len(), 0);
}
```

**目的**: `MemoList::new()` が空のリストを作成することを確認

---

#### 2. メモを1つ追加

```rust
#[test]
fn test_add_memo() {
    let mut list = MemoList::new();
    list.add_memo("買い物".to_string());
    
    assert_eq!(list.memos.len(), 1);
    assert_eq!(list.memos[0].id, 1);
    assert_eq!(list.memos[0].content, "買い物");
}
```

**目的**: 
- メモが正しく追加されるか
- ID が 1 から始まるか
- 内容が保存されるか

---

#### 3. 複数メモを追加

```rust
#[test]
fn test_add_multiple_memos() {
    let mut list = MemoList::new();
    list.add_memo("買い物".to_string());
    list.add_memo("勉強".to_string());
    list.add_memo("運動".to_string());
    
    assert_eq!(list.memos.len(), 3);
    assert_eq!(list.memos[0].id, 1);
    assert_eq!(list.memos[1].id, 2);
    assert_eq!(list.memos[2].id, 3);
}
```

**目的**: ID が連番になることを確認

---

#### 4. メモ削除（成功）

```rust
#[test]
fn test_remove_memo_success() {
    let mut list = MemoList::new();
    list.add_memo("買い物".to_string());
    list.add_memo("勉強".to_string());
    
    let removed = list.remove_memo(1);
    
    assert!(removed);  // true が返る
    assert_eq!(list.memos.len(), 1);
    assert_eq!(list.memos[0].id, 2);
}
```

**目的**: 
- 削除が成功すると `true` が返る
- 削除後のリストが正しい

---

#### 5. メモ削除（見つからない）

```rust
#[test]
fn test_remove_memo_not_found() {
    let mut list = MemoList::new();
    list.add_memo("買い物".to_string());
    
    let removed = list.remove_memo(999);
    
    assert!(!removed);  // false が返る
    assert_eq!(list.memos.len(), 1);
}
```

**目的**: 存在しない ID を削除しようとすると `false` が返る

---

#### 6. 空リストの次 ID

```rust
#[test]
fn test_next_id_empty_list() {
    let list = MemoList::new();
    assert_eq!(list.next_id(), 1);
}
```

**目的**: 空リストの次 ID は 1

---

#### 7. 削除後の次 ID

```rust
#[test]
fn test_next_id_after_deletion() {
    let mut list = MemoList::new();
    list.add_memo("メモ1".to_string());
    list.add_memo("メモ2".to_string());
    list.add_memo("メモ3".to_string());
    list.remove_memo(2);
    
    // 削除後も次のIDは最大ID + 1
    assert_eq!(list.next_id(), 4);
}
```

**目的**: 削除してもIDが重複しないことを確認

---

### storage.rs のテスト（4個）

#### 1. ファイルがない場合の読み込み

```rust
#[test]
fn test_load_memos_no_file() {
    cleanup();
    
    // ファイルがない場合は空のリストを返す
    let result = load_memos_from_path(TEST_FILE);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().memos.len(), 0);
    
    cleanup();
}
```

**目的**: ファイルがなくてもエラーにならず、空リストを返す

---

#### 2. 保存と読み込みのラウンドトリップ

```rust
#[test]
fn test_save_and_load_memos() {
    cleanup();
    
    // メモを作成して保存
    let mut list = MemoList::new();
    list.add_memo("テストメモ1".to_string());
    list.add_memo("テストメモ2".to_string());
    
    let save_result = save_memos_to_path(&list, TEST_FILE);
    assert!(save_result.is_ok());
    
    // 読み込んで確認
    let loaded = load_memos_from_path(TEST_FILE).unwrap();
    assert_eq!(loaded.memos.len(), 2);
    assert_eq!(loaded.memos[0].content, "テストメモ1");
    assert_eq!(loaded.memos[1].content, "テストメモ2");
    
    cleanup();
}
```

**目的**: 保存したデータが正しく読み込めることを確認

---

#### 3. 空リストの整形

```rust
#[test]
fn test_list_memos_formatted_empty() {
    // 空のリストの場合
    let list = MemoList::new();
    
    let lines: Vec<String> = list
        .memos
        .iter()
        .map(|memo| format!("[id:{}] {} - {}", memo.id, memo.content, memo.created_at))
        .collect();
    
    assert_eq!(lines.len(), 0);
}
```

**目的**: 空リストの整形が正しく動作

---

#### 4. データありリストの整形

```rust
#[test]
fn test_list_memos_formatted_with_data() {
    let mut list = MemoList::new();
    list.add_memo("テストメモ1".to_string());
    list.add_memo("テストメモ2".to_string());
    
    let lines: Vec<String> = list
        .memos
        .iter()
        .map(|memo| format!("[id:{}] {} - {}", memo.id, memo.content, memo.created_at))
        .collect();
    
    let output = lines.join("\n");
    
    assert!(output.contains("テストメモ1"));
    assert!(output.contains("テストメモ2"));
    assert!(output.contains("id:1"));
    assert!(output.contains("id:2"));
}
```

**目的**: メモが正しい形式で整形される

---

## ベストプラクティス

### 1. AAA パターン（Arrange-Act-Assert）

```rust
#[test]
fn test_example() {
    // Arrange: 準備
    let mut list = MemoList::new();
    list.add_memo("test".to_string());
    
    // Act: 実行
    let removed = list.remove_memo(1);
    
    // Assert: 検証
    assert!(removed);
    assert_eq!(list.memos.len(), 0);
}
```

### 2. 1つのテストで1つのこと

❌ **悪い例**（複数のことを検証）

```rust
#[test]
fn test_everything() {
    let mut list = MemoList::new();
    list.add_memo("test".to_string());
    assert_eq!(list.memos.len(), 1);
    
    list.remove_memo(1);
    assert_eq!(list.memos.len(), 0);
    
    list.add_memo("another".to_string());
    assert_eq!(list.memos[0].id, 2);
}
```

✅ **良い例**（1つのことだけ検証）

```rust
#[test]
fn test_add_memo() {
    let mut list = MemoList::new();
    list.add_memo("test".to_string());
    assert_eq!(list.memos.len(), 1);
}

#[test]
fn test_remove_memo() {
    let mut list = MemoList::new();
    list.add_memo("test".to_string());
    list.remove_memo(1);
    assert_eq!(list.memos.len(), 0);
}
```

### 3. テスト名は具体的に

| ❌ 悪い名前 | ✅ 良い名前 |
|-----------|-----------|
| `test1` | `test_add_memo` |
| `test_remove` | `test_remove_memo_success` |
| `test_error` | `test_remove_memo_not_found` |

### 4. 副作用を残さない

```rust
#[cfg(test)]
mod tests {
    const TEST_FILE: &str = "data/test_memos.json";

    fn cleanup() {
        let _ = fs::remove_file(TEST_FILE);
    }

    #[test]
    fn test_something() {
        cleanup();  // 前
        
        // テスト実行
        
        cleanup();  // 後
    }
}
```

### 5. エッジケースをテスト

- **空の入力**: 空リスト、空文字列
- **境界値**: 最小値、最大値
- **異常系**: 存在しないID、不正な形式
- **重複**: 同じIDを2回削除

```rust
#[test]
fn test_edge_cases() {
    let list = MemoList::new();
    assert_eq!(list.next_id(), 1);  // 空リスト
    
    let removed = list.remove_memo(0);  // 存在しないID
    assert!(!removed);
}
```

---

## テストのディレクトリ構成

```
hello_rust/
├── src/
│   ├── main.rs
│   └── memo/
│       ├── mod.rs
│       ├── types.rs       # ← ユニットテストを内部に記述
│       ├── storage.rs     # ← ユニットテストを内部に記述
│       └── cli.rs
└── tests/                 # ← 統合テスト（今後追加可能）
    └── integration_test.rs
```

### ユニットテスト vs 統合テスト

| 種類 | 場所 | 目的 |
|------|------|------|
| **ユニットテスト** | `src/` 内の `#[cfg(test)]` | 個別の関数・構造体をテスト |
| **統合テスト** | `tests/` ディレクトリ | 複数モジュールの連携をテスト |

---

## まとめ

### テスト実装の流れ

1. **テストモジュールを作成** (`#[cfg(test)] mod tests`)
2. **テスト関数を書く** (`#[test] fn test_xxx()`)
3. **AAA パターン**で実装（Arrange, Act, Assert）
4. **`cargo test`** で実行
5. **エラーを修正** → 繰り返し

### 今回のテスト結果

```
✅ types.rs: 7個のテスト
✅ storage.rs: 4個のテスト
✅ 合計: 11個すべてパス
```

### テストを書くメリット

- **バグの早期発見**
- **リファクタリングが安全**
- **ドキュメントになる**
- **自信を持ってコードを変更できる**

---

## 参考リンク

- [公式ドキュメント: テスト](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example: テスト](https://doc.rust-lang.org/rust-by-example/testing.html)
- [cargo test のオプション](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

---

**Happy Testing! 🧪**
