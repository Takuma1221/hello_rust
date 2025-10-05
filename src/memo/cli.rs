use crate::memo::storage;

/// メモアプリの CLI エントリーポイント
pub fn run() -> Result<(), String> {
    // 1. コマンドライン引数を取得
    let args: Vec<String> = std::env::args().collect();

    // 2. 引数が足りない場合はヘルプを表示
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    // 3. サブコマンドで分岐
    match args[1].as_str() {
        "add" => handle_add(&args[2..]),
        "list" => handle_list(),
        "delete" => handle_delete(&args[2..]),
        _ => {
            println!("❌ 不明なコマンド: {}", args[1]);
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("📝 メモアプリ");
    println!("使い方:");
    println!("  cargo run -- add <内容>      - メモを追加");
    println!("  cargo run -- list            - メモ一覧を表示");
    println!("  cargo run -- delete <id>     - メモを削除");
}

fn handle_add(args: &[String]) -> Result<(), String> {
    // 1. args が空なら "内容を指定してください" エラー
    if args.is_empty() {
        return Err("内容を指定してください".to_string());
    }

    // 2. args を全部つなげてメモ内容にする
    let content = args.join(" ");

    // 3. load_memos() で読み込み（mut をつける）
    let mut memo_list = storage::load_memos()?;

    // 4. add_memo() で追加
    memo_list.add_memo(content.clone());

    // 5. save_memos() で保存（参照で渡す）
    storage::save_memos(&memo_list)?;

    // 6. "✅ メモを追加しました: {}" と表示
    println!("✅ メモを追加しました: {}", content);

    Ok(())
}

fn handle_list() -> Result<(), String> {
    // 1. list_memos_formatted() を呼ぶ
    let output = storage::list_memos_formatted()?;

    // 2. 結果を println! で表示
    println!("{}", output);

    Ok(())
}

fn handle_delete(args: &[String]) -> Result<(), String> {
    // 1. args が空なら "ID を指定してください" エラー
    if args.is_empty() {
        return Err("ID を指定してください".to_string());
    }

    // 2. args[0] を u32 にパース
    let id: u32 = args[0]
        .parse()
        .map_err(|_| "ID は数値で指定してください".to_string())?;

    // 3. load_memos() で読み込み（mut をつける）
    let mut memo_list = storage::load_memos()?;

    // 4. remove_memo() で削除（戻り値が false なら "見つかりません"）
    let removed = memo_list.remove_memo(id);
    if !removed {
        return Err(format!("❌ メモが見つかりません (id: {})", id));
    }

    // 5. save_memos() で保存
    storage::save_memos(&memo_list)?;

    // 6. "✅ メモを削除しました (id: {})" と表示
    println!("✅ メモを削除しました (id: {})", id);

    Ok(())
}
