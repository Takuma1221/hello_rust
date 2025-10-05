# Weather メモアプリ

OpenWeatherMap API と連携した天気メモアプリ。

## 📚 概要

- OpenWeatherMap API で天気情報を取得
- 天気情報付きのメモを保存
- 保存したメモの一覧表示・検索

## 📁 ドキュメント

- **`weather_memo_app_spec.md`** - アプリの仕様書
- **`weather_setup_guide.md`** - セットアップ手順（API キー取得など）

## 🎯 実装済み機能

- ✅ 天気情報取得（都市名指定）
- ✅ 天気メモの保存（JSON 形式）
- ✅ メモ一覧表示
- ✅ 都市名で検索
- ✅ OpenAI Function Calling 統合

## 📂 関連コード

```
src/weather/
├── mod.rs          # モジュール定義
├── types.rs        # 型定義（WeatherInfo, SavedMemo, MemoStorage）
├── api.rs          # OpenWeatherMap API 連携
├── storage.rs      # ファイル I/O（JSON 保存・読み込み）
├── tools.rs        # Function Calling ツール定義
└── play.rs         # メインループ
```

## 🚀 実行方法

```bash
# .env ファイルに API キーを設定
OPENWEATHER_API_KEY=your_api_key_here

# main.rs で weather::play::play() のコメントアウトを解除
cargo run
```

## 🗂️ データ保存先

```
data/weather_memos.json
```

## 📖 学習した概念

- HTTP リクエスト（reqwest）
- JSON シリアライズ・デシリアライズ（serde）
- ファイル I/O（std::fs）
- OpenAI Function Calling の実践
- 環境変数の扱い（dotenv）
