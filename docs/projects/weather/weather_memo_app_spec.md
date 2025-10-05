# 天気予報メモアプリ 仕様書

## 🎯 プロジェクト概要

**名前**: WeatherMemo  
**目的**: 自然言語で天気を聞いて、記録を JSON ファイルに保存する CLI アプリ  
**推定時間**: 2-3 時間  
**学習目標**:

- 外部 Web API 連携（OpenWeatherMap）
- ファイル I/O とデータ永続化
- 複数の引数型を扱うツール設計

---

## 📋 機能一覧

### 1. 天気取得（API 連携）

```
ユーザー: 東京の天気を教えて
AI: 実行: fetch_weather("東京")
    東京の天気: 晴れ、気温 22℃、湿度 60%
```

### 2. 天気メモ保存（ファイル I/O）

```
ユーザー: 今の東京の天気をメモして
AI: 実行: fetch_weather("東京")
    実行: save_weather_memo("東京", "晴れ", 22.0, "散歩日和")
    メモを保存しました！
```

### 3. メモ一覧表示

```
ユーザー: 過去のメモを見せて
AI: 実行: list_memos()
    【保存済みメモ】
    1. 2025-10-05 14:30 | 東京 | 晴れ 22℃ | 散歩日和
    2. 2025-10-04 09:15 | 大阪 | 曇り 19℃ | 肌寒い
```

### 4. 特定都市のメモ検索

```
ユーザー: 東京のメモだけ見せて
AI: 実行: search_memos("東京")
    【東京のメモ】
    1. 2025-10-05 14:30 | 晴れ 22℃ | 散歩日和
```

---

## 🛠️ 使用するツール関数

### ツール 1: `fetch_weather`

```rust
async fn fetch_weather(city: String) -> Result<WeatherInfo, Error>
```

- **役割**: OpenWeatherMap API で現在の天気取得
- **引数**: `city` (都市名: 日本語 or 英語)
- **返値**: 天気・気温・湿度

### ツール 2: `save_weather_memo`

```rust
fn save_weather_memo(
    city: String,
    weather: String,
    temperature: f64,
    note: String
) -> Result<String, Error>
```

- **役割**: 天気情報 + ユーザーメモを JSON に保存
- **引数**: 都市名、天気、気温、メモ
- **返値**: 成功メッセージ

### ツール 3: `list_memos`

```rust
fn list_memos() -> Result<String, Error>
```

- **役割**: 全メモを新しい順に表示
- **引数**: なし
- **返値**: 整形済みテキスト

### ツール 4: `search_memos`

```rust
fn search_memos(city: String) -> Result<String, Error>
```

- **役割**: 特定都市のメモだけ抽出
- **引数**: `city` (都市名)
- **返値**: 整形済みテキスト

---

## 📁 ファイル構成

```
src/
├── main.rs                      # エントリーポイント
├── function_call/               # 既存の計算ツール
│   ├── api.rs
│   ├── errors.rs
│   ├── play.rs
│   ├── tools.rs
│   └── types.rs
└── weather/                     # 新規: 天気アプリ
    ├── mod.rs                   # モジュール宣言
    ├── types.rs                 # データ型定義
    ├── api.rs                   # OpenWeatherMap 連携
    ├── storage.rs               # JSON ファイル I/O
    ├── tools.rs                 # ツール実装
    └── play.rs                  # メインループ

data/
└── weather_memos.json           # 保存データ（自動生成）

.env
OPENAI_API_KEY=sk-...
OPENWEATHER_API_KEY=...          # 新規追加
```

---

## 🗂️ データ構造

### WeatherInfo（API レスポンス）

```rust
#[derive(Debug, Deserialize)]
pub struct WeatherInfo {
    pub city: String,
    pub weather: String,      // "晴れ", "曇り", etc.
    pub temperature: f64,     // 摂氏
    pub humidity: u32,        // 湿度 %
}
```

### SavedMemo（保存用）

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SavedMemo {
    pub timestamp: String,    // ISO8601 形式
    pub city: String,
    pub weather: String,
    pub temperature: f64,
    pub note: String,         // ユーザーのメモ
}
```

### MemoStorage（ファイル全体）

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoStorage {
    pub memos: Vec<SavedMemo>,
}
```

---

## 🔑 必要な準備

### 1. OpenWeatherMap API キー取得

1. https://openweathermap.org/ でアカウント作成（無料）
2. API Keys から取得
3. `.env` に追加:

```bash
OPENWEATHER_API_KEY=your_key_here
```

### 2. 依存クレート追加

```toml
[dependencies]
# 既存
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dotenv = "0.15"

# 新規追加
chrono = { version = "0.4", features = ["serde"] }  # 日時処理
```

---

## 🚀 実装ステップ

### Phase 1: データ型定義（15 分）

- `weather/types.rs` で構造体定義
- serde の Serialize/Deserialize 導入

### Phase 2: ストレージ実装（30 分）

- `weather/storage.rs` で JSON 読み書き
- ファイルがない場合の初期化処理

### Phase 3: 天気 API 実装（30 分）

- `weather/api.rs` で OpenWeatherMap 呼び出し
- エラーハンドリング（都市名不明、API エラー）

### Phase 4: ツール実装（45 分）

- `weather/tools.rs` で 4 つのツール関数
- 引数パース用の構造体定義

### Phase 5: メインループ（30 分）

- `weather/play.rs` で OpenAI 連携
- ツール schema 定義
- ループ処理

### Phase 6: テスト（15 分）

- 各機能の動作確認
- エラーケースの確認

---

## 📝 サンプル会話フロー

```
$ cargo run --bin weather_app

天気アプリを起動しました！何をしましょうか？
> 東京の今日の天気は？

実行: fetch_weather("東京")
東京の天気: 晴れ、気温 22.5℃、湿度 58%

> これをメモして「公園散歩に最適」って追加で

実行: save_weather_memo("東京", "晴れ", 22.5, "公園散歩に最適")
メモを保存しました！

> メモ一覧を見せて

実行: list_memos()
【保存済みメモ】
1. 2025-10-05 15:30 | 東京 | 晴れ 22.5℃ | 公園散歩に最適
```

---

## 🎓 学習ポイント

### Web API 連携

- `reqwest::get()` で HTTP リクエスト
- クエリパラメータの構築
- JSON レスポンスのデシリアライズ

### ファイル I/O

- `std::fs::read_to_string()` / `write()`
- ファイル存在確認 (`Path::exists()`)
- ディレクトリ作成 (`create_dir_all()`)

### データ永続化

- serde_json で構造体 ↔ JSON
- 追記ロジック（既存配列に push）
- タイムスタンプ生成（chrono）

### 複数引数型

- String, f64, u32 を混在
- 各ツール用の引数構造体設計
- enum で複数パターン対応

---

## 🔧 拡張アイデア（余裕があれば）

- [ ] 週間予報取得
- [ ] グラフ化（気温の推移）
- [ ] 通知機能（雨なら傘提案）
- [ ] 他の都市と比較
- [ ] CSV エクスポート

---

準備ができたら実装を開始しましょう！👨‍💻
