# Function Calling プロジェクト

OpenAI の Function Calling 機能を学習するプロジェクト。

## 📚 概要

GPT モデルに計算ツール（sum, times, pow）を提供し、自動的に適切なツールを呼び出してもらう学習プロジェクト。

## 📁 ドキュメント

- **`stage1.md`** - Stage1 実装ガイド（計算ツールの実装）
- **`enum_trait_basics.md`** - Enum と Trait の基礎学習
- **`file_split_refactor.md`** - ファイル分割リファクタリング手順

## 🎯 実装済み機能

- ✅ 計算ツール（sum, times, pow）
- ✅ 複数ツール呼び出しループ（MAX_ITER=5）
- ✅ ファイル分割（types, errors, api, tools, play）
- ✅ エラーハンドリング（カスタムエラー型）

## 📂 関連コード

```
src/function_call/
├── api.rs          # OpenAI API 連携
├── errors.rs       # エラー型定義
├── play.rs         # メインループ
├── tools.rs        # ツール実装（sum, times, pow）
└── types.rs        # 型定義
```

## 🚀 実行方法

```bash
# main.rs で function_call::play::play() のコメントアウトを解除
cargo run
```

## 📖 学習した概念

- OpenAI Function Calling の仕組み
- Enum と Trait を使った型安全な設計
- カスタムエラー型の実装
- モジュール分割のベストプラクティス
