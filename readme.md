         
# Small Tools

## 简介

这是一个包含各种小型实用工具的 Rust 项目，旨在提供简单易用的命令行工具集合。目前包含待办事项管理和 AI 聊天助手两个主要功能模块。

## 特性

- 📝 **待办事项管理**：创建、编辑、删除和查看待办事项，支持设置截止日期
- 🤖 **AI 聊天助手**：与 AI 模型进行对话，支持流式输出
- 🔧 **模型配置**：添加、编辑、删除和选择不同的 AI 模型
- 📋 **Prompt 配置**：管理和使用自定义的对话提示
- 💾 **本地存储**：所有数据保存在本地，支持 Windows 和其他操作系统
- 🎨 **美观界面**：使用 Unicode 字符美化命令行界面

## 构建和运行

### 先决条件

确保您已安装 Rust 编程语言和 Cargo 包管理器。您可以通过以下命令安装它们：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

对于 Windows 用户，请访问 [Rust 官方网站](https://www.rust-lang.org/tools/install) 下载安装程序。

### 构建

在项目根目录下运行以下命令来构建项目：

```bash
cargo build
```

### 运行

您可以运行以下命令启动应用程序：

```bash
cargo run
```

## 使用方法

### 主菜单

启动应用后，您将看到主菜单，可以选择以下功能：

1. 待办事项列表 (todo_list)
2. 问答模式 (chat)

### 待办事项管理

在待办事项管理模块中，您可以：

- 添加新的待办事项（标题、内容、截止日期）
- 编辑现有待办事项
- 删除待办事项
- 查看所有待办事项列表

### AI 聊天助手

在聊天模块中，您可以：

- 配置 AI 模型（API Key、模型名称、URL）
- 设置自定义 Prompt
- 与 AI 进行对话，支持流式输出
- 使用特殊命令：
  - `:b` - 返回上级菜单
  - `:c` - 清空对话历史
  - `:cls` - 清屏
  - `:revert` - 撤销最后一次对话

## 数据存储

所有配置和数据都保存在本地：

- Windows: `%LOCALAPPDATA%\SmallTool\`
- 其他系统: `~/.local/share/small_tools/`

## 项目结构

```
.gitignore
Cargo.lock
Cargo.toml
readme.md
src\
    chat_mod\        # 聊天功能模块
        chat.rs      # 聊天核心功能
        mod.rs       # 模块导出
        model.rs     # AI 模型管理
        prompt.rs    # 提示词管理
    todo_mod\        # 待办事项功能模块
        mod.rs       # 模块导出
        todo_list.rs # 待办事项管理
    lib.rs           # 库入口
    main.rs          # 主程序入口
```

## 模块说明

### chat_mod

- `chat.rs`: 实现聊天功能的核心逻辑，包括消息处理、API 请求和流式输出。
- `model.rs`: 管理 AI 模型配置，包括添加、编辑、删除和选择模型
- `prompt.rs`: 管理对话提示配置，支持自定义系统提示

### todo_mod

- `todo_list.rs`: 实现待办事项管理功能，包括添加、编辑、删除和显示待办事项

## 环境变量

可以通过环境变量设置默认的 AI 模型配置：

- `CHAT_URL`: 默认的 API URL
- `CHAT_API_KEY`: 默认的 API Key

## 贡献

欢迎提交 Pull Request 或创建 Issue 来改进这个项目。

        