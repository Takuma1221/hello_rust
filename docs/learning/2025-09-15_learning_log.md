# 学習ログ 2025-09-15

## 2025-09-15

### Stage1 Step3: arguments を構造体にパース
- モデルが返す `tool_calls[].function.arguments` は JSON 文字列。
- `serde_json::from_str::<SumArgs>(&raw)` で安全に型へ変換。
- 目的: 手動パースよりバグ減 / 型チェック / 将来拡張しやすい。

```rust
#[derive(serde::Deserialize)]
struct SumArgs { a: f64, b: f64 }
let raw = &tc.function.arguments;
let args: SumArgs = serde_json::from_str(raw)?; // 失敗時 Err
```

### raw 変数を一度挟む理由
- 同じ `tc.function.arguments` を複数回書かず可読性向上。
- ログ出力で再利用しやすい (`{raw}`)。
- 今後前処理 (trim, 長さ制限) を追加しやすい。
- 借用を一度で済ませる。

### match を使った Result 分岐
- `match` で `Ok(v)` と `Err(e)` を分けてログ + 早期 return。
- `?` では任意のログを挟めないので今回用途と合わない。

```rust
let args: SumArgs = match serde_json::from_str(raw) {
    Ok(v) => v,
    Err(e) => {
        eprintln!("引数パース失敗: {e}\nraw: {raw}");
        return Ok(());
    }
};
```

### Step2 -> Step3 の差分認識
| 項目 | Step2 | Step3 |
| ---- | ----- | ----- |
| tool_calls 判定 | 有無だけ | 先頭を取得し実データ利用 |
| arguments | 文字列のまま | `SumArgs` にパース |
| 関数実行 | なし | `calc_sum(a,b)` を呼ぶ |
| エラー処理 | JSON全体パース | 引数パース追加 |

### チェックポイント状況
- [x] Stage1 Step1 初回リクエスト
- [x] Stage1 Step2 tool_calls 判定
- [x] Stage1 Step3 引数パース & 関数実行
- [ ] Stage1 Step4 2 回目呼び出し (tool メッセージ送信)

---
参照: `docs/learning/function_calling/stage1.md`, `src/openai_function_call.rs`, `docs/learning/qna_log.md`
