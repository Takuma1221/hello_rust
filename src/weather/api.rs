use reqwest::Client;
use serde::Deserialize;
use std::env;

use crate::weather::types::WeatherInfo;

/// OpenWeatherMap API のレスポンス構造（必要な部分のみ）
#[derive(Debug, Deserialize)]
struct OpenWeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

/// 都市名から天気情報を取得
/// 
/// # Arguments
/// * `city` - 都市名（日本語または英語）
/// 
/// # Returns
/// * `Ok(WeatherInfo)` - 天気情報
/// * `Err(String)` - エラーメッセージ
pub async fn fetch_weather(city: &str) -> Result<WeatherInfo, String> {
    let api_key = env::var("OPENWEATHER_API_KEY")
        .map_err(|_| "環境変数 OPENWEATHER_API_KEY が設定されていません".to_string())?;

    let client = Client::new();
    let url = "https://api.openweathermap.org/data/2.5/weather";

    // リクエストパラメータ
    let response = client
        .get(url)
        .query(&[
            ("q", city),
            ("appid", &api_key),
            ("units", "metric"), // 摂氏
            ("lang", "ja"),      // 日本語の説明
        ])
        .send()
        .await
        .map_err(|e| format!("API リクエストエラー: {}", e))?;

    // ステータスコード確認
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!(
            "API エラー（ステータス: {}）: {}",
            status, error_text
        ));
    }

    // JSON パース
    let data: OpenWeatherResponse = response
        .json()
        .await
        .map_err(|e| format!("JSON パースエラー: {}", e))?;

    // WeatherInfo に変換
    let weather_desc = data
        .weather
        .first()
        .map(|w| w.description.clone())
        .unwrap_or_else(|| "不明".to_string());

    Ok(WeatherInfo {
        city: data.name,
        weather: weather_desc,
        temperature: data.main.temp,
        humidity: data.main.humidity,
    })
}

/// 天気情報を整形して表示用文字列に
pub fn format_weather_info(info: &WeatherInfo) -> String {
    format!(
        "🌤️ {} の天気: {}, 気温 {}℃, 湿度 {}%",
        info.city, info.weather, info.temperature, info.humidity
    )
}
