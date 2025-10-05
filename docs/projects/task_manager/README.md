# タスク管理 AI アシスタント - プロジェクト仕様書

## 📋 プロジェクト概要

AI に自然な会話でタスクを管理してもらうアプリケーション。
「明日までにレポート書かないと」と話しかけると、自動でタスクを追加してくれる。

## 🎯 目標

- AI 関数呼び出しの実践
- 日付パースの実装
- タスク管理ロジックの設計
- **ヒント少なめで自力実装**

---

## 💬 使用例

### 会話例 1: タスク追加

```
User: "明日までにレポート提出しないと"
AI: → add_task("レポート提出", "2025-10-06", "high") を呼び出し
AI: "タスクを追加しました：レポート提出（期限: 2025-10-06、優先度: 高）"

User: "来週月曜に歯医者の予約"
AI: → add_task("歯医者の予約", "2025-10-14", "medium") を呼び出し
AI: "タスクを追加しました：歯医者の予約（期限: 2025-10-14）"
```

### 会話例 2: タスク一覧

```
User: "今日やることリストアップして"
AI: → list_tasks("today") を呼び出し
AI: "今日のタスクは2件です：
     1. [高] レポート提出（期限: 今日）
     2. [中] 買い物（期限: 今日）"

User: "今週のタスク全部見せて"
AI: → list_tasks("week") を呼び出し
AI: "今週のタスクは5件です：..."
```

### 会話例 3: タスク完了

```
User: "レポート終わった！"
AI: → complete_task("レポート提出") を呼び出し
AI: "お疲れ様です！「レポート提出」を完了にしました ✓"
```

### 会話例 4: タスク削除

```
User: "歯医者の予約キャンセルになった"
AI: → delete_task("歯医者の予約") を呼び出し
AI: "「歯医者の予約」を削除しました"
```

---

## 🛠️ 実装する機能

### 必須機能（Phase 1）

- ✅ タスク追加（タイトル、期限、優先度）
- ✅ タスク一覧表示（今日、今週、全て）
- ✅ タスク完了
- ✅ タスク削除
- ✅ JSON ファイルへの永続化

### 拡張機能（Phase 2 - オプション）

- ⭐ タスク検索（キーワード）
- ⭐ 期限切れアラート
- ⭐ タグ機能（#仕事、#プライベート）
- ⭐ 統計情報（完了率など）

---

## 📊 データ構造

### Task 構造体

```rust
struct Task {
    id: u32,
    title: String,
    due_date: String,        // "2025-10-06" 形式
    priority: Priority,      // High, Medium, Low
    completed: bool,
    created_at: String,
}
```

### Priority 列挙型

```rust
enum Priority {
    High,
    Medium,
    Low,
}
```

### TaskList 構造体

```rust
struct TaskList {
    tasks: Vec<Task>,
}
```

---

## 🔧 実装する関数（AI が呼び出す）

### 1. `add_task`

タスクを追加する

**引数**:

- `title`: String - タスクのタイトル
- `due_date`: String - 期限（"2025-10-06" 形式）
- `priority`: String - 優先度（"high", "medium", "low"）

**戻り値**: 成功メッセージ

---

### 2. `list_tasks`

タスク一覧を取得

**引数**:

- `filter`: String - フィルター（"today", "week", "all"）

**戻り値**: タスクリストの文字列

---

### 3. `complete_task`

タスクを完了にする

**引数**:

- `title`: String - タスクのタイトル（または ID）

**戻り値**: 成功メッセージ

---

### 4. `delete_task`

タスクを削除する

**引数**:

- `title`: String - タスクのタイトル（または ID）

**戻り値**: 成功メッセージ

---

## 💡 実装のヒント（最小限）

### ヒント 1: 日付の扱い

- `chrono` クレートを使う
- `Local::now()` で今日の日付
- 文字列パースは `.parse::<NaiveDate>()`

### ヒント 2: フィルター

- "today": 期限が今日のタスク
- "week": 期限が今週のタスク
- "all": 全タスク

### ヒント 3: タスクの検索

- タイトルで検索（部分一致で OK）
- `.find()` や `.iter().find()` を使う

### ヒント 4: ファイル構成（推奨）

```
src/task_manager/
├── mod.rs           # モジュール定義
├── types.rs         # Task, TaskList, Priority の定義
├── storage.rs       # ファイル I/O
├── tools.rs         # AI が呼び出す関数
└── play.rs          # メインループ
```

---

## 🎓 学習目標

このプロジェクトで学べること：

1. **AI 関数呼び出しの実践** - 複数ツールの設計
2. **日付操作** - chrono の使い方
3. **検索・フィルタリング** - イテレータの活用
4. **エラーハンドリング** - タスクが見つからない場合など
5. **自力実装** - ヒント少なめで考える力

---

## 🚀 開発の進め方

### Step 1: 型定義（types.rs）

- `Task`, `TaskList`, `Priority` を定義
- メソッド実装（`add_task`, `find_task`, `complete_task` など）

### Step 2: ストレージ（storage.rs）

- JSON ファイルへの保存・読み込み
- `data/tasks.json` に保存

### Step 3: ツール定義（tools.rs）

- AI が呼び出す 4 つの関数を実装
- OpenAI の関数定義 JSON を作成

### Step 4: メインループ（play.rs）

- OpenAI API との連携
- 会話ループの実装

### Step 5: テスト

- ユニットテスト追加
- 動作確認

---

## 📝 実装チェックリスト

### Phase 1: 基本機能

- [ ] types.rs: Task, TaskList, Priority 定義
- [ ] types.rs: add_task メソッド実装
- [ ] types.rs: find_task メソッド実装
- [ ] types.rs: complete_task メソッド実装
- [ ] types.rs: delete_task メソッド実装
- [ ] storage.rs: load_tasks 実装
- [ ] storage.rs: save_tasks 実装
- [ ] tools.rs: tool_add_task 実装
- [ ] tools.rs: tool_list_tasks 実装
- [ ] tools.rs: tool_complete_task 実装
- [ ] tools.rs: tool_delete_task 実装
- [ ] play.rs: メインループ実装
- [ ] 動作確認

### Phase 2: テスト（オプション）

- [ ] types.rs にテスト追加
- [ ] storage.rs にテスト追加
- [ ] 全テスト実行

### Phase 3: 拡張機能（オプション）

- [ ] タグ機能
- [ ] 検索機能
- [ ] 統計情報

---

## 🎯 成功の定義

以下の会話が動作すれば OK！

```
User: "明日までにレポート書く"
AI: "タスクを追加しました：レポート書く（期限: 明日）"

User: "今日のタスク教えて"
AI: "今日のタスクは1件です：..."

User: "レポート終わった"
AI: "完了にしました ✓"
```

---

## 📚 参考

- Weather アプリ（`src/weather/`）の構造を参考に
- Memo アプリ（`src/memo/`）のストレージ実装を参考に
- Function Calling の基本は Stage1 を参考に

---

**準備ができたら `src/task_manager/` を作成して始めましょう！** 🚀

質問があればいつでも聞いてください（でも、なるべく自分で考えてみてね！）
