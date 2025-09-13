use rand::Rng;
use std::io;

const HANDS: [&str; 3] = ["グー", "チョキ", "パー"];

// 結果の型を定義
#[derive(Debug)]
enum ResultType {
    Draw,
    Win,
    Lose,
}

// 結果の型に実装ブロックでメソッドを定義
impl ResultType {
    fn to_str(&self) -> &'static str { // kiku
        match self {
            ResultType::Draw => "あいこ",
            ResultType::Win => "かち",
            ResultType::Lose => "まけ",
        }
    }
}

// じゃんけんの結果を判定する関数
fn judge(player: usize, cpu: usize) -> ResultType {
    match (player + 3 - cpu) % 3 {
        0 => ResultType::Draw,
        1 => ResultType::Lose,
        2 => ResultType::Win,
        _ => unreachable!(),
    }
}

// じゃんけんの結果を表示する関数
fn print_score(player_wins: usize, cpu_wins: usize, draws: usize) {
    println!("スコア: 勝ち {} / 負け {} / 引き分け {}", player_wins, cpu_wins, draws);
}

// プレイヤーの入力を取得する関数
fn get_player_input() -> usize {
    loop {
        println!("あなたの手を選んでください（0: グー, 1: チョキ, 2: パー）");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");

        if let Ok(num) = input.trim().parse::<usize>() {
            if num <= 2 {
                return num;
            }
        }
        println!("0〜2の数字を入力してください。");
    }
}

pub fn play() {
    let mut player_wins = 0;
    let mut cpu_wins = 0;
    let mut draws = 0;
    let mut history: Vec<ResultType> = Vec::new();

    println!("じゃんけんゲーム！5回勝利で終了です。");

    while player_wins < 5 {
        let player_choice = get_player_input();
        let cpu_choice = rand::thread_rng().gen_range(0..=2);

        println!("あなた: {}", HANDS[player_choice]);
        println!("コンピュータ: {}", HANDS[cpu_choice]);

        let result = judge(player_choice, cpu_choice);
        println!("{}です！", result.to_str());

        match result {
            ResultType::Draw => draws += 1,
            ResultType::Win => player_wins += 1,
            ResultType::Lose => cpu_wins += 1,
        }

        history.push(result);
        print_score(player_wins, cpu_wins, draws);
    }

    println!("君の勝ちだよ！おめでとう！");
    println!("対戦履歴:");
    for (i, result) in history.iter().enumerate() {
        println!("{}回目: {}", i + 1, result.to_str());
    }
}
