use super::Task;
use super::*;
use crate::config::{Config, TaskConfig};
use crate::push::{Push, PushPlus};
use cron::Schedule;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::process::Command;
use std::str::FromStr;
use tokio::sync::mpsc;
use tokio::time;

// 处理用户命令
pub fn handle_command(tasks: &mut HashMap<String, Task>, command: String, config: Config) {
    clear_screen();
    let parts: Vec<&str> = command.split_whitespace().collect();
    match parts.as_slice() {
        ["ls"] => {
            // Handle "ls" command
            list_tasks(tasks);
        }
        ["new", task_name, command, schedule, run_on_startup, restart_after_stop, enabled] => {
            // Handle "new" command
            // Create a new task with the provided parameters
            // task_name: String, schedule: String, run_on_startup: String, restart_after_stop: String
            let mut sche: Option<cron::Schedule> = None;
            match Schedule::from_str(schedule) {
                Ok(a) => {
                    sche = Some(a);
                    ()
                }
                Err(_) => {}
            };
            let new_task = Task::new(
                task_name,
                sche,
                run_on_startup.parse().unwrap(),
                Some(command.parse().unwrap()), // Replace with the actual command/file path if needed
                restart_after_stop.parse().unwrap(),
                enabled.parse().unwrap(),
            );
            tasks.insert(task_name.to_string(), new_task);
            println!("Created new task: {}", task_name);
            let task_configs: Vec<TaskConfig> = tasks
                .iter()
                .map(|(_, task)| TaskConfig::from(task))
                .collect();
            // let task_configs: Vec<config::TaskConfig> = tasks
            //     .iter()
            //     .filter_map(|(key, task)| {
            //         if key == task_name {
            //             Some(config::TaskConfig::from(task))
            //         } else {
            //             None
            //         }
            //     })
            //     .collect();
            let file = OpenOptions::new()
                .write(true)
                .create(false)
                .append(false)
                .open("config.yaml")
                .expect("Error opening file: ");

            serde_yaml::to_writer(
                file,
                &Config {
                    tasks: task_configs,
                    push: config.push,
                    is_push_enabled: config.is_push_enabled,
                },
            )
            .expect("Error writing to file:");
        }
        ["stop", task_name] => {
            // Handle "stop" command
            // Stop the specified task
            let task_name = task_name.trim_matches(|c| c == '"' || c == '\'');
            if let Some(task) = tasks.get_mut(task_name) {
                task.stop();
                println!("Stopped task: {}", task_name);
            } else {
                println!("Task not found: {}", task_name);
            }
        }
        ["start", task_name] => {
            // Handle "start" command
            // Start the specified task
            if let Some(task) = tasks.get_mut(task_name.to_owned()) {
                task.start();
                println!("Started task: {}", task_name);
            } else {
                println!("Task not found: {}", task_name);
            }
        }
        ["restart", task_name] => {
            // Handle "restart" command
            // Restart the specified task
            if let Some(task) = tasks.get_mut(task_name.to_owned()) {
                task.restart();
                println!("Restarted task: {}", task_name);
            } else {
                println!("Task not found: {}", task_name);
            }
        }
        ["exit"] | ["quit"] | ["q"] => {
            println!("Exiting program.");
            std::process::exit(0);
        }
        ["help"] | ["h"] => display_help(),
        _ => display_help(),
    }
    print!("$ ")
}

// 主循环，处理任务
pub async fn handle_tasks(
    mut tasks: HashMap<String, Task>,
    mut command_receiver: mpsc::Receiver<String>,
    config: Config,
) -> ! {
    // 启动任务
    for (_, task) in &mut tasks {
        if task.enabled {
            if task.run_on_startup {
                task.start();
            }
        }
    }
    clear_screen();
    list_tasks(&tasks);
    print!("$ ");
    // 主循环
    loop {
        // 遍历任务，检查任务状态,并执行cron表达式
        for (_task_name, task) in &mut tasks {
            if task.enabled {
                if task.should_run_now() {
                    task.start()
                }
            }

            match task.status {
                TaskStatus::Running => task.check_child(),
                TaskStatus::Completed => {}
                TaskStatus::ExceptionalExit => {
                    if task.restart_after_stop {
                        task.start()
                    }
                    if config.is_push_enabled {
                        let p = PushPlus::new(config.push.push_plus_token.as_str());
                        p.send_message(
                            format!("task: {}  Exceptional Exit", task.name.clone()),
                            "Nyar 推送".to_string(),
                        )
                        .await
                        .unwrap();
                    }
                }
                TaskStatus::UserStopped => {}
                TaskStatus::Unknown => {}
                TaskStatus::NotStarted => {}
            }
        }

        // 休眠一段时间
        time::sleep(time::Duration::from_secs(1)).await;

        // 处理外部命令
        while let Ok(command) = command_receiver.try_recv() {
            handle_command(&mut tasks, command, config.clone());
        }
    }
}

// 异步任务，处理用户输入
pub async fn handle_user_input(sender: mpsc::Sender<String>) {
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        sender.send(input.trim().to_string()).await.unwrap();
    }
}

// 显示帮助信息
fn display_help() {
    println!("Available commands:");
    println!("ls\t\t\tList tasks");
    println!("new <name> <command> <schedule> <run_on_startup> <restart_after_stop> <enabled>\tCreate a new task");
    println!("stop <name>\t\tStop the specified task");
    println!("start <name>\t\tStart the specified task");
    println!("restart <name>\t\tRestart the specified task");
    println!("exit | quit | q\tExit the program");
    println!("help | h\t\tDisplay this help message");
}
// 清除屏幕
pub fn clear_screen() {
    if cfg!(windows) {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}
