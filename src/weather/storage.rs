use std::fs;
use std::path::Path;

use crate::weather::types::{MemoStorage, SavedMemo};

/// メモファイルのデフォルトパス
const MEMO_FILE: &str = "data/weather_memos.json";

/// メモをファイルから読み込む
/// ファイルが存在しない場合は空のストレージを返す
pub fn load_memos() -> Result<MemoStorage, String> {
    let path = Path::new(MEMO_FILE);

    // ファイルが存在しない場合は空を返す
    if !path.exists() {
        return Ok(MemoStorage::new());
    }

    // ファイル読み込み
    let content = fs::read_to_string(path)
        .map_err(|e| format!("ファイル読み込みエラー: {}", e))?;

    // JSON パース
    let storage: MemoStorage = serde_json::from_str(&content)
        .map_err(|e| format!("JSON パースエラー: {}", e))?;

    Ok(storage)
}

/// メモをファイルに保存
pub fn save_memos(storage: &MemoStorage) -> Result<(), String> {
    let path = Path::new(MEMO_FILE);

    // data ディレクトリがなければ作成
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("ディレクトリ作成エラー: {}", e))?;
    }

    // JSON シリアライズ（整形あり）
    let json = serde_json::to_string_pretty(storage)
        .map_err(|e| format!("JSON シリアライズエラー: {}", e))?;

    // ファイル書き込み
    fs::write(path, json)
        .map_err(|e| format!("ファイル書き込みエラー: {}", e))?;

    Ok(())
}

/// 新しいメモを追加して保存
pub fn add_and_save_memo(memo: SavedMemo) -> Result<String, String> {
    let mut storage = load_memos()?;
    storage.add_memo(memo.clone());
    save_memos(&storage)?;
    
    Ok(format!(
        "✅ メモを保存しました: {} | {} {}℃",
        memo.city, memo.weather, memo.temperature
    ))
}

/// メモ一覧を文字列で取得（表示用）
pub fn format_all_memos() -> Result<String, String> {
    let storage = load_memos()?;

    if storage.memos.is_empty() {
        return Ok("📭 保存されているメモはありません".to_string());
    }

    let mut output = String::from("📋 保存済みメモ一覧:\n");
    for (i, memo) in storage.memos.iter().enumerate() {
        output.push_str(&format!(
            "{}. {} | {} | {} {}℃ | {}\n",
            i + 1,
            memo.timestamp,
            memo.city,
            memo.weather,
            memo.temperature,
            memo.note
        ));
    }

    Ok(output)
}

/// 特定都市のメモを検索して文字列で返す
pub fn format_memos_by_city(city: &str) -> Result<String, String> {
    let storage = load_memos()?;
    let results = storage.search_by_city(city);

    if results.is_empty() {
        return Ok(format!("🔍 「{}」のメモは見つかりませんでした", city));
    }

    let mut output = format!("🔍 「{}」のメモ:\n", city);
    for (i, memo) in results.iter().enumerate() {
        output.push_str(&format!(
            "{}. {} | {} {}℃ | {}\n",
            i + 1,
            memo.timestamp,
            memo.weather,
            memo.temperature,
            memo.note
        ));
    }

    Ok(output)
}
