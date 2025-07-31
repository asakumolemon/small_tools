# Small Tools

## 简介

这是一个包含各种小型工具的 Rust 项目。

## 构建和运行

### 先决条件

确保您已安装 Rust 编程语言和 Cargo 包管理器。您可以通过以下命令安装它们：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 构建

在项目根目录下运行以下命令来构建项目：

```bash
cargo build
```

### 运行

您可以运行项目中的特定二进制文件，例如 `main`：

```bash
cargo run
```

## 项目结构

```
.gitignore
Cargo.lock
Cargo.toml
readme.md
src\
    chat_mod\
        chat.rs
        mod.rs
        model.rs
        prompt.rs
    lib.rs
    main.rs
    todo_mod\
        mod.rs
        todo_list.rs
```

## 模块说明

- `chat_mod`: 包含与聊天功能相关的模块，例如 `chat.rs`、`model.rs` 和 `prompt.rs`。
- `todo_mod`: 包含与待办事项列表功能相关的模块，例如 `todo_list.rs`。
