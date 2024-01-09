use chrono::Utc;
use cron::Schedule;
use std::env::consts::OS;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::process::{Child, Command};
use std::collections::HashMap;
mod handle;
pub use handle::*;

const LOGS_DIR: &str = "logs";
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TaskStatus {
    NotStarted,
    Running,
    Completed,
    UserStopped,
    ExceptionalExit,
    Unknown, // New variant for unknown state
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let var_name = match self {
            TaskStatus::Running => write!(f, "Running"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::ExceptionalExit => write!(f, "ExceptionalExit"),
            TaskStatus::UserStopped => write!(f, "UserStopped"),
            TaskStatus::Unknown => write!(f, "Unknown"),
            TaskStatus::NotStarted => write!(f, "NotStarted"),
            // Add more cases as needed
        };
        var_name
    }
}


#[derive(Debug)]
pub struct Task {
    pub name: String,
    // 表示任务的调度计划。
    pub schedule: Option<Schedule>,

    // 表示由任务生成的子进程。
    pub child: Option<Child>,

    // 表示任务是否在系统启动时运行。
    pub run_on_startup: bool,

    // 表示任务要执行的命令。
    pub command: Option<String>,

    // 表示任务停止后是否需要再次启动。
    pub restart_after_stop: bool,
    pub status: TaskStatus,
    pub enabled: bool,
    pub completion_count: u32,
}

impl Task {
    pub fn new(
        name: &str,
        schedule: Option<Schedule>,
        run_on_startup: bool,
        command: Option<String>,
        restart_after_stop: bool,
        enabled: bool,
    ) -> Task {
        Task {
            schedule,
            child: None,
            run_on_startup,
            command,
            restart_after_stop,
            name: name.to_string(),
            status: TaskStatus::NotStarted,
            enabled: enabled,
            completion_count: 0,
        }
    }

    // 启动任务
    pub fn start(&mut self) {
        if self.run_on_startup || self.schedule.is_none() {
            self.status = TaskStatus::Running;
            self.child = Some(self.start_subprocess());
        }
    }

    pub fn start_subprocess(&mut self) -> Child {
        if let Err(_) = std::fs::create_dir_all(LOGS_DIR) {}
        // 如果用户填写了命令，则使用用户的命令；否则默认使用 "main.py"
        let shell = match OS {
            "windows" => "powershell",
            _ => "bash",
        };

        let mut cmd = Command::new(shell);
        if OS == "windows" {
            cmd.arg("-NoProfile");
            cmd.arg("-Command");
        } else {
            cmd.arg("-c");
        }

        cmd.arg(self.command.clone().expect("请传入命令"));

        // 设置输出流
        cmd.stdout(File::create(format!("logs/{}-output.log", self.name)).expect("stdout err"));
        cmd.stderr(File::create(format!("logs/{}-error.log", self.name)).expect("stderr err"));
        // 启动子进程
        self.set_status(TaskStatus::Running);
        cmd.spawn().expect("Failed to start subprocess")
    }

    // 检查子进程是否停止
    pub fn check_child(&mut self) {
        if let Some(ref mut child) = self.child {
            if let Some(status) = child.try_wait().expect("Failed to check child status") {
                if status.success() {
                    // println!("Child process exited successfully");
                    self.status = TaskStatus::Completed;
                    self.completion_count += 1
                } else {
                    self.status = TaskStatus::ExceptionalExit;
                    // println!("Child process exited with an error, restarting...");
                    // self.start();
                }
            }
        }
    }
    pub fn schedule_to_string(&self) -> String {
        self.schedule
            .as_ref()
            .map_or_else(|| "None".to_string(), |s| s.to_string())
    }
    // 停止子进程
    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.child {
            println!("Stopping task...");
            child.kill().expect("Failed to kill child process");
            self.status = TaskStatus::UserStopped;
            self.child = None;
            self.schedule = None;
        }
    }
    pub fn restart(&mut self) {
        // Implement your task restart logic here
        self.stop();
        self.start();
    }
    pub fn set_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }
    pub fn should_run_now(&self) -> bool {
        std::io::stdout().flush().unwrap(); // 刷新输出缓冲区
                                            // 检查任务是否应该在当前时间执行
        match self.schedule {
            Some(ref s) => {
                let upcoming_times: Vec<_> = s.upcoming(Utc).take(1).collect();
                // println!("Debug: Upcoming times: {:?}", upcoming_times);
                // println!("{:?}", Utc::now());
                let should_run = upcoming_times
                    .iter()
                    .any(|dt| (Utc::now().timestamp()) - dt.timestamp() == -1);
                // println!("Debug: Should run: {}", should_run);

                should_run
            }
            None => {
                // println!("Debug: Task has no schedule");
                false
            }
        }
    }
}



fn list_tasks(tasks: &HashMap<String, Task>) {
    println!(
        "{: <20}  {: <20}  {: <15}  {: <10}  {: <15}  {: <10}  {: <10}  {: <10}",
        "Task Name",
        "Schedule",
        "Run on Startup",
        "Enabled",
        "Command",
        "Restart",
        "Status",
        "CompletionCount"
    );

    for (_, task) in tasks {
        println!(
            "{: <20}  {: <20}  {: <15}  {: <10}  {: <15}  {: <10}  {: <10}   {: <10}",
            task.name,
            task.schedule_to_string(),
            task.run_on_startup,
            task.enabled,
            task.command.as_ref().map_or("", |c| c.as_str()),
            task.restart_after_stop,
            task.status,
            task.completion_count
        );
    }
}
