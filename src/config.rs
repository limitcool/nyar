use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use crate::task::Task;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub is_push_enabled: bool,
    pub push: PushConfig,
    pub tasks: Vec<TaskConfig>,
}

impl Config {
    pub fn config_file() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "initcool", "nyar")
            .expect("Failed to get project directories");
        let mut config_file = PathBuf::from(proj_dirs.config_dir());
        std::fs::create_dir_all(&config_file).expect("");
        config_file.push("config.yaml");
        return  config_file;
    }

    pub fn load_default_config() -> Result<Self, Box<dyn std::error::Error>> {
        match File::open(Config::config_file()) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();

                let config: Config = serde_yaml::from_str(&content)?;
                Ok(config)
            }
            Err(_e) => {
                let default_config = Config {
                    tasks: vec![TaskConfig {
                        name: "DefaultTask".to_string(),
                        schedule: Some("*/10 * * * * *".to_string()),
                        run_on_startup: true,
                        command: Some("echo Nyar!".to_string()),
                        restart_after_stop: false,
                        enabled: true,
                    }],
                    is_push_enabled: false,
                    push: PushConfig {
                        push_plus_token: String::new(),
                    },
                };

                // Write default_config to config.yaml
                match serde_yaml::to_writer(File::create(Config::config_file())?, &default_config) {
                    Ok(_) => {
                        println!("Default configuration written to config.yaml");
                        Ok(default_config)
                    }
                    Err(err) => {
                        eprintln!(
                            "Error writing default configuration to config.yaml: {}",
                            err
                        );
                        Err(Box::new(err))
                    }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskConfig {
    pub name: String,
    pub command: Option<String>,
    pub run_on_startup: bool,
    pub schedule: Option<String>,
    pub restart_after_stop: bool,
    pub enabled: bool,
}

impl From<&Task> for TaskConfig {
    fn from(task: &Task) -> Self {
        let mut sched: Option<cron::Schedule> = None;
        match task.schedule.clone() {
            Some(e) => {
                sched = Some(e);
                ()
            }
            None => {}
        };
        let mut sched_str = String::new();
        if !sched.is_none() {
            sched_str = sched.unwrap().to_string();
        }

        return Self {
            name: task.name.clone(),
            command: task.command.clone(),
            run_on_startup: task.run_on_startup,
            schedule: Some(sched_str),
            restart_after_stop: task.restart_after_stop,
            enabled: task.enabled,
        };
    }
}
impl TaskConfig {
    pub fn to_task(&self) -> Task {
        let schedule = match self.schedule {
            Some(ref s) => match cron::Schedule::from_str(s) {
                Ok(parsed_schedule) => Some(parsed_schedule),
                Err(_) => None,
            },
            None => None,
        };
        let task = Task::new(
            self.name.as_str(),
            schedule,
            self.run_on_startup,
            self.command.clone(),
            self.restart_after_stop,
            self.enabled,
        );
        task
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PushConfig {
    pub push_plus_token: String,
}
