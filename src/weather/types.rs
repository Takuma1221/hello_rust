use serde::{Deserialize, Serialize};

/// OpenWeatherMap API からのレスポンス情報
#[derive(Debug, Deserialize)]
pub struct WeatherInfo {
    pub city: String,
    pub weather: String,  // 天気の説明 ("晴れ", "曇り", etc.)
    pub temperature: f64, // 気温（摂氏）
    pub humidity: u32,    // 湿度 (%)
}

/// 保存する天気メモ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedMemo {
    pub timestamp: String, // ISO8601 形式の日時
    pub city: String,      // 都市名
    pub weather: String,   // 天気
    pub temperature: f64,  // 気温
    pub note: String,      // ユーザーのメモ
}

/// メモ全体を格納する構造
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoStorage {
    pub memos: Vec<SavedMemo>,
}

impl MemoStorage {
    /// 空のストレージを作成
    pub fn new() -> Self {
        Self { memos: Vec::new() }
    }

    /// メモを追加（新しい順に並べる）
    pub fn add_memo(&mut self, memo: SavedMemo) {
        self.memos.insert(0, memo); // 先頭に追加
    }

    /// 特定都市のメモを検索
    pub fn search_by_city(&self, city: &str) -> Vec<&SavedMemo> {
        self.memos
            .iter()
            .filter(|m| m.city.contains(city))
            .collect()
    }
}

// ツール用の引数型

/// fetch_weather ツール用引数
#[derive(Debug, Deserialize)]
pub struct FetchWeatherArgs {
    pub city: String,
}

/// save_weather_memo ツール用引数
#[derive(Debug, Deserialize)]
pub struct SaveMemoArgs {
    pub city: String,
    pub weather: String,
    pub temperature: f64,
    pub note: String,
}

/// search_memos ツール用引数
#[derive(Debug, Deserialize)]
pub struct SearchMemosArgs {
    pub city: String,
}
