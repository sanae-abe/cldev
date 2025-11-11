# cldev - Claude Development CLI

[![Build Status](https://github.com/sanae-abe/cldev/workflows/CI/badge.svg)](https://github.com/sanae-abe/cldev/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**cldev** 是一个用于管理 Claude Code 开发工作流的统一 CLI 工具。它将开发命令整合到单个类型安全、超快速的 Rust 二进制文件中，并完全支持国际化（英语/日语/中文）。

**当前状态**：已实现 35 个命令

[English](README.md) | [日本語](README.ja.md) | 简体中文 | [繁體中文](README.zh-TW.md)

---

## 目录

- [概述](#概述)
- [主要功能](#主要功能)
- [安装](#安装)
- [快速入门](#快速入门)
- [使用示例](#使用示例)
- [支持的语言和技术栈](#支持的语言和技术栈)
- [配置系统](#配置系统)
- [命令参考](#命令参考)
- [开发](#开发)
- [贡献](#贡献)
- [许可证](#许可证)

---

## 概述

**cldev** 将分散的 shell 脚本替换为统一的开发工具包：

- **统一**：9 个类别 35 个命令集成到单一工具（config、dev、git、quality、tech、ops、analysis、lr、todo）
- **加速**：启动速度提升 1.5 倍（约 21ms，比 gh CLI 的 32ms 快 1.5 倍）
- **简化**：安装简单（`cargo install cldev` 或 `brew install cldev`）
- **国际化**：所有输出支持多语言（英语/日语/简体中文/繁体中文，可扩展的 i18n 系统）
- **安全**：安全操作（防止路径遍历、防止命令注入）
- **自动检测**：自动检测项目类型（Node.js、Rust、Go、Python 等）

### 为什么选择 cldev？

**之前：**
```bash
# 分散在 3 个不同脚本集合中的命令
~/.claude/scripts/claude validate
uc feature user-auth
~/.claude/learning-analytics/context-search.sh "encryption"
```

**之后：**
```bash
# 具有智能默认值的单一统一 CLI
cldev config check
cldev dev feature user-auth
cldev lr find "encryption"
```

**改进：**
- 命令数量减少 15%（41 → 35）
- 命令语法缩短 77%
- 安装速度提升 80%
- 执行速度提升 1.5 倍（与 gh CLI 相比）
- 完全类型安全（Rust）
- i18n 支持（英语/日语/简体中文/繁体中文）

---

## 主要功能

### 🚀 性能
- **快速启动**：约 21ms（比 gh CLI 的 32ms 快 1.5 倍）
- **紧凑二进制**：3.3MB（比 gh CLI 的 51MB 小 93%）
- **优化的发布构建**：LTO、strip、codegen-units=1
- **高效资源使用**：最小内存占用

### 🌐 国际化
- **当前支持**：英语（en）、日语（ja）、简体中文（zh）、繁体中文（zh-TW）
- **路线图**：韩语（ko）- 第 2 年 Q2，其他语言按需添加
- **自动检测**：使用 `LANG` 环境变量
- **可扩展**：基于 JSON 的 i18n 系统（可升级到 fluent-rs）

### 🔒 安全
- **防止路径遍历**：安全的路径规范化
- **防止命令注入**：安全的命令执行
- **权限验证**：配置文件安全检查（600）
- **输入验证**：全面的清理

### 🎯 开发者体验
- **Shell 补全**：Bash、Zsh、Fish、PowerShell
- **交互式设置**：引导式配置向导
- **智能自动检测**：Git 远程、项目类型、技术栈
- **丰富输出**：彩色、格式化、表情符号增强（可配置）
- **全面帮助**：所有命令的详细 `--help`

### 🏗️ 架构
- **模块化设计**：清晰的关注点分离
- **3 层配置**：全局 → 技术栈 → 项目
- **类型安全**：Rust 的编译时保证
- **可扩展**：插件就绪的命令系统

### 📚 学习记录系统
- **内置知识库**：使用 `cldev lr find "主题"` 搜索过去的解决方案
- **问题跟踪**：使用 `cldev lr problems` 跟踪未解决的问题
- **学习分析**：使用 `cldev lr stats` 查看统计和模式
- **UTF-8 支持**：完全支持日语/中文全文搜索

与典型的开发 CLI 不同，cldev 包含一个可搜索的学习记录系统。虽然许多开发者在单独的工具中手动维护 TIL（今天我学到了）仓库或工程日志，但 cldev 将这直接集成到您的工作流中——使过去的解决方案可以从命令行立即检索。

```bash
# 记录学习会话
cldev lr new "JWT 认证实现"

# 搜索过去的解决方案
cldev lr find "authentication" --field topic

# 查看学习统计
cldev lr stats --period week
```

---

## 安装

### 选项 1：Cargo（Rust 包管理器）

```bash
# 从 crates.io 安装
cargo install cldev

# 或从源代码构建
git clone https://github.com/sanae-abe/cldev.git
cd cldev
cargo install --path .
```

### 选项 2：Homebrew（macOS/Linux）

```bash
# 添加 tap（即将推出）
brew tap sanae-abe/cldev
brew install cldev
```

### 选项 3：预构建二进制文件

下载适用于您平台的最新版本：

- [Linux x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [Linux aarch64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS aarch64 (Apple Silicon)](https://github.com/sanae-abe/cldev/releases/latest)
- [Windows x86_64](https://github.com/sanae-abe/cldev/releases/latest)

```bash
# 解压并安装
tar xzf cldev-*-x86_64-unknown-linux-gnu.tar.gz
sudo mv cldev /usr/local/bin/
```

### 验证安装

```bash
cldev --version
# 输出：cldev 1.0.0
```

---

## 快速入门

### 1. 初始化配置（5 分钟）

运行交互式设置向导：

```bash
cldev config init
```

这将：
- 检测您的语言偏好
- 配置 Claude Code 目录（`~/.claude`）
- 设置项目根目录
- 检测 Git CLI 工具（gh/glab）
- 安装 shell 补全
- 创建配置文件

### 2. 验证配置

```bash
cldev config check
# ✅ 配置文件有效
# 💡 下一步：cldev dev feature
```

### 3. 开始第一个功能

```bash
cldev dev feature user-authentication
# 引导您完成：
# - 分支创建
# - 实现规划
# - 测试脚手架
# - 提交准备
```

---

## 全局选项

所有命令都支持以下全局标志：

```bash
--verbose, -v      # 详细输出
--quiet, -q        # 抑制非错误输出
--no-color         # 禁用彩色输出
--lang <LANG>      # 覆盖语言（en/ja/zh/zh-TW）
--help, -h         # 显示帮助
--version, -V      # 显示版本
```

---

## 配置系统

### 3 层层次结构

```
🌍 全局配置 (~/.config/cldev/config.toml)
    │ 适用于所有项目的基本设置
    ▼
🔧 技术栈配置 (~/.claude/stacks/*.md)
    │ 技术特定设置（web/api/mobile/data-science）
    ▼
🎯 项目配置 (project/.claude/config.toml)
    │ 项目特定覆盖
```

### 配置文件结构

**位置**：`~/.config/cldev/config.toml`

```toml
# cldev 配置文件
version = "1.0.0"

[general]
language = "zh"  # en, ja, zh, 或 zh-TW
claude_dir = "/Users/username/.claude"
projects_dir = "/Users/username/projects"

[git]
github_cli = true
gitlab_cli = false
default_base_branch = "main"
auto_push = true

[quality]
auto_fix = false
run_tests_before_commit = true

[dev]
auto_create_branch = true
branch_prefix = "feature"
session_recording = true

[lr]
sessions_dir = "/Users/username/.claude/learning-sessions"
auto_save = true
default_tags = ["development", "claude-code"]

[ui]
color = true
emoji = true
progress_bar = true

[performance]
parallel_tasks = 4
timeout_seconds = 300
```

---

## 开发

### 前提条件

- **Rust 1.70+**（通过 [rustup](https://rustup.rs/) 安装）
- **Git 2.30+**
- 可选：`gh`（GitHub CLI）、`glab`（GitLab CLI）

### 从源代码构建

```bash
# 克隆仓库
git clone https://github.com/sanae-abe/cldev.git
cd cldev

# 调试模式构建
cargo build

# 构建优化的发布二进制
cargo build --release

# 本地安装
cargo install --path .
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 带输出运行
cargo test -- --nocapture

# 运行特定测试
cargo test test_config_load
```

### 代码质量

```bash
# 格式化代码
cargo fmt

# 检查格式
cargo fmt -- --check

# 运行 linter
cargo clippy

# 带严格检查运行 linter
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 许可证

本项目采用双重许可：

- **MIT 许可证**（[LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT）
- **Apache 许可证 2.0**（[LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）

您可以选择其中任一许可证使用。

---

## 支持

- **问题**：[GitHub Issues](https://github.com/sanae-abe/cldev/issues)
- **讨论**：[GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- **文档**：[docs/](docs/)

---

**由 cldev 团队用 ❤️ 制作**

*通过统一、智能的 CLI 工作流赋能开发者*
