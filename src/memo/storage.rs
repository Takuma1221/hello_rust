// ここに storage.rs を書いてください！
// ガイド: docs/learning/memo_storage_guide.md を参照

use std::fs;
use std::path::Path;

use crate::memo::types::MemoList;

/// メモファイルのデフォルトパス
const MEMO_FILE: &str = "data/memos.json";

/// ファイルからメモリストを読み込む
/// ファイルがなければ空のリストを返す
pub fn load_memos() -> Result<MemoList, String> {
    load_memos_from_path(MEMO_FILE)
}

/// 内部用：指定されたパスから読み込む
fn load_memos_from_path(file_path: &str) -> Result<MemoList, String> {
    let path = Path::new(file_path);

    // 改善1: Rust では if の条件に () は不要
    if !path.exists() {
        return Ok(MemoList::new());
    }

    // ファイル読み込み
    let content = fs::read_to_string(path).map_err(|e| format!("ファイル読み込みエラー: {}", e))?;

    // JSON に変換
    let memo_list: MemoList =
        serde_json::from_str(&content).map_err(|e| format!("JSON 変換エラー: {}", e))?;

    // 改善2: Rust では最後の式は return 不要（暗黙の return）
    Ok(memo_list)
}

/// メモリストをファイルに保存する
pub fn save_memos(memo_list: &MemoList) -> Result<(), String> {
    save_memos_to_path(memo_list, MEMO_FILE)
}

/// 内部用：指定されたパスに保存する
fn save_memos_to_path(memo_list: &MemoList, file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    // data/ ディレクトリがなければ作成
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("ディレクトリ作成エラー: {}", e))?;
        }
    }

    // MemoList を JSON 文字列に変換（整形付き）
    let json =
        serde_json::to_string_pretty(memo_list).map_err(|e| format!("JSON 変換エラー: {}", e))?;

    // ファイルに書き込み
    fs::write(path, json).map_err(|e| format!("ファイル書き込みエラー: {}", e))?;

    Ok(())
}

pub fn list_memos_formatted() -> Result<String, String> {
    let memo_list = load_memos()?;

    if memo_list.memos.is_empty() {
        return Ok("📭 メモはありません".to_string());
    }

    let lines: Vec<String> = memo_list
        .memos
        .iter()
        .map(|memo| format!("[id:{}] {} - {}", memo.id, memo.content, memo.created_at))
        .collect();

    let output = format!("📝 メモ一覧:\n{}\n", lines.join("\n"));
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // テスト用の一時ファイルパス
    const TEST_FILE: &str = "data/test_memos.json";

    // テスト後にファイルを削除
    fn cleanup() {
        let _ = fs::remove_file(TEST_FILE);
    }

    #[test]
    fn test_load_memos_no_file() {
        cleanup();

        // ファイルがない場合は空のリストを返す
        let result = load_memos_from_path(TEST_FILE);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().memos.len(), 0);

        cleanup();
    }

    #[test]
    fn test_save_and_load_memos() {
        cleanup();

        // メモを作成して保存
        let mut list = MemoList::new();
        list.add_memo("テストメモ1".to_string());
        list.add_memo("テストメモ2".to_string());

        let save_result = save_memos_to_path(&list, TEST_FILE);
        assert!(save_result.is_ok());

        // 読み込んで確認
        let loaded = load_memos_from_path(TEST_FILE).unwrap();
        assert_eq!(loaded.memos.len(), 2);
        assert_eq!(loaded.memos[0].content, "テストメモ1");
        assert_eq!(loaded.memos[1].content, "テストメモ2");

        cleanup();
    }

    #[test]
    fn test_list_memos_formatted_empty() {
        // 空のリストの場合
        let list = MemoList::new();

        let lines: Vec<String> = list
            .memos
            .iter()
            .map(|memo| format!("[id:{}] {} - {}", memo.id, memo.content, memo.created_at))
            .collect();

        assert_eq!(lines.len(), 0);
    }

    #[test]
    fn test_list_memos_formatted_with_data() {
        let mut list = MemoList::new();
        list.add_memo("テストメモ1".to_string());
        list.add_memo("テストメモ2".to_string());

        let lines: Vec<String> = list
            .memos
            .iter()
            .map(|memo| format!("[id:{}] {} - {}", memo.id, memo.content, memo.created_at))
            .collect();

        let output = lines.join("\n");

        assert!(output.contains("テストメモ1"));
        assert!(output.contains("テストメモ2"));
        assert!(output.contains("id:1"));
        assert!(output.contains("id:2"));
    }
}
