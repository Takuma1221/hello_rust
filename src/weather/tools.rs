use chrono::Local;

use crate::weather::api::{fetch_weather as api_fetch_weather, format_weather_info};
use crate::weather::storage::{add_and_save_memo, format_all_memos, format_memos_by_city};
use crate::weather::types::{FetchWeatherArgs, SaveMemoArgs, SavedMemo, SearchMemosArgs};

/// ツール1: 天気を取得
/// 
/// OpenWeatherMap API を呼び出して天気情報を取得し、
/// 整形した文字列を返す
pub async fn tool_fetch_weather(args: FetchWeatherArgs) -> Result<String, String> {
    println!("🔧 ツール実行: fetch_weather(city=\"{}\")", args.city);

    let weather_info = api_fetch_weather(&args.city).await?;
    let formatted = format_weather_info(&weather_info);

    Ok(formatted)
}

/// ツール2: 天気メモを保存
/// 
/// 天気情報とユーザーのメモを JSON ファイルに保存
pub async fn tool_save_weather_memo(args: SaveMemoArgs) -> Result<String, String> {
    println!(
        "🔧 ツール実行: save_weather_memo(city=\"{}\", weather=\"{}\", temp={}, note=\"{}\")",
        args.city, args.weather, args.temperature, args.note
    );

    // 現在時刻を取得（ISO8601 形式）
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let memo = SavedMemo {
        timestamp,
        city: args.city,
        weather: args.weather,
        temperature: args.temperature,
        note: args.note,
    };

    add_and_save_memo(memo)
}

/// ツール3: 全メモを一覧表示
/// 
/// 保存されている全ての天気メモを新しい順に表示
pub async fn tool_list_memos() -> Result<String, String> {
    println!("🔧 ツール実行: list_memos()");

    format_all_memos()
}

/// ツール4: 特定都市のメモを検索
/// 
/// 指定された都市名を含むメモだけを抽出して表示
pub async fn tool_search_memos(args: SearchMemosArgs) -> Result<String, String> {
    println!("🔧 ツール実行: search_memos(city=\"{}\")", args.city);

    format_memos_by_city(&args.city)
}
