# Git コミット / プッシュ運用ポリシー

目的: コードやドキュメントの変更を小さく安全に積み上げ、再現性とレビュー容易性を高める。

## 基本原則
- "一言で説明できる" まとまりができたら即コミット。
- 目安: コミットメッセージが 60 文字以内で機能差分を表現できるか。
- コードとドキュメントが同じトピックなら一緒にコミット (例: 構造体追加 + 学習ログメモ)。
- ビルドが通らない状態のコミットは禁止 (生成コード除く)。

## 自動コミットトリガ基準 (アシスタント判断)
次のいずれかに該当したら push まで行う:
1. 公開 API / 型定義 (struct / enum / trait / 関数シグネチャ) を変更/追加
2. 30 行以上のコード差分 (コメント除く) を追加/変更
3. docs/ または prompts/ に新しいファイル追加
4. 既存ドキュメント大幅更新 (20 行以上)
5. ツール呼び出しフローに影響するロジック修正
6. 新しいテスト追加または既存テスト修正

## 見送るケース
- スペースやフォーマットのみ (複数溜めてからでも可)
- 未完成の途中下書き (コメントで TODO を残し後続で完了予定時)

## コミットメッセージ規約
形式: `<種別>: <概要>`  (プレフィックスは英語, 概要は簡潔日本語可)

推奨種別:
- feat: 新機能 / 目に見える追加
- fix: バグ修正
- refactor: 振る舞い変更なしの整理
- docs: ドキュメント/ログ/ガイドのみ
- chore: 設定や依存、細かな非機能
- test: テスト関連

例:
- feat: add SumArgs parsing for tool call
- fix: handle empty tool_calls gracefully
- docs: add lifetime &'a str Q&A
- refactor: split request/response types

## 作業フロー (Assistant 側)
1. 変更適用
2. `cargo build --quiet` / (将来) `cargo test --quiet` 成功確認
3. コミット基準該当チェック → YES なら commit/push
4. 応答内でコミットハッシュと要約表示

## 衝突回避
- main 直 push 方針 (少人数前提) / ブランチ運用が必要になったら改訂
- 衝突検知時は `git pull --rebase` → 再ビルド → 再 push

## ロールバック方針
- 直前コミットだけ戻したい: `git revert <hash>` を推奨 (履歴保持)
- 連続複数をまとめて取り消し: `git reset --hard <hash>` は原則ローカルのみで使用

## 将来拡張アイデア
- pre-commit hook で `cargo fmt -- --check` / `cargo clippy -- -D warnings`
- Conventional Commits 形式へ昇格

## 改訂履歴
- v1 (2025-09-13): 初版作成
