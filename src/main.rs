use std::collections::HashMap;

use tokio::sync::mpsc;

mod config;
mod push;
mod task;

#[allow(unreachable_code)]
#[tokio::main]
async fn main() {
    // 创建任务集合，每个任务有自己的 cron 表达式和输出文件
    let mut tasks: HashMap<String, task::Task> = HashMap::new();
    let config = config::Config::from_file("config.yaml").unwrap();

    for task_config in config.clone().tasks {
        let task = task_config.to_task();
        println!("{:?}", task.name);
        tasks.insert(task.name.clone(), task);
    }

    // 创建异步任务处理用户输入
    let (command_sender, command_receiver) = mpsc::channel(10);
    let user_input_handle = tokio::task::spawn(task::handle_user_input(command_sender));

    // 创建异步任务处理任务
    let tasks_handle = tokio::task::spawn(task::handle_tasks(tasks, command_receiver, config));

    // 等待任务处理结束
    tokio::try_join!(user_input_handle, tasks_handle).unwrap();
}
