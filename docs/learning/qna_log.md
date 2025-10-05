# Q&A 集約ログ

繰り返し参照したい質問と回答を整理。新しい質問は末尾へ追加。

---

### 質問: Function Calling の arguments を型にパースする理由

日付: 2025-09-15
カテゴリ: [OpenAI] [Serde]
Q: `function.arguments` をそのまま文字列で扱わず `SumArgs` にする目的は？
A: モデルは JSON 文字列を返すだけなので、型へデシリアライズして型安全・欠落/型不一致検出・拡張容易性を得る。手動で `split` 等するよりバグと分岐を減らせる。余計なキーは無視される (厳密化したいなら `#[serde(deny_unknown_fields)]`)。
コード例:

```rust
#[derive(serde::Deserialize)]
struct SumArgs { a: f64, b: f64 }
let raw = &tc.function.arguments;
let args: SumArgs = serde_json::from_str(raw)?;
let result = calc_sum(args.a, args.b);
```

関連用語: serde_json, from_str, deny_unknown_fields
再発防止メモ: 文字列のまま index せず構造体に寄せる癖を付ける。

---

### 質問: 一度 `let raw = &tc.function.arguments;` に置く利点

日付: 2025-09-15
カテゴリ: [Rust]
Q: 直接 `from_str(&tc.function.arguments)` と書かない理由は？
A: (1) 繰り返し書かずに可読性向上 (2) ログで `{raw}` を再利用 (3) 将来前処理追加余地 (trim/長さ制限) (4) 借用を一箇所に集中 (5) 意図「これが生の JSON」を名前で明示。
コード例:

```rust
let raw = &tc.function.arguments; // 生 JSON 文字列
if raw.len() > 200 { /* 省略表示など拡張しやすい */ }
let args: SumArgs = serde_json::from_str(raw)?;
```

関連用語: borrow, readability, refactoring point
再発防止メモ: 同じ式を 2 回以上使うなら一度束縛する選択肢を検討。

---

### 質問: match で Result を展開する書き方の意図

日付: 2025-09-15
カテゴリ: [Rust]
Q: `match serde_json::from_str(raw) { Ok(v) => v, Err(e) => { ... return Ok(()); } }` の狙い？
A: ログ出力とカスタム早期終了を挟みたいので `?` ではなく `match`。成功分岐は `SumArgs` を返し失敗はメッセージ表示後に関数終了。`if let` でも書けるが失敗側で早期 return を書く形になり、`match` の方が Ok/Err が並び分かりやすい。
コード例:

```rust
let args: SumArgs = match serde_json::from_str(raw) {
  Ok(v) => v,
  Err(e) => {
    eprintln!("引数パース失敗: {e} raw:{raw}");
    return Ok(()); // ここで静かに終了
  }
};
```

関連用語: Result, ?演算子, early return
再発防止メモ: ログ + 続行制御が必要なら `match` / `if let`、素通しで良ければ `?`。

---

### 質問: `&'a str` の `'a` は何？

日付: 2025-09-13
カテゴリ: [Rust]
Q: `&'a str` の書き方と意味が混乱する。
A: `&str` は文字列スライス参照、`'a` はその参照のライフタイム(有効期間ラベル)。`&'a str` は「少なくとも `'a` の間有効な &str」。ジェネリクス同様に関数や構造体でパラメータ化し、複数参照の生存関係をコンパイラへ明示する。単一引数関数などは省略規則(elision)で書かなくて良い。戻り値がどの引数に紐づくか不透明な場合に明示する。
コード例:

```rust
fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }
struct Holder<'a> { name: &'a str } // Holder が name より長生きできない
```

関連用語: lifetime, elision, &'static str, borrow checker
再発防止メモ: `'a` は特別な意味の文字でなく任意ラベル。複数参照間の相対寿命を縛るための“型レベルの期間変数”。

---

### 質問: String と &str の違い

日付: 2025-09-13
カテゴリ: [Rust]
Q: mismatched types: expected `String`, found `&str` というエラー。違いは？
A: `String` はヒープ所有の可変長 UTF-8。`&str` は既存文字列領域への借用スライスで不変。所有が必要/保持するなら `String`、借用だけで良い引数は `&str`。エラーは関数が所有文字列(`String`)を要求しているのにリテラル(`&'static str`)を渡したため。
コード例:

```rust
fn need_string(s: String) {}
fn need_str(s: &str) {}

need_string("hi".to_string()); // OK
need_str("hi");               // OK
let owned = String::from("hi");
need_str(&owned);              // &String -> &str
```

関連用語: ownership, to_string, to_owned, AsRef<str>
再発防止メモ: 引数は基本 `&str` で受け取り内部で保持する場合のみ `String` 化。

---

### 質問: impl とは何か

日付: 2025-09-13
カテゴリ: [Rust]
Q: `impl` の役割は？
A: 型にメソッドや関連関数、トレイトの実装を与えるブロック。`new()` は自動では生成されないので必要なら固有 impl 内で定義。トレイト実装用の `impl Trait for Type` 形式と、固有メソッド用の `impl Type` 形式がある。複数に分けて整理可能。
コード例:

```rust
struct User { id: u32 }
impl User { fn new(id: u32) -> Self { Self { id } } fn id(&self) -> u32 { self.id } }
impl std::fmt::Display for User { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "User({})", self.id) } }
```

関連用語: trait, inherent method, associated function, derive
再発防止メモ: `Type::new()` は自作したときだけ存在。

---

### 質問: serde の #[serde(default)] の意味

日付: 2025-09-13
カテゴリ: [Serde]
Q: `#[serde(default)]` は何をする？
A: 入力 JSON にフィールドが無い場合にエラーにせず `Default` 実装や指定関数で初期値を埋める。必須 → 任意化でき後方互換性確保に便利。`Option<T>` とは役割が異なり、`Option` は None を入れ、`default` は具体値を入れる。
コード例:

```rust
#[derive(serde::Deserialize, Default)]
struct Limits { max_conn: u32, timeout_ms: u64 }

#[derive(serde::Deserialize)]
struct Cfg {
  #[serde(default)] limits: Limits,              // 無ければ Limits::default()
  #[serde(default)] features: Vec<String>,       // []
  #[serde(default)] port: u16,                   // 0
}
```

関連用語: Default, Option, backward compatibility
再発防止メモ: 必須項目に付けると検知漏れするので注意。

---

### 質問: 配列 / オブジェクト(JSON) を Rust でどう表現する？

日付: 2025-09-13
カテゴリ: [Rust]
Q: JSON の配列やオブジェクトを構造体でどう表す？
A: 配列は固定長なら `[T; N]`、可変長は `Vec<T>`。オブジェクトは形が固定なら struct、キー可変なら `HashMap<String, T>`、型も未確定なら `serde_json::Value`。追加プロパティ吸収は `#[serde(flatten)]` + `HashMap` 併用。
コード例:

```rust
#[derive(serde::Deserialize)]
struct User { id: u32, name: String }

#[derive(serde::Deserialize)]
struct Wrapper { users: std::collections::HashMap<String, User> }

#[derive(serde::Deserialize)]
struct Color { rgb: [u8; 3] } // 固定長

#[derive(serde::Deserialize)]
struct Config {
  #[serde(flatten)] extra: std::collections::HashMap<String, serde_json::Value>,
}
```

関連用語: Vec, HashMap, Value, flatten
再発防止メモ: "形が決まっているか/キーが動的か/型が不明か" の三択で考える。

---

### 質問: Option の Some と is_empty の違い

日付: 2025-09-13
カテゴリ: [Rust]
Q: `Some` と `is_empty()` の違いは？
A: `Some` は `Option<T>` の「値が存在する」バリアント。`is_empty()` はコレクションや文字列の内容が空か (len==0) を判定するメソッドで、存在そのものではなく中身のサイズを見ている。`Some(String::new())` は「値はあるが中身は空」状態になり得る。
コード例:

```rust
let s: Option<String> = Some(String::new());
assert!(s.is_some());                 // 値はある
assert!(s.as_ref().is_some_and(|v| v.is_empty())); // 中身は空
```

関連用語: Option, Some, None, is_some, is_none
再発防止メモ: 「存在チェック」と「中身の空」は別軸で考える。

---

### 質問: serde の #[serde(rename = "role")] の意味

日付: 2025-09-13
カテゴリ: [Serde]
Q: `#[serde(rename = "role")]` は何をしている？
A: Rust フィールド名と JSON キー名が異なるとき、シリアライズ/デシリアライズ時に JSON キーを強制的に `role` にする指定。フィールド名が同じなら不要。
コード例:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Message { #[serde(rename = "role")] kind: String }
```

関連用語: rename_all, default, Option<T>
再発防止メモ: 外部 API 仕様が優先されるときだけ rename。

---

### 質問: シリアライズ / デシリアライズ とは

日付: 2025-09-13
カテゴリ: [Serde]
Q: 用語がわからない。
A: シリアライズは Rust の値を JSON 等の文字列/バイト列に変換。デシリアライズはその逆で文字列を Rust 型に戻す。API 通信では文字列形式が必要なので往復で利用する。
コード例:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct User { id: u32, name: String }
let txt = serde_json::to_string(&User { id: 1, name: "Taro".into() })?;
let u: User = serde_json::from_str(&txt)?;
```

関連用語: JSON, serde, to_string, from_str
再発防止メモ: 「梱包/開封」の比喩で覚える。

---

### 質問: Function Calling の schema とは

日付: 2025-09-13
カテゴリ: [OpenAI]
Q: schema って何？
A: モデルに渡す関数(ツール)の引数構造定義。JSON Schema 形式で type/properties/required を指定し、モデルが正しい arguments JSON を生成できるようにする。
コード例(抜粋):

```json
{
  "type": "function",
  "function": {
    "name": "calc_sum",
    "parameters": {
      "type": "object",
      "properties": { "a": { "type": "number" }, "b": { "type": "number" } },
      "required": ["a", "b"]
    }
  }
}
```

関連用語: tool_calls, arguments, JSON Schema
再発防止メモ: parameters=引数の型表。

---

### 質問: cargo build --quiet の意味

日付: 2025-09-13
カテゴリ: [Tooling]
Q: `--quiet` は何？
A: ビルド進行ログを抑制し、警告とエラーのみ表示するフラグ。CI やスクリプトでログを簡潔に保つ目的。
コード例:

```bash
cargo build --quiet
```

関連用語: cargo, build, warn
再発防止メモ: 出力が減るだけで最適化などは変わらない。

---

### 質問: tool_calls の判定方法

日付: 2025-09-13
カテゴリ: [OpenAI]
Q: tool_calls があるかどうやって判定する？
A: レスポンスで message.tool_calls が空かどうかを見る。`if !msg.tool_calls.is_empty() { ... }`。空なら通常回答。
コード例:

```rust
if !msg.tool_calls.is_empty() {
    println!("tool call: {}", msg.tool_calls[0].function.name);
}
```

関連用語: Vec, is_empty, Option
再発防止メモ: Option ではなく Vec なので is_empty。

---

### 質問: 所有 (ownership) と 借用 (borrowing) の違い

日付: 2025-09-13
カテゴリ: [Rust]
Q: 所有と借用の概念的な違いは？なぜ必要？
A: 所有はメモリ上のデータ(ヒープ資源)のライフタイムをどの変数が解放責任を持つかを静的に決めるルール。借用は「その所有権を奪わずに一時的に参照する」行為。Rust は GC が無いので「唯一の所有者がスコープを抜けたら drop」という単純規則で安全に解放できる。借用は参照 `&T` (不変) と可変参照 `&mut T` の 2 種。コンパイル時の借用チェッカが以下を保証する:

1. どの時点でも「可変参照は最大 1 つ」または「不変参照はいくつでも」だが両立しない (同時に可変 & 不変は不可)。
2. 参照は元の所有者より長く生きられない (ダングリング防止)。

コード例(所有移動と借用):

```rust
fn takes(s: String) { println!("{}", s); }        // 所有を消費 (move)
fn borrow(s: &str) { println!("{}", s); }          // 借用のみ (参照)

fn main() {
  let a = String::from("hello");
  borrow(&a);            // a はまだ使える (借用)
  // takes(a);            // これを呼ぶと a の所有権は関数へ移動し以後 a 使用不可
  takes(a.clone());       // clone でヒープ複製し元の a を保持
  println!("still: {}", a); // OK

  let mut v = 10;
  let r1 = &v;            // 不変参照
  let r2 = &v;            // もう1つ不変参照 OK
  println!("{} {}", r1, r2);
  // let m = &mut v;      // 同時に可変参照は禁止 (r1/r2 が生存中)

  let mref = &mut v;      // 不変参照の使用が終わった後なら可変参照取得OK
  *mref += 1;
}
```

所有権が move される典型: `let b = a;` (String, Vec 等はデータをコピーせず所有権移転)。コピー型(usize, bool, char, Copy 派生)は move でなくビットコピー。借用の利点はコストゼロで共有できる点、所有移動はライフタイム明確化と不要コピー削減。迷ったら「関数が保持する必要があるなら所有(String/Vec 等), 使い終われば不要なら &借用」。

関連用語: move, shallow copy, deep copy, clone, &T, &mut T, Copy, ライフタイム
再発防止メモ: エラー `borrow of moved value` が出たら「所有権どこで move されたか」を探す思考ルーチンを徹底。

---

### 質問: 参照（`&`）と所有権の違い

日付: 2025-10-05
カテゴリ: [Rust]
Q: 型に `&` がつくのはなぜ？`&MemoList` のような書き方の意味は？
A: `&` は参照（reference）= 値を借りるだけで所有権は移動しない。関数に値を渡すとデフォルトでは所有権が移動（move）して元の場所では使えなくなるが、参照なら呼び出し後も使える。`&T` は読み取り専用、`&mut T` は変更可能な借用。
コード例:

```rust
// 所有権が移動（元で使えなくなる）
fn consume(list: MemoList) { }

// 参照で借りる（元で使える）
fn save_memos(list: &MemoList) { }

let my_list = MemoList::new();
save_memos(&my_list);  // 参照を渡す
// ここでも my_list は使える！
```

関連用語: borrow, ownership, mutable, immutable, reference
再発防止メモ: 読み取り専用なら `&T`、変更するなら `&mut T`、所有権を渡すなら `T`

---

### 質問: let, let mut, const の違い

日付: 2025-10-05
カテゴリ: [Rust]
Q: `let mut` と `let` と `const` の使い分けは？
A: `let` は不変変数（再代入不可）、`let mut` は可変変数（再代入可能）、`const` はコンパイル時定数（型注釈必須、実行時計算不可、グローバルスコープ可）。Rust はデフォルト不変で安全性優先。
コード例:

```rust
let x = 5;           // 不変
let mut y = 10;      // 可変
const MAX: u32 = 100; // 定数（型注釈必須）

y += 1;  // OK
x += 1;  // エラー！

// シャドーイング（let で型変更可能）
let spaces = "   ";
let spaces = spaces.len();  // OK（新しい変数）
```

関連用語: immutable, mutable, shadowing, compile-time constant
再発防止メモ: 基本は `let`、変更するなら `mut`、設定値なら `const`

---

### 質問: .map_err() と ? 演算子の仕組み

日付: 2025-10-05
カテゴリ: [Rust]
Q: `.map_err()` と `?` 演算子はどう動く？エラー処理の流れは？
A: `Result<T, E>` の `E`（エラー型）を別の型に変換するのが `.map_err()`。`?` は `Err` なら即座に return する糖衣構文。`io::Error` を `String` に変換して統一したエラー型にするために使う。
コード例:

```rust
// .map_err() でエラー型変換
let content = fs::read_to_string(path)
    .map_err(|e| format!("読み込みエラー: {}", e))?;
//           ^^^ io::Error → String に変換
//                                                  ^ Err なら return

// ? の展開イメージ
match fs::read_to_string(path) {
    Ok(content) => content,
    Err(e) => return Err(format!("エラー: {}", e)),
}
```

関連用語: Result, error propagation, early return, type conversion
再発防止メモ: `?` は `Result` を返す関数でのみ使える。複数の `?` を連鎖できる

---

### 質問: .collect() の役割

日付: 2025-10-05
カテゴリ: [Rust]
Q: `.collect()` は何をしている？
A: イテレータの要素を集めて `Vec`, `HashSet`, `String` などのコレクションに変換する。`.map()` は遅延評価（lazy）なので、`.collect()` で初めて実際に処理が実行される。型推論が賢く、戻り値の型から自動判断。
コード例:

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers
    .iter()                    // イテレータに変換
    .map(|n| n * 2)           // 各要素を変換（まだ実行されない）
    .collect();                // Vec に集める（ここで実行）
// doubled = [2, 4, 6]
```

関連用語: iterator, lazy evaluation, type inference, functional programming
再発防止メモ: `.map()` だけでは実行されない、`.collect()` で初めて動く

---

### 質問: format! と &format!() の違い

日付: 2025-10-05
カテゴリ: [Rust]
Q: `format!()` と `&format!()` の使い分けは？
A: `format!()` は `String` を返す。`&format!()` は `&String` への参照だが、寿命が短い（式の終わりで消える）ので危険。`push_str()` など `&str` を受け取る関数には `&format!(...)` が使えるが、基本は `String` を返す方が安全。
コード例:

```rust
// String を返す
let message = format!("Hello, {}!", name);

// &str を受け取る関数に渡す
output.push_str(&format!("[id:{}] {}\n", id, content));
//               ^ 一時的な参照（この行だけ有効）

// これはエラー（参照の寿命が短い）
let temp = &format!("test");  // 危険！
```

関連用語: String, &str, lifetime, temporary value
再発防止メモ: 基本は `format!()` で `String` を返す、`&` は必要な時だけ

---

### 質問: todo!() マクロの使い方

日付: 2025-10-05
カテゴリ: [Rust]
Q: 関数の最後にある `todo!()` は何？
A: 「まだ実装していない」をマークするマクロ。コンパイルは通る（型チェックは OK）が、実行するとパニック。関数の雛形を作る時に便利。実装完了後は `Ok(())` などに置き換える。
コード例:

```rust
fn handle_add(args: &[String]) -> Result<(), String> {
    // ... 色々処理 ...
    todo!()  // ← 実装中のマーカー
}

// 実装完了後
fn handle_add(args: &[String]) -> Result<(), String> {
    // ... 処理完了 ...
    Ok(())  // ← todo!() を削除して正しい戻り値
}
```

関連用語: unimplemented!, unreachable!, panic!
再発防止メモ: `todo!()` は開発中のマーカー、実装したら削除

---
