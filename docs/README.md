# ドキュメント構成

このディレクトリには、Rust 学習プロジェクトのドキュメントが整理されています。

## 📁 フォルダ構成

```
docs/
├── projects/           # プロジェクトごとのガイド・仕様
│   ├── function_calling/
│   ├── weather/
│   └── memo/
├── learning/           # 学習ログ・Q&A
├── guides/             # 汎用的な技術ガイド（Rust全般）
└── prompts/            # GitHub Copilot 用プロンプト設定
```

---

## 📂 各フォルダの説明

### `projects/` - プロジェクト別ドキュメント

各プロジェクトの仕様書、ガイド、実装メモを格納。

#### `projects/function_calling/`

OpenAI Function Calling の学習プロジェクト（計算ツール）

- `stage1.md` - Stage1（計算ツール）の実装ガイド
- `enum_trait_basics.md` - Enum と Trait の基礎学習
- `file_split_refactor.md` - ファイル分割リファクタリング

#### `projects/weather/`

天気メモアプリ（OpenWeatherMap API 連携）

- `weather_memo_app_spec.md` - アプリ仕様書
- `weather_setup_guide.md` - セットアップ手順

#### `projects/memo/`

シンプルメモ帳 CLI アプリ

- `memo_app_guide.md` - Phase 1（types.rs）ガイド
- `memo_storage_guide.md` - Phase 2（storage.rs）ガイド
- `memo_cli_guide.md` - Phase 3（cli.rs）ガイド

---

### `learning/` - 学習記録

日々の学習ログと Q&A を保存。

- `2025-09-13_learning_log.md` - 学習ログ（9/13）
- `2025-09-15_learning_log.md` - 学習ログ（9/15）
- `qna_log.md` - 質問と回答のログ

---

### `guides/` - 技術ガイド

特定プロジェクトに依存しない、汎用的な Rust の技術ガイド。

- `rust_testing_guide.md` - Rust テストコード完全ガイド
  - テストの書き方
  - アサーションの使い方
  - ベストプラクティス

（今後、エラーハンドリング、所有権、非同期プログラミングなどのガイドを追加予定）

---

### `prompts/` - Copilot 設定

GitHub Copilot の動作をカスタマイズするプロンプト設定。

- `code_comment_style.md` - コメントのスタイルガイド
- `commit_push_policy.md` - Git コミット・プッシュのポリシー
- `response_tone_guide.md` - 応答トーンのガイド
- `update_docs_instruction.md` - ドキュメント更新の指示

---

## 🔍 ドキュメントの探し方

### プロジェクトの実装を進めたい

→ `projects/` 内の該当プロジェクトフォルダを参照

### Rust の基礎を学びたい

→ `guides/` 内の技術ガイドを参照

### 過去の学習内容を振り返りたい

→ `learning/` 内の学習ログを参照

### Copilot の動作を調整したい

→ `prompts/` 内の設定ファイルを編集

---

## 📝 ドキュメント作成ルール

1. **プロジェクト固有の内容** → `projects/プロジェクト名/` に配置
2. **汎用的な技術ガイド** → `guides/` に配置
3. **日々の学習記録** → `learning/` に日付付きで配置
4. **設定ファイル** → `prompts/` に配置

---

## 🗂️ 今後の拡張予定

### guides/ に追加予定

- `rust_ownership_guide.md` - 所有権・借用の詳細ガイド
- `rust_error_handling_guide.md` - エラーハンドリング完全ガイド
- `rust_async_guide.md` - 非同期プログラミングガイド
- `rust_traits_guide.md` - トレイトの詳細ガイド

### projects/ に追加予定

- 新しいプロジェクトごとにフォルダを作成

---

**最終更新**: 2025 年 10 月 5 日
