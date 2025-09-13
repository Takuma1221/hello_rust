use rand::Rng;
use std::io;

pub fn play() {
    let mut consecutive_wins = 0;
    let hands = ["グー", "チョキ", "パー"];
    println!("じゃんけん連勝チャレンジ！");

    while consecutive_wins < 3 {
        println!("あなたの手を選んでください（0: グー, 1: チョキ, 2: パー）");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        let player_choice: usize = match input.trim().parse() {
            Ok(num) if num <= 2 => num,
            _ => {
                println!("0〜2の数字を入力してください");
                continue;
            }
        };
        
        let cpu_choice = rand::thread_rng().gen_range(0..=2);
        
        println!("あなた: {}", hands[player_choice]);
        println!("コンピュータ: {}", hands[cpu_choice]);
        
        match (player_choice+ 3 - cpu_choice) % 3 {
            0 =>{
                    println!("あいこです！");
                    consecutive_wins = 0;
                },
            1 =>{
                println!("あなたの負け！");
                consecutive_wins = 0;
            } ,
            2 =>{
                println!("あなたの勝ち！");
                consecutive_wins += 1;
            } ,
            _ => unreachable!(),
        }
    } 
    println!("チャレンジ成功！！！")
}