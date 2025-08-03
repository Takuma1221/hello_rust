use std::io;
use rand::Rng;

pub fn play() {
    let cpu_target = rand::thread_rng().gen_range(0..=100);

    println!("数字あてゲーム！");
    println!("0〜100の数字を当ててね！");

    loop {
        println!("なんの数字だと思う？");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");

        let player_choice: usize = match input.trim().parse() {
            Ok(num) if num <= 100 => num,
            _ => {
                println!("0〜100の数字を入力してください");
                continue; // 入力が無効ならループをスキップして次の入力へ
            }
        };

        if player_choice == cpu_target {
            println!("あたりだよ！！！");
            break; // 正解ならループを抜ける
        } else if player_choice < cpu_target {
            println!("もっと大きいよ！");
        } else {
            println!("もっと小さいよ！");
        }
    }
}
