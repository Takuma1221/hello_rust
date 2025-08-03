use std::io;

mod rejanken;
mod janken2;
mod number_guess;
mod todo;

fn main() {
    println!("遊びたいゲームを選んでください");
    println!("1: じゃんけん");
    println!("2: じゃんけん連勝チャレンジ");  
    println!("3: 数字当てゲーム");  
    println!("4: タスク管理");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let choice = input.trim();

    match choice {
        "1" => rejanken::play(),
        "2" => janken2::play(),
        "3" => number_guess::play(),
        "4" => todo::play(),
        _ => println!("無効な入力です"),
    }
}
