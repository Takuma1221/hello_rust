use std::io;

#[derive(Debug)]
enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Debug)]
struct Task {
    id: usize,
    description: String,
    status: Status,
}

// タスクを追加する関数
fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.len(); // 今のタスク数をIDに使う
    let task = Task {
        id,
        description,
        status: Status::Todo,
    };
    tasks.push(task);
    println!("タスクを追加しました！ID: {}", id);
}

fn edit_task(tasks: &mut Vec<Task>, id: usize, new_description: String) {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.description = new_description;
        println!("タスクを更新しました！ID: {}", id);
    } else {
        println!("タスクが見つかりません。ID: {}", id);
    }
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("[{}] [{:?}] {}", task.id, task.status, task.description);
    }
}

pub fn play() {
    let mut tasks: Vec<Task> = Vec::new();
    println!("タスク内容を教えてください");
    let mut input_description = String::new();
    io::stdin().read_line(&mut input_description).expect("入力エラー");

    add_task(&mut tasks, input_description.to_string());

    list_tasks(&tasks);
}