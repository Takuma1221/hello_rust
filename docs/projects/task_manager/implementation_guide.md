# タスク管理 AI アシスタント - 実装ガイド（最小限のヒント）

## 📝 TODO リスト

### Phase 1: 型定義（types.rs）

#### Step 1: Priority 列挙型

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

#### Step 2: Task 構造体

必要なフィールド:

- `id`, `title`, `due_date`, `priority`, `completed`, `created_at`

#### Step 3: TaskList 構造体

- `tasks: Vec<Task>`

#### Step 4: メソッド実装

- `new()` - 空のリスト作成
- `add_task()` - 新しいタスク追加
- `find_task()` - タイトルでタスク検索
- `complete_task()` - タスクを完了にする
- `delete_task()` - タスクを削除
- `list_by_filter()` - フィルターに応じたタスク一覧

---

### Phase 2: ストレージ（storage.rs）

#### 必要な関数

1. `load_tasks()` - JSON ファイルから読み込み
2. `save_tasks()` - JSON ファイルに保存

**参考**: `memo/storage.rs` とほぼ同じ

---

### Phase 3: ツール定義（tools.rs）

#### 実装する関数

1. `tool_add_task(args)` - タスク追加
2. `tool_list_tasks(args)` - タスク一覧
3. `tool_complete_task(args)` - タスク完了
4. `tool_delete_task(args)` - タスク削除
5. `get_tool_definitions()` - ツール定義 JSON

**参考**: `weather/tools.rs` の構造

---

### Phase 4: メインループ（play.rs）

**参考**: `weather/play.rs` をベースに実装

#### 必要な処理

1. ユーザー入力
2. OpenAI API 呼び出し
3. tool_calls の処理
4. 結果表示

---

## 🔍 デバッグのヒント

### コンパイルエラーが出たら

1. 型が合っているか確認
2. `pub` キーワードが必要か確認
3. `use` で必要なモジュールをインポートしているか確認

### 実行時エラーが出たら

1. JSON のパースエラー → `println!("{:?}", args)` でデバッグ
2. ファイルが見つからない → `data/` ディレクトリを作成
3. API エラー → `.env` ファイルに API キーがあるか確認

---

## 💡 つまずいたら

### 質問の仕方（例）

- ❌ 「動かない」
- ✅ 「add_task で〇〇のエラーが出る。こう書いたけど何が違う？」

### ヒントをもらう前に

1. エラーメッセージを読む
2. 似たコード（weather, memo）を参考にする
3. 型を確認する

---

**自力で頑張ってみてください！できたら見せてください 😊**
