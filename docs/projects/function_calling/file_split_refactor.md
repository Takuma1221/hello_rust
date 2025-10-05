# ファイル分割リファクタ記録 (Stage1)

## 目的

単一ファイル `openai_function_call.rs` に集中していた責務を分割し、見通しと変更容易性を向上。Step4 (2 回目 API 呼び出し) やツール追加に備える。

## 分割前の主な責務

- 型定義 (受信/送信/ツール JSON Schema)
- エラー表現 (ToolCallError, OpenAiCallError)
- ツール呼び出し実行ロジック (execute_first_tool_call)
- HTTP リクエスト組み立て & 送信
- CLI I/O (標準入力受け取り)
- メイン処理 (play)

1 ファイル内で縦に並び、関心ごとが混在していた。

## 分割後構成

```
src/
  function_call/
    types.rs         // API 入出力 & ツール定義型
    errors.rs        // Outcome / ドメイン & 通信エラー型
    api.rs           // build_request_json / send_chat_completion
    tools.rs         // execute_first_tool_call (レジストリ経由で実行)
    play.rs          // ユースケース本体 (CLI → 1回目呼び出し → 判定)
main.rs              // モジュール公開 & play 起動
```

## それぞれの責務詳細

| ファイル  | 役割                                              | 今後の拡張余地                                     |
| --------- | ------------------------------------------------- | -------------------------------------------------- |
| types.rs  | シリアライズ/デシリアライズ対象の純粋なデータ構造 | 複数ツール用 args enum, 2 回目 tool message 型追加 |
| errors.rs | ドメイン状態(Outcome) + 失敗理由(enum)            | thiserror 導入 / エラー連鎖(source)                |
| api.rs    | OpenAI Chat API 呼び出し共通部                    | リトライ / タイムアウト / logging middleware       |
| tools.rs  | 受信 tool_calls の検査 & 実行                     | 複数 tool_calls ループ / 部分実行 / 並列化検討     |
| play.rs   | Stage1 シナリオ制御                               | Step4 2 回目呼び出し・会話履歴管理                 |
| main.rs   | エントリポイント                                  | 将来: clap でサブコマンド化                        |

## 移動マッピング (主関数・型)

| 旧                                                | 新        |
| ------------------------------------------------- | --------- |
| ChatResponse 他                                   | types.rs  |
| ToolCallOutcome / ToolCallError / OpenAiCallError | errors.rs |
| build_request_json / send_chat_completion         | api.rs    |
| execute_first_tool_call                           | tools.rs  |
| calc_sum / calc_times + play                      | play.rs   |

## 期待効果

- 開発者が「どこを開けば何があるか」を直感的に把握
- 変更時の差分が狭い範囲に留まりレビュー容易
- Step4 以降で増える: 会話履歴型 / 2 回目メッセージ構築関数 を新ファイル or 既存への自然追加がしやすい

## 既知の課題 / TODO 候補

1. `OpenAiCallError` の未使用バリアント(JsonBuild/JsonParse) → 実利用か削除
2. `openai.rs` の未使用 `play` 関数 (警告) → 削除検討
3. `play.rs` に #[tokio::main] → main.rs との二重ランタイム防止のため将来 main 側に集約
4. Step4 実装: ツール結果を tool ロールメッセージに変換 →2 回目 API 呼び出し → 最終回答表示
5. レジストリ: 現在文字列 → 関数ポインタ固定。将来クロージャ対応なら `Box<dyn Fn(...)>` 化
6. 結果型: `ToolCallOutcome::Executed` が数値限定 → enum ToolResult { Number(f64), Text(String) } 拡張案

## 次ステップ提案 (優先順)

1. Step4 コード化 (second_messages 生成 + 再送 API)
2. OpenAiCallError 精緻化 or thiserror 導入
3. ToolResult enum 拡張 & JSON 変換ヘルパ
4. ユニットテスト: args パース / execute_first_tool_call のエラー分岐
5. clap 導入で `hello_rust function-call` などサブコマンド化

---

(以上) 分割理由と新構成、今後の拡張ポイントをまとめました。
