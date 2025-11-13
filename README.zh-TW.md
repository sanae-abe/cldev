# cldev - Claude Development CLI

[![Build Status](https://github.com/sanae-abe/cldev/workflows/CI/badge.svg)](https://github.com/sanae-abe/cldev/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**cldev** 是一個用於管理 Claude Code 開發工作流程的統一 CLI 工具。它將開發命令整合到單個類型安全、超快速的 Rust 二進位檔案中，並完全支援國際化（英語/日語/中文）。

**當前狀態**：已實現 35 個命令

[English](README.md) | [日本語](README.ja.md) | [简体中文](README.zh.md) | 繁體中文

---

## 目錄

- [概述](#概述)
- [主要功能](#主要功能)
- [安裝](#安裝)
- [快速入門](#快速入門)
- [使用範例](#使用範例)
- [支援的語言與技術堆疊](#支援的語言與技術堆疊)
- [配置系統](#配置系統)
- [命令參考](#命令參考)
- [開發](#開發)
- [貢獻](#貢獻)
- [授權](#授權)

---

## 概述

**cldev** 將分散的 shell 腳本替換為統一的開發工具包：

- **統一**：9 個類別 35 個命令整合到單一工具（config、dev、git、quality、tech、ops、analysis、lr、todo）
- **加速**：啟動速度提升 1.5 倍（約 21ms，比 gh CLI 的 32ms 快 1.5 倍）
- **簡化**：安裝簡單（`cargo install cldev` 或 `brew install cldev`）
- **國際化**：所有輸出支援多語言（英語/日語/簡體中文/繁體中文，可擴展的 i18n 系統）
- **安全**：安全操作（防止路徑遍歷、防止命令注入）
- **自動檢測**：自動檢測專案類型（Node.js、Rust、Go、Python 等）

### 為什麼選擇 cldev？

**之前：**
```bash
# 分散在 3 個不同腳本集合中的命令
~/.claude/scripts/claude validate
uc feature user-auth
~/.claude/learning-analytics/context-search.sh "encryption"
```

**之後：**
```bash
# 具有智慧預設值的單一統一 CLI
cldev config check
cldev dev feature user-auth
cldev lr find "encryption"
```

**改進：**
- 命令數量減少 15%（41 → 35）
- 命令語法縮短 77%
- 安裝速度提升 80%
- 執行速度提升 1.5 倍（與 gh CLI 相比）
- 完全類型安全（Rust）
- i18n 支援（英語/日語/簡體中文/繁體中文）

---

## 主要功能

### 🚀 效能
- **快速啟動**：約 21ms（比 gh CLI 的 32ms 快 1.5 倍）
- **緊湊二進位檔**：3.3MB（比 gh CLI 的 51MB 小 93%）
- **最佳化的發布建置**：LTO、strip、codegen-units=1
- **高效資源使用**：最小記憶體占用

### 🌐 國際化
- **目前支援**：英語（en）、日語（ja）、簡體中文（zh）、繁體中文（zh-TW）
- **路線圖**：韓語（ko）- 第 2 年 Q2，其他語言按需新增
- **自動檢測**：使用 `LANG` 環境變數
- **可擴展**：基於 JSON 的 i18n 系統（可升級到 fluent-rs）

### 🔒 安全
- **防止路徑遍歷**：安全的路徑規範化
- **防止命令注入**：安全的命令執行
- **權限驗證**：配置檔案安全檢查（600）
- **輸入驗證**：全面的清理

### 🎯 開發者體驗
- **Shell 補全**：Bash、Zsh、Fish、PowerShell
- **互動式設定**：引導式配置精靈
- **智慧自動檢測**：Git 遠端、專案類型、技術堆疊
- **豐富輸出**：彩色、格式化、表情符號增強（可配置）
- **全面說明**：所有命令的詳細 `--help`

### 🏗️ 架構
- **模組化設計**：清晰的關注點分離
- **3 層配置**：全域 → 技術堆疊 → 專案
- **類型安全**：Rust 的編譯時保證
- **可擴展**：外掛就緒的命令系統

### 📚 學習記錄系統
- **內建知識庫**：使用 `cldev lr find "主題"` 搜尋過去的解決方案
- **問題追蹤**：使用 `cldev lr problems` 追蹤未解決的問題
- **學習分析**：使用 `cldev lr stats` 檢視統計和模式
- **UTF-8 支援**：完全支援日語/中文全文搜尋

與典型的開發 CLI 不同，cldev 包含一個可搜尋的學習記錄系統。雖然許多開發者在單獨的工具中手動維護 TIL（今天我學到了）倉庫或工程日誌，但 cldev 將這直接整合到您的工作流程中——使過去的解決方案可以從命令列立即檢索。

```bash
# 記錄學習會話
cldev lr new "JWT 認證實作"

# 搜尋過去的解決方案
cldev lr find "authentication" --field topic

# 檢視學習統計
cldev lr stats --period week
```

---

## 安裝

### 選項 1：Cargo（Rust 套件管理器）

```bash
# 從 crates.io 安裝
cargo install cldev

# 或從原始碼建置
git clone https://github.com/sanae-abe/cldev.git
cd cldev
cargo install --path .
```

### 選項 2：Homebrew（macOS/Linux）

```bash
# 新增 tap（即將推出）
brew tap sanae-abe/cldev
brew install cldev
```

### 選項 3：預建置二進位檔案

下載適用於您平台的最新版本：

- [Linux x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [Linux aarch64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS aarch64 (Apple Silicon)](https://github.com/sanae-abe/cldev/releases/latest)
- [Windows x86_64](https://github.com/sanae-abe/cldev/releases/latest)

```bash
# 解壓並安裝
tar xzf cldev-*-x86_64-unknown-linux-gnu.tar.gz
sudo mv cldev /usr/local/bin/
```

### 驗證安裝

```bash
cldev --version
# 輸出：cldev 1.0.0
```

---

## 快速入門

### 1. 初始化配置（5 分鐘）

執行互動式設定精靈：

```bash
cldev config init
```

這將：
- 檢測您的語言偏好
- 配置 Claude Code 目錄（`~/.claude`）
- 設定專案根目錄
- 檢測 Git CLI 工具（gh/glab）
- 安裝 shell 補全
- 建立配置檔案

### 2. 驗證配置

```bash
cldev config check
# ✅ 配置檔案有效
# 💡 下一步：cldev dev feature
```

### 3. 開始第一個功能

```bash
cldev dev feature user-authentication
# 引導您完成：
# - 分支建立
# - 實作規劃
# - 測試腳手架
# - 提交準備
```

---

## 全域選項

所有命令都支援以下全域標誌：

```bash
--verbose, -v      # 詳細輸出
--quiet, -q        # 抑制非錯誤輸出
--no-color         # 停用彩色輸出
--lang <LANG>      # 覆蓋語言（en/ja/zh/zh-TW）
--help, -h         # 顯示說明
--version, -V      # 顯示版本
```

---

## 配置系統

### 3 層層次結構

```
🌍 全域配置 (~/.config/cldev/config.toml)
    │ 適用於所有專案的基本設定
    ▼
🔧 技術堆疊配置 (~/.claude/stacks/*.md)
    │ 技術特定設定（web/api/mobile/data-science）
    ▼
🎯 專案配置 (project/.claude/config.toml)
    │ 專案特定覆寫
```

### 配置檔案結構

**位置**：`~/.config/cldev/config.toml`

```toml
# cldev 配置檔案
version = "1.0.0"

[general]
language = "zh-TW"  # en, ja, zh, 或 zh-TW
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
sessions_dir = "/Users/username/.claude/learnings"
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

## 開發

### 前提條件

- **Rust 1.70+**（透過 [rustup](https://rustup.rs/) 安裝）
- **Git 2.30+**
- 選用：`gh`（GitHub CLI）、`glab`（GitLab CLI）

### 從原始碼建置

```bash
# 複製倉庫
git clone https://github.com/sanae-abe/cldev.git
cd cldev

# 除錯模式建置
cargo build

# 建置最佳化的發布二進位檔
cargo build --release

# 本機安裝
cargo install --path .
```

### 執行測試

```bash
# 執行所有測試
cargo test

# 帶輸出執行
cargo test -- --nocapture

# 執行特定測試
cargo test test_config_load
```

### 程式碼品質

```bash
# 格式化程式碼
cargo fmt

# 檢查格式
cargo fmt -- --check

# 執行 linter
cargo clippy

# 帶嚴格檢查執行 linter
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 授權

本專案採用雙重授權：

- **MIT 授權**（[LICENSE-MIT](LICENSE-MIT) 或 http://opensource.org/licenses/MIT）
- **Apache 授權 2.0**（[LICENSE-APACHE](LICENSE-APACHE) 或 http://www.apache.org/licenses/LICENSE-2.0）

您可以選擇其中任一授權使用。

---

## 支援

- **問題**：[GitHub Issues](https://github.com/sanae-abe/cldev/issues)
- **討論**：[GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- **文件**：[docs/](docs/)

---

**由 cldev 團隊用 ❤️ 製作**

*透過統一、智慧的 CLI 工作流程賦能開發者*
