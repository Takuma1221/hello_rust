# 学習ログ (開始日: 2025-09-13)

このファイルはセッション中に得た知識を逐次追記していくログです。日付ごとに節を追加し、用語 → 要約 → サンプル → チェックポイントの順で整理します。

## 2025-09-13

### Option / Some / None

- `Option<T>` は値が「ある/ない」を表す列挙型: `Some(T)` or `None`。
- 典型パターン: `if let Some(v) = opt { ... }` / `match` で網羅。
- よく使うメソッド: `map`, `and_then`, `unwrap_or`, `ok_or`。

### if let パターン

- `match` の簡略記法。指定パターンにマッチした場合のみブロック実行。

```rust
if let Some(first) = vec.first() {
    println!("{}", first);
} else {
    println!("空");
}
```

### serde の `#[serde(rename = "...")]`

- フィールド名と JSON のキー名が違うときに対応付ける。
- フィールド名が同じなら不要。

```rust
#[derive(Serialize, Deserialize)]
struct Message {
    #[serde(rename = "role")] // JSON "role"
    kind: String,              // Rust 内では kind
}
```

### シリアライズ / デシリアライズ

- シリアライズ: Rust 値 -> JSON 文字列など。
- デシリアライズ: JSON 文字列 -> Rust 型。
- 目的: ネットワーク/API/保存媒体とやり取り可能にする。

```rust
#[derive(Serialize, Deserialize)]
struct User { id: u32, name: String }
let u = User { id: 1, name: "Taro".into() };
let json_text = serde_json::to_string(&u)?; // {"id":1,"name":"Taro"}
let back: User = serde_json::from_str(&json_text)?;
```

### OpenAI Function Calling: schema とは

- 関数の引数仕様を JSON Schema 形式で宣言したものを API に渡す。
- モデルは tool_calls で `function.name` と `arguments`(JSON 文字列) を返す。
- 例 (calc_sum): properties / required で型と必須性を伝える。

### tool_calls 判定 (Stage1 ステップ 2 まで)

- レスポンス構造体に `tool_calls: Vec<ToolCall>` を追加し、空/非空で分岐。

```rust
if !msg.tool_calls.is_empty() { /* tool call */ } else { /* 通常回答 */ }
```

- まだ arguments のパースや 2 回目呼び出しは未実装。

### cargo build --quiet

- ビルド出力(進行ログ)を抑制し、警告/エラーのみ表示。
- CI やスクリプトでログをコンパクトにしたいときに有効。

### serde default 属性

- 欠けたフィールドを `Default` 実装値で自動補填しエラーにしない。
- 後方互換性確保 / 一部設定省略を許可するときに有効。

```rust
#[derive(serde::Deserialize, Default)]
struct Limits { max_conn: u32, timeout_ms: u64 }
#[derive(serde::Deserialize)]
struct Cfg { #[serde(default)] limits: Limits }
```

- `Option<T>` は存在有無、`default` は具体値補填で役割が異なる。

### 配列の表現

- 固定長: `[T; N]` (例: `[u8;3]` RGB)
- 可変長: `Vec<T>` (最頻)
- 参照スライス: `&[T]` (借用・所有権不要)

```rust
struct Color { rgb: [u8; 3] }
struct Numbers { items: Vec<i32> }
fn sum(slice: &[i32]) -> i32 { slice.iter().sum() }
```

### オブジェクト(JSON)の表現

- 形固定: `struct`
- キー可変: `HashMap<String, T>`
- 型未確定/混在: `serde_json::Value`
- 追加プロパティ吸収: `#[serde(flatten)]` + `HashMap`

```rust
#[derive(serde::Deserialize)]
struct User { id: u32, name: String }
#[derive(serde::Deserialize)]
struct Users { map: std::collections::HashMap<String, User> }
```

### impl ブロック

- 型にメソッド/関連関数やトレイト実装を与えるための構文。
- 自動生成されないので `new()` など欲しい API は自分で書く。
- 複数 impl に分割可 (役割ごと整理)。

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

- `impl Trait` 構文(戻り値位置など)とは別概念。
- `derive` で生成される実装(例: Debug, Default)とも区別。

### チェックポイント状況

- [x] Stage1 ステップ 1: ツール付き初回リクエスト送信
- [x] Stage1 ステップ 2: tool_calls 有無判定
- [ ] Stage1 ステップ 3: arguments パース & 関数実行
- [ ] Stage1 ステップ 4: tool メッセージ追加で 2 回目呼び出し

---

追記ルールは `prompts/update_docs_instruction.md` を参照。Stage1 の詳細は `docs/function_calling/stage1.md`。Q&A 一覧は `docs/learning/qna_log.md`。 (Q&A へ serde default / 配列 / オブジェクト 表現を追加済)

**運用変更メモ (2025-09-13 後半)**: Q&A ログ方針を「再参照しそうなもののみ」→「挨拶/軽微確認以外の全質問を必ず記録」に更新。スキップ時は理由コメントを本ログに残す。

### 所有 (ownership) / 借用 (borrowing)

- Rust のメモリ安全性は「唯一の所有者 + 参照の整合性」ルールで保証。
- 所有移動(move): `let b = a;` で `a` がムーブし以後 `a` 使用不可 (Copy 型除く)。
- 借用参照: `&T` は読み取り専用を複数同時可、`&mut T` は排他的 1 つのみ。
- 不変参照と可変参照は同一ライフタイムで共存不可 (データ競合防止)。
- 参照は所有者より長く生きられない (ライフタイム規則で静的検査)。

```rust
let mut s = String::from("abc");
let r1 = &s;        // 不変参照 OK
let r2 = &s;        // もう一つ不変 OK
// let m = &mut s;  // エラー: 不変参照が生存中
println!("{}{}", r1, r2); // r1/r2 使用終了
let m = &mut s;     // ここなら可変参照取得可
m.push('x');
```

- 関数へ所有を渡すか借用で済ませるかは「その関数が値を保持する必要があるか」で判断。
- `clone()` は所有移動を避けたいが同じヒープ内容を複製したい場合にのみ使用 (コスト意識)。

Q&A 追記メモ: 詳細な Q&A を `qna_log.md` に追加 (所有/借用)。

### ライフタイム付き参照 `&'a str`

- `&str` は UTF-8 文字列スライス参照、`'a` はその参照が有効な期間ラベル。
- 複数の参照引数と戻り値の寿命関係を明示するための“期間ジェネリクス”。
- 省略規則(elision)で単純ケース(引数 1 つなど)は明示不要。
- `&'static str` はプログラム全期間有効 (文字列リテラル)。

```rust
fn pick<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }
struct Holder<'a> { name: &'a str }
```

Q&A 追記メモ: `&'a str` のエントリを qna_log.md に追加。
