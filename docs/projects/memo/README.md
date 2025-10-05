# Memo CLI アプリ

シンプルなコマンドラインメモ帳アプリ。

## 📚 概要

CLI からメモの追加・一覧表示・削除ができるシンプルなメモ帳。
Rust の基礎（型、ファイル I/O、エラーハンドリング、テスト）を学習するプロジェクト。

## 📁 ドキュメント

- **`memo_app_guide.md`** - Phase 1: types.rs の実装ガイド
- **`memo_storage_guide.md`** - Phase 2: storage.rs の実装ガイド
- **`memo_cli_guide.md`** - Phase 3: cli.rs の実装ガイド

## 🎯 実装済み機能

- ✅ メモ追加（タイムスタンプ自動付与）
- ✅ メモ一覧表示（整形済み）
- ✅ メモ削除（ID 指定）
- ✅ JSON 形式で永続化
- ✅ 包括的なユニットテスト（11 個）

## 📂 関連コード

```
src/memo/
├── mod.rs          # モジュール定義
├── types.rs        # 型定義（Memo, MemoList）+ テスト
├── storage.rs      # ファイル I/O（保存・読み込み・整形）+ テスト
└── cli.rs          # CLI 引数パース・ハンドラ
```

## 🚀 使い方

### メモを追加

```bash
cargo run -- add "買い物に行く"
cargo run -- add "Rust の勉強をする"
```

### メモ一覧を表示

```bash
cargo run -- list
```

### メモを削除

```bash
cargo run -- delete 1
```

## 🗂️ データ保存先

```
data/memos.json
```

## 🧪 テスト

```bash
# 全テストを実行
cargo test

# 特定のテストだけ実行
cargo test test_add
cargo test memo::types
```

### テスト内容

- ✅ types.rs: 7 個のテスト（追加、削除、ID 生成）
- ✅ storage.rs: 4 個のテスト（ファイル I/O、整形）

## 📖 学習した概念

### Phase 1: 型定義

- 構造体（Struct）
- impl ブロック
- 可変性（mut）
- イテレータ（.iter(), .map(), .max()）
- Vec の操作（.push(), .retain()）

### Phase 2: ファイル I/O

- std::fs（read_to_string, write, create_dir_all）
- serde（Serialize, Deserialize）
- Result 型とエラーハンドリング
- .map_err() と ? 演算子
- Path と PathBuf

### Phase 3: CLI

- コマンドライン引数（std::env::args）
- パターンマッチング（match）
- スライス（&[String]）
- 文字列パース（.parse()）
- エラーメッセージの設計

### テスト

- #[cfg(test)] と #[test]
- アサーション（assert_eq!, assert!）
- AAA パターン（Arrange, Act, Assert）
- テストの分離（cleanup）
- ユニットテスト vs 統合テスト

## 🎓 学習の進め方

1. **Phase 1** - `memo_app_guide.md` を読んで types.rs を実装
2. **Phase 2** - `memo_storage_guide.md` を読んで storage.rs を実装
3. **Phase 3** - `memo_cli_guide.md` を読んで cli.rs を実装
4. **テスト** - `../guides/rust_testing_guide.md` を参考にテストを書く

各フェーズで「読む → 実装 → レビュー → 概念理解」のサイクルで学習。
