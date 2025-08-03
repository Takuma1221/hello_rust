use rand::Rng;
use std::io;



fn print_histroy(words: String, history: &mut Vec<String>, result) {
    println!("{}です！", words);
    result += 1;
    history.push(words.to_string());
}

pub fn play() {
    let draw_words = "あいこ";
    let player_win_words = "かち";
    let cpu_win_words = "まけ";

    let hands = ["グー", "チョキ", "パー"];
    let mut player_wins = 0;
    let mut cpu_wins = 0;
    let mut draws = 0;
    let mut history: Vec<String> = Vec::new(); // 可変かどうか

    println!("じゃんけんゲーム！");
    while player_wins < 5 {
        println!("あなたの手を選んでください（0: グー, 1: チョキ, 2: パー）");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        let player_choice: usize = match input.trim().parse() {
            // OKの使い方
            Ok(num) if num <= 2 => num,
            _ => {
                println!("0〜2の数字を入力してください");
                return;
            }
        };
        
        let cpu_choice = rand::thread_rng().gen_range(0..=2);
        
        println!("あなた: {}", hands[player_choice]);
        println!("コンピュータ: {}", hands[cpu_choice]);
        
        match (player_choice+ 3 - cpu_choice) % 3 {
            0 =>{
                    print_histroy(draw_words, &mut history, draws);
                    println!("スコア: 勝ち {} / 負け {} / 引き分け {}", player_wins, cpu_wins, draws);
                },
            1 =>{
                print_histroy(cpu_win_words, &mut history, cpu_wins);
                println!("スコア: 勝ち {} / 負け {} / 引き分け {}", player_wins, cpu_wins, draws);
            } ,
            2 =>{
                print_histroy(player_win_words, &mut history, player_wins);
                println!("スコア: 勝ち {} / 負け {} / 引き分け {}", player_wins, cpu_wins, draws);
            } ,
            _ => unreachable!(),
        }
    }
    println!("君の勝ちだよ！おめでとう！");
    println!("対戦履歴:");
    // ポインタ
    for (i, result) in history.iter().enumerate() {
     println!("{}回目: {}", i + 1, result);
    }
    
}
