# 天気メモアプリ セットアップガイド

## 📝 OpenWeatherMap API キー取得手順

### 1. アカウント作成

1. https://openweathermap.org/ にアクセス
2. 右上の「Sign In」→「Create an Account」
3. メールアドレス、ユーザー名、パスワードを入力して登録
4. 登録したメールに確認メールが届くので認証

### 2. API キー取得

1. ログイン後、右上のユーザー名をクリック →「My API keys」
2. デフォルトで 1 つキーが生成済み（または「Create Key」で新規作成）
3. API キーをコピー（例: `a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6`）

### 3. .env ファイルに追加

`/Users/aokitakuma/workspace/hello_rust/.env` を開いて、以下を追加:

```bash
OPENAI_API_KEY=sk-your-openai-key-here
OPENWEATHER_API_KEY=your-openweathermap-key-here  # ← これを追加
```

**例:**

```bash
OPENAI_API_KEY=sk-proj-abc123...
OPENWEATHER_API_KEY=a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6
```

---

## 🚀 実行方法

### 基本実行

```bash
cargo run
```

### 動作確認用サンプル入力

#### 1. 天気取得だけ

```
東京の天気を教えて
```

#### 2. 天気取得 + メモ保存

```
大阪の天気を調べて、「明日の出張に備える」ってメモして
```

#### 3. メモ一覧表示

```
保存したメモを全部見せて
```

#### 4. 特定都市のメモ検索

```
東京のメモだけ表示して
```

---

## 📂 保存データの場所

メモは以下のファイルに JSON 形式で保存されます:

```
/Users/aokitakuma/workspace/hello_rust/data/weather_memos.json
```

中身の例:

```json
{
  "memos": [
    {
      "timestamp": "2025-10-05 15:30:00",
      "city": "Tokyo",
      "weather": "晴れ",
      "temperature": 22.5,
      "note": "公園散歩に最適"
    }
  ]
}
```

---

## 🔧 トラブルシューティング

### エラー: "環境変数 OPENWEATHER_API_KEY が設定されていません"

→ `.env` ファイルに API キーを追加してください

### エラー: "API エラー（ステータス: 401）"

→ API キーが間違っているか無効です。再確認してください

### エラー: "API エラー（ステータス: 404）"

→ 都市名が見つかりません。英語名を試してください（例: Tokyo, Osaka）

### 天気が取得できない

→ 無料プランでは 1 分に 60 回までのリクエスト制限があります

---

## 🎓 学習ポイント

このプロジェクトで学べたこと:

- ✅ 外部 Web API の呼び出し（reqwest + async/await）
- ✅ JSON ファイルの読み書き（serde_json）
- ✅ 複数の引数型を持つツール設計
- ✅ エラーハンドリング（Result 型の連鎖）
- ✅ 日時処理（chrono）
- ✅ モジュール分割の実践

---

## 🚀 次のステップ

余裕があれば以下の拡張に挑戦:

1. **週間予報取得**

   - OpenWeatherMap の別エンドポイント利用
   - 5 日間の予報を一覧表示

2. **グラフ化**

   - `plotters` クレートで気温推移をグラフ化
   - PNG 画像として保存

3. **通知機能**

   - 「明日雨なら傘を忘れずに」と通知
   - cron で定期実行

4. **複数都市比較**

   - 同時に複数都市の天気を取得
   - 一番暖かい/涼しい都市を提案

5. **CSV エクスポート**
   - メモを CSV 形式でエクスポート
   - Excel で分析可能に

---

楽しんでください！🌤️
