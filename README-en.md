English| [简体中文](README.md)

# `nyar`

`nyar` is a task management program written in Rust, which allows you to run and manage various tasks in the background, such as scheduled tasks, start tasks, restart tasks, etc.

- `nyar` uses the performance and security of Rust to ensure the rapid and stable execution of tasks.

- `nyar` uses the YAML format configuration file, which allows you to easily create and modify task parameters and attributes.

- `nyar` provides a concise command line interface, which allows you to easily control and view the status and output of tasks.

- `nyar` supports multiple platforms, including Linux, Windows, and MacOS.

## Installation
you can use Cargo to install nyar from crates.io:
```bash
cargo install nyar
```

Or, You can download the source code of `nyar` from [GitHub](https://github.com/limitcool/nyar), and then compile and install it using the cargo tool:


```bash
git clone https://github.com/limitcool/nyar.git
cd nyar
cargo build --release
cargo install --path .
```

Or you can directly download the binary file of `nyar` from [GitHub](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html), and then put it into your system path:

```bash
# Linux
wget https://github.com/limitcool/nyar/releases/download/v0.1.0/nyar-v0.1.0-x86_64-linux.tar.xz
xz -d nyar-v0.1.0-x86_64-linux.tar.xz
tar -xvf nyar-v0.1.0-x86_64-linux.tar
chmod +x nyar-linux-x86_64
mv nyar-linux-x86_64 /usr/local/bin/nyar
```


## Configuration

`nyar` uses the YAML format configuration file, which allows you to easily create and modify task parameters and attributes.

The basic structure of the configuration file is as follows:

```yaml
is_push_enabled: false # Whether to enable push service
push: # Push service configuration
  push_plus_token: '' # pushplus token
tasks: # Task list
- name: DefaultTask # Task name
  command: echo Nyar! # Task command
  run_on_startup: true # Whether to run on startup
  schedule: '*/10 * * * * *' # Task execution frequency
  restart_after_stop: false # Whether to restart after stopping
  enabled: true # Whether to enable the task
```

You can add or delete tasks, or modify task parameters and attributes according to your needs. The meanings of task parameters and attributes are as follows:

- `name`: Task name, must be unique, cannot contain spaces or special characters.
- `command`: Task command, can be any valid shell command, can include environment variables or parameters.
- `run_on_startup`: Whether to run on startup, can be `true` or `false`. If set to `true`, the task will be executed when the program starts. If set to `false`, the task will not be executed until manually started.
- `schedule`: Task execution frequency, can be a cron expression representing how often the task is executed, or a timestamp representing when the task is executed once. The cron expression format is `seconds minute hour day month week`, for example `*/10 * * * * *` means execute the task every 10 seconds. If set to a timestamp, the task will only be executed once at that time.
- `restart_after_stop`: Whether to restart after stopping, can be `true` or `false`. If set to `true`, the task will be restarted after it is executed or encounters an error until manually stopped. If set to `false`, the task will not be restarted after it is executed or encounters an error.
- `enabled`: Whether to enable the task, can be `true` or `false`. If set to `false`, the task will not be executed unless manually started.

## Usage

`nyar` provides a concise command line interface that allows you to easily control and view the status and output of tasks. `nyar` supports the following commands:

- `ls`: Lists detailed information about all tasks.
- `new <name> <command> <schedule> <run_on_startup> <restart_after_stop> <enabled>`: Creates a new task and sets its attributes based on the provided parameters. The meaning of the parameters is the same as in the configuration file.
- `stop <name>`: Stops the specified task, where `<name>` is the task name.
- `start <name>`: Starts the specified task, where `<name>` is the task name.
- `restart <name>`: Restarts the specified task, where `<name>` is the task name.
- `exit | quit | q`: Exits the `nyar` program.
- `help | h`: Displays help information.