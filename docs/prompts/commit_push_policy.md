# Git コミット / プッシュ運用ポリシー

目的: 小さめの変更をこまめに共有し、あとで振り返りやすくするための目安です。

## 基本原則

- 「一言で説明できるまとまり」になったらコミットで区切る
- メッセージは 60 文字以内を目安 (長くなる = まとめ過ぎ)
- 変更したコードと関連する docs は一緒に保存
- ビルド失敗状態は残さない (生成物除く)

## 自動コミット & push 条件

1. 公開 API / 型 (struct / enum / trait / 関数シグネチャ) 追加や変更
2. 実コード 30 行超の追加・変更 (コメント除く)
3. `docs/` または `prompts/` に新しいファイル追加
4. 既存ドキュメント 20 行超の更新
5. ツール呼び出しフローへ影響するロジック修正
6. テスト追加または既存テスト修正

## 見送るケース

- 空白や整形だけ (少し溜めてからでも可)
- 途中で形が揺れている下書き (TODO 付きで後続仕上げ)

## コミットメッセージ規約

形式: `<種別>: <概要>` (種別は英語, 概要は簡潔な日本語で十分)

種別:

- feat: 新機能 / 目に見える追加
- fix: バグ修正
- refactor: 動作変えない整理
- docs: ドキュメントや学習ログ
- chore: 環境設定 / 依存更新 など
- test: テスト関連

例:

- feat: add SumArgs parsing for tool call
- fix: handle empty tool_calls gracefully
- docs: add lifetime &'a str Q&A
- refactor: split request/response types

## 作業フロー (Assistant 側)

1. 変更する
2. `cargo build --quiet` (将来: `cargo test`) で成功を確認
3. 条件チェック → 該当なら commit & push
4. 応答でコミットハッシュ + 変更要約を共有

## 衝突回避

- 少人数想定で `main` 直 push。必要になればブランチ運用へ移行
- 衝突時: `git pull --rebase` → ビルド再確認 → push

## ロールバック方針

- 直前だけ戻す: `git revert <hash>` (履歴を壊さない)
- まとまった巻き戻し: `git reset --hard <hash>` (ローカルのみ, 慎重に)

## 将来拡張アイデア

- pre-commit で fmt / clippy 自動チェック
- Conventional Commits へ拡張

## 改訂履歴

- v1 (2025-09-13): 初版作成
- v1.1 (2025-09-13): 初学者向けに文面を丁寧化
