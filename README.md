[English](README-en.md) | 简体中文

# nyar

`nyar` 是一个用 `Rust` 编写的任务管理程序，它可以让您在后台运行和管理各种任务，例如定时任务，启动任务，重启任务等。

- `nyar`使用了 `Rust` 的高性能和安全性，保证了任务的快速和稳定的执行。

- `nyar`使用了 `YAML` 格式的配置文件，让您可以方便地创建和修改任务的参数和属性。

- `nyar`提供了一个简洁的命令行界面，让您可以轻松地控制和查看任务的状态和输出。

- `nyar`支持多种平台，包括 `Linux`，`Windows` 和 `MacOS`。

## 安装
你可以使用Cargo从crates.io安装nyar：
```bash
cargo install nyar
```
您可以从 [GitHub](https://github.com/limitcool/nyar) 下载 nyar 的源码，然后使用 cargo 工具进行编译和安装：

```bash
git clone https://github.com/limitcool/nyar.git
cd nyar
cargo build --release
cargo install --path .
```

或者，您也可以直接从 [GitHub](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) 下载 nyar 的二进制文件，然后将其放到您的系统路径中：

```bash
# Linux
wget https://github.com/limitcool/nyar/releases/download/v0.1.0/nyar-v0.1.0-x86_64-linux.tar.xz
xz -d nyar-v0.1.0-x86_64-linux.tar.xz
tar -xvf nyar-v0.1.0-x86_64-linux.tar
chmod +x nyar-linux-x86_64
mv nyar-linux-x86_64 /usr/local/bin/nyar
```

## 配置

nyar 使用 YAML 格式的配置文件，让您可以方便地创建和修改任务的参数和属性。

配置文件的基本结构如下：

```yaml
is_push_enabled: false # 是否启用推送服务
push: # 推送服务的配置
  push_plus_token: '' # pushplus 的 token
tasks: # 任务列表
- name: DefaultTask # 任务名
  command: echo Nyar! # 任务命令
  run_on_startup: true # 是否在启动时运行
  schedule: '*/10 * * * * *' # 任务执行周期
  restart_after_stop: false # 是否在停止后重启
  enabled: true # 是否启用任务
```

您可以根据您的需要，添加或删除任务，或者修改任务的参数和属性。任务的参数和属性的含义如下：

- `name`: 任务名，必须是唯一的，不能包含空格或特殊字符。
- `command`: 任务命令，可以是任何有效的 shell 命令，可以包含环境变量或参数。
- `run_on_startup`: 是否在启动时运行，可以是 `true` 或 `false`。
- `schedule`: 任务执行周期，可以是一个 cron 表达式，表示每隔多久执行一次任务，或者一个时间戳，表示在某个时间点执行一次任务。cron 表达式的格式是 `秒 分 时 日 月 周`，例如 `*/10 * * * * *` 表示每隔 10 秒执行一次任务。时间戳的格式是 `YYYYMMDD.HHMMSS`，例如 `20210101.000000` 表示在 2021 年 1 月 1 日 0 点 0 分 0 秒执行一次任务。
- `restart_after_stop`: 是否在停止后重启，可以是 `true` 或 `false`。如果是 `true`，则表示在任务执行完毕或出错后，会重新启动任务，直到手动停止任务。如果是 `false`，则表示在任务执行完毕或出错后，不会重新启动任务。
- `enabled`: 是否启用任务，可以是 `true` 或 `false`。如果是 `true`，则表示任务会按照配置文件的设置进行执行。如果是 `false`，则表示任务不会执行，除非手动启动任务。

## 使用

nyar 提供了一个简洁的命令行界面，让您可以轻松地控制和查看任务的状态和输出。nyar 支持以下命令：

- `ls`: 列出所有任务的信息。
- `new <name> <command> <schedule> <run_on_startup> <restart_after_stop> <enabled>`: 创建一个新的任务，并根据参数设置任务的属性。参数的含义和配置文件中的相同。
- `stop <name>`: 停止指定的任务，参数是任务名。
- `start <name>`: 启动指定的任务，参数是任务名。
- `restart <name>`: 重启指定的任务，参数是任务名。
- `exit | quit | q`: 退出 nyar 程序。
- `help | h`: 显示帮助信息。
