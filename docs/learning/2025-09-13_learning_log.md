# 学習ログ (開始日: 2025-09-13)

このファイルは学んだ内容を順番に積み上げるログです。日付ごとに節を作り、用語 → 要約 → サンプル → チェックポイントの流れを意識します。

## 2025-09-13

### Option / Some / None

- `Option<T>` は「値がある/ない」を表す列挙型: `Some(T)` or `None`。
- よく見る形: `if let Some(v) = opt { ... }` / `match` で網羅。
- よく使うメソッド: `map`, `and_then`, `unwrap_or`, `ok_or`。

### if let パターン

- `match` の短縮。欲しい形だけ取り出して残りは捨てたい時に楽。

```rust
if let Some(first) = vec.first() {
    println!("{}", first);
} else {
    println!("空");
}
```

### serde の `#[serde(rename = "...")]`

- フィールド名と JSON のキー名が違うときに合わせる。
- 同じなら不要。

```rust
#[derive(Serialize, Deserialize)]
struct Message {
    #[serde(rename = "role")] // JSON "role"
    kind: String,              // Rust 内では kind
}
```

### シリアライズ / デシリアライズ

- シリアライズ: Rust 値 -> JSON。
- デシリアライズ: JSON -> Rust 型。
- 目的: API や保存でやり取り可能にする。

```rust
#[derive(Serialize, Deserialize)]
struct User { id: u32, name: String }
let u = User { id: 1, name: "Taro".into() };
let json_text = serde_json::to_string(&u)?; // {"id":1,"name":"Taro"}
let back: User = serde_json::from_str(&json_text)?;
```

### OpenAI Function Calling: schema とは

- 関数の引数仕様を JSON Schema 形式で渡す仕組み。
- モデルは `tool_calls` に `function.name` と `arguments`(JSON文字列) を載せる。
- 例 (calc_sum): properties / required で型と必須を伝える。

### tool_calls 判定 (Stage1 ステップ 2 まで)

- `tool_calls: Vec<ToolCall>` を見て空/非空で分岐。

```rust
if !msg.tool_calls.is_empty() { /* tool call */ } else { /* 通常回答 */ }
```

- まだ arguments のパースや 2 回目呼び出しは未実装。

### cargo build --quiet

- 進行ログを抑えて警告/エラーだけ見たい時に便利。
- CI や自動スクリプトで出力を短く保つ。

### serde default 属性

- 欠けたフィールドを `Default` で埋めてエラーを避けるテクニック。
- 後方互換や設定省略を許したいとき有効。

```rust
#[derive(serde::Deserialize, Default)]
struct Limits { max_conn: u32, timeout_ms: u64 }
#[derive(serde::Deserialize)]
struct Cfg { #[serde(default)] limits: Limits }
```

- `Option<T>` = あるかどうか / `default` = 値を補う。

### 配列の表現

- 固定長: `[T; N]` (例: RGB `[u8;3]`)
- 可変長: `Vec<T>`
- 借用読み取りのみ: `&[T]`

```rust
struct Color { rgb: [u8; 3] }
struct Numbers { items: Vec<i32> }
fn sum(slice: &[i32]) -> i32 { slice.iter().sum() }
```

### オブジェクト(JSON)の表現

- 形固定: `struct`
- キー可変: `HashMap<String, T>`
- 型いろいろ: `serde_json::Value`
- 余剰キー吸収: `flatten + HashMap`

```rust
#[derive(serde::Deserialize)]
struct User { id: u32, name: String }
#[derive(serde::Deserialize)]
struct Users { map: std::collections::HashMap<String, User> }
```

### impl ブロック

- 型にメソッド/関連関数/トレイト実装を追加する枠。
- `new()` など欲しい便利関数は自作。
- 分割して整理して良い。

```rust
struct User { id: u32, name: String }

impl User {                 // 固有実装 (inherent methods)
    fn new(id: u32, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
    fn id(&self) -> u32 { self.id }
}

impl std::fmt::Display for User { // トレイト実装
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({}, {})", self.id, self.name)
    }
}
```

- `impl Trait` や derive とは役割が違う。

### チェックポイント状況

- [x] Stage1 ステップ 1: ツール付き初回リクエスト送信
- [x] Stage1 ステップ 2: tool_calls 有無判定
- [ ] Stage1 ステップ 3: arguments パース & 関数実行
- [ ] Stage1 ステップ 4: tool メッセージ追加で 2 回目呼び出し

---

参照: `prompts/update_docs_instruction.md` / `docs/function_calling/stage1.md` / `docs/learning/qna_log.md`

【運用変更 (2025-09-13)】Q&A は挨拶/軽微確認以外は全部記録。省いたら理由をここにメモ。

### 所有 (ownership) / 借用 (borrowing)

- Rust の安全性 = 「所有者は 1 つ」+「参照ルール」で確保。
- move: `let b = a;` で `a` は以後使えない (Copy 型除く)。
- `&T` は複数OK / `&mut T` は 1 つだけ。
- 不変と可変は同時不可。
- 参照は元データより長生きできない。

```rust
let mut s = String::from("abc");
let r1 = &s;        // 不変参照 OK
let r2 = &s;        // もう一つ不変 OK
// let m = &mut s;  // エラー: 不変参照が生存中
println!("{}{}", r1, r2); // r1/r2 使用終了
let m = &mut s;     // ここなら可変参照取得可
m.push('x');
```

- 所有渡し vs 借用 = 関数が長期保持するかどうか。
- `clone()` は本当に複製したい時だけ (コスト注意)。

Q&A 追記メモ: 詳細な Q&A を `qna_log.md` に追加 (所有/借用)。

### ライフタイム付き参照 `&'a str`

- `&str` は文字列スライス参照。`'a` はその参照たちの生存グループ名。
- 複数参照と戻り値の関係を示す印。
- 単純な形は省略可。`&'static str` は全期間有効 (リテラル)。

```rust
fn pick<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }
struct Holder<'a> { name: &'a str }
```

Q&A 追記メモ: `&'a str` のエントリを qna_log.md に追加。
