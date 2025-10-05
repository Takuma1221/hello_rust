use chrono::Local;
use serde::{Deserialize, Serialize};

/// 1つのメモを表す構造体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memo {
    pub id: u32,            // ← pub を追加（外部から読めるように）
    pub content: String,    // ← pub を追加
    pub created_at: String, // ← pub を追加
}

/// メモのリスト全体
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoList {
    pub memos: Vec<Memo>, // ← pub を追加
}

impl MemoList {
    /// 空のメモリストを作成
    pub fn new() -> Self {
        Self { memos: Vec::new() }
    }

    /// 新しいメモを追加
    pub fn add_memo(&mut self, content: String) {
        let id = self.next_id();
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let memo = Memo {
            id,
            content,
            created_at,
        };

        self.memos.push(memo); // 末尾に追加
    }

    /// メモを削除（成功したら true）
    pub fn remove_memo(&mut self, id: u32) -> bool {
        let before_len = self.memos.len();
        self.memos.retain(|m| m.id != id); // id が一致しないものだけ残す
        self.memos.len() < before_len // 長さが減ったら削除成功
    }

    /// 次の ID を計算
    /// 削除済みのIDと重複する可能性 -> 削除したものとも重複しない方がいい
    fn next_id(&self) -> u32 {
        self.memos.iter().map(|m| m.id).max().unwrap_or(0) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_memo_list() {
        let list = MemoList::new();
        assert_eq!(list.memos.len(), 0);
    }

    #[test]
    fn test_add_memo() {
        let mut list = MemoList::new();
        list.add_memo("買い物".to_string());

        assert_eq!(list.memos.len(), 1);
        assert_eq!(list.memos[0].id, 1);
        assert_eq!(list.memos[0].content, "買い物");
    }

    #[test]
    fn test_add_multiple_memos() {
        let mut list = MemoList::new();
        list.add_memo("買い物".to_string());
        list.add_memo("勉強".to_string());
        list.add_memo("運動".to_string());

        assert_eq!(list.memos.len(), 3);
        assert_eq!(list.memos[0].id, 1);
        assert_eq!(list.memos[1].id, 2);
        assert_eq!(list.memos[2].id, 3);
    }

    #[test]
    fn test_remove_memo_success() {
        let mut list = MemoList::new();
        list.add_memo("買い物".to_string());
        list.add_memo("勉強".to_string());

        let removed = list.remove_memo(1);

        assert!(removed);
        assert_eq!(list.memos.len(), 1);
        assert_eq!(list.memos[0].id, 2);
    }

    #[test]
    fn test_remove_memo_not_found() {
        let mut list = MemoList::new();
        list.add_memo("買い物".to_string());

        let removed = list.remove_memo(999);

        assert!(!removed);
        assert_eq!(list.memos.len(), 1);
    }

    #[test]
    fn test_next_id_empty_list() {
        let list = MemoList::new();
        assert_eq!(list.next_id(), 1);
    }

    #[test]
    fn test_next_id_after_deletion() {
        let mut list = MemoList::new();
        list.add_memo("メモ1".to_string());
        list.add_memo("メモ2".to_string());
        list.add_memo("メモ3".to_string());
        list.remove_memo(2);

        // 削除後も次のIDは最大ID + 1
        assert_eq!(list.next_id(), 4);
    }
}
