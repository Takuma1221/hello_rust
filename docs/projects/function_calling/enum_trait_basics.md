# Rust 基礎メモ: enum / バリアント / トレイト / エラー型 入門

(最初にざっくり) "enum は値の種類(ケース)を列挙した型"、"バリアントはその個々のケース"、"トレイトは振る舞い(インターフェイス)の契約" です。

---

## 1. enum とは

複数パターンのうち **どれか 1 つ** を保持する型。

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

- `TrafficLight` が型名
- `Red` / `Yellow` / `Green` が “バリアント” (variant)
- どれか 1 つしか同時に入らない

### 1.1 値を持つバリアント

```rust
enum Shape {
    Circle(f64),          // 半径
    Rectangle { w: f64, h: f64 },
    Unit,                 // 何も持たない
}
```

- タプル風 / 構造体風 どちらも書ける
- `Shape::Circle(3.0)` のように使う

### 1.2 なぜ enum を使う？

- "状態を安全に列挙" → 不正状態をコンパイル時に排除
- match で網羅性チェック (書き忘れがあれば警告 or エラー)
- Option / Result など標準ライブラリの多くが enum

---

## 2. バリアント (Variant)

“enum の中に定義された個々のケース”。
例: `Result<T, E>` のバリアントは `Ok(T)` と `Err(E)`。

```rust
let r: Result<i32, &str> = Ok(10);
match r {
    Ok(v) => println!("成功: {v}"),
    Err(e) => println!("失敗: {e}"),
}
```

---

## 3. match とパターンマッチ

enum を取り出す基本手段。

```rust
fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14159 * r * r,
        Shape::Rectangle { w, h } => w * h,
        Shape::Unit => 0.0,
    }
}
```

- すべてのバリアントを列挙 → コンパイラが安全性チェック
- 一部だけ扱いたい場合は `_ => { ... }` で「その他」

---

## 4. トレイト (Trait)

“この振る舞いを持つ” という共通インターフェイスの定義。

```rust
trait Animal {
    fn speak(&self) -> String; // メソッドシグネチャ
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> String { "wan".into() }
}
```

- `impl TraitName for Type` で実装
- ジェネリック境界や &dyn Trait で多態性 (ポリモーフィズム)

### 4.1 よく使う標準トレイト

| トレイト           | 役割                    |
| ------------------ | ----------------------- |
| `Debug`            | `{:?}` でデバッグ表示   |
| `Display`          | `{}` で人向け表示       |
| `Clone`            | 値の複製                |
| `Copy`             | ビットコピー (小さい値) |
| `PartialEq` / `Eq` | 比較(==)                |
| `Hash`             | HashMap のキーに必要    |
| `Error`            | エラーオブジェクト表現  |

---

## 5. Display / Debug の違い

```rust
#[derive(Debug)]
struct User { id: u32, name: String }

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

let u = User { id: 1, name: "Alice".into() };
println!("{:?}", u); // Debug → User { id: 1, name: "Alice" }
println!("{}", u);   // Display → Alice(1)
```

- Debug: 構造そのまま (開発者向け)
- Display: ユーザーに見せる整形済みテキスト

---

## 6. Error トレイト

エラー型に統一インターフェイスを与える。要件:

1. `Debug` 実装 (たいてい `#[derive(Debug)]`)
2. `Display` 実装 (メッセージ)
3. `impl std::error::Error for 型名 {}`

これで `Result<T, MyErr>` や `Box<dyn Error>` に格納可能。

### 6.1 例: 手書き

```rust
#[derive(Debug)]
enum MyErr { NotFound, Parse(String) }
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::NotFound => write!(f, "not found"),
            MyErr::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}
impl std::error::Error for MyErr {}
```

### 6.2 thiserror で簡略化

```toml
# Cargo.toml
thiserror = "1"
```

```rust
use thiserror::Error;
#[derive(Debug, Error)]
enum MyErr2 {
    #[error("not found")] NotFound,
    #[error("parse error: {0}")] Parse(String),
}
```

---

## 7. Option vs Result vs カスタム Outcome

| 型                              | 目的             | 失敗理由       | 追加情報             |
| ------------------------------- | ---------------- | -------------- | -------------------- |
| Option<T>                       | 値がある/ない    | 理由は持たない | なし                 |
| Result<T,E>                     | 成功/失敗        | E に理由       | あり                 |
| 独自 enum (例: ToolCallOutcome) | ドメイン特化状態 | 状態ごと       | 状態ごとにフィールド |

今回: ツール無し / 実行成功(名前と結果) を区別 → `ToolCallOutcome` が自然。

---

## 8. エラー設計ミニ指針

- まずは単純: `enum Error { KindA, KindB(String) }`
- 増えて管理が辛くなったら thiserror 導入
- ライブラリ公開を意識 → 具体エラー型を公開しすぎない (将来互換性)
- アプリ内部だけ → `anyhow` で一括ラップも可

---

## 9. よくある初学者ハマりポイント

| 症状                                      | 原因                       | 対策                                 |
| ----------------------------------------- | -------------------------- | ------------------------------------ |
| `the trait Error is not implemented`      | Debug/Display 未実装       | derive + 手書き Display              |
| `mismatched types` for match              | すべてのバリアント列挙不足 | `_ =>` か網羅的に書く                |
| `no method named get` on HashMap key &str | 所有権/参照違い            | `get("key")` は &str で OK。型再確認 |
| `cannot move out`                         | enum の内部値 move         | 参照にするか `clone()`               |

---

## 10. 現在のコードでの適用点

- `ToolCallOutcome` = Option では説明力不足を解消する enum
- `ToolCallError` = ドメインエラー (空 / 未知ツール / 引数パース)
- `OpenAiCallError` = 通信層エラー (今後バリアント精緻化可能)
- Display 実装でログ行が読みやすい日本語化

---

## 11. もう少し進むなら

1. `OpenAiCallError` を thiserror 化
2. `ToolCallError` に元 serde_json::Error を保持 (`#[source]`) してチェーン表示
3. 2 回目 API 呼び出し (ツール結果を tool メッセージ化 → 最終回答)
4. `ToolCallOutcome::Executed` を enum ToolResult で汎用化 (Number / Text / Json など)

---

## 12. 練習課題

1. `Shape` に Triangle{base,height} を追加し面積計算に反映
2. `MyErr` に Io(std::io::Error) 追加し Display 実装
3. `Option<T>` を `Result<T, MyErr>` にリファクタして失敗理由を表示
4. `ToolCallOutcome` に `ParseOnly{ args: SumArgs }` を追加 (実行はまだしない状態を想定) して match を拡張

---

## 13. 参考 (公式)

- The Rust Book: Enums, Traits, Error Handling
- anyhow / thiserror クレート README

---

(以上) 初心者視点で「まずこれを押さえると enum / trait / error の輪郭が掴める」要素を凝縮しました。次は課題に挑戦してみてください。
