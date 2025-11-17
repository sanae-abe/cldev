# cldev - Claude Development CLI

[![Build Status](https://github.com/sanae-abe/cldev/workflows/CI/badge.svg)](https://github.com/sanae-abe/cldev/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.83%2B-orange.svg)](https://www.rust-lang.org)

**cldev** は、Claude Code との開発ワークフローを管理するための統合CLIツールです。開発コマンドを単一の型安全で高速なRustバイナリに統合し、完全な多言語対応（英語/日本語/中国語）を提供します。

**現在のステータス**: 35コマンド実装済み

[English](README.md) | 日本語

<!-- Screenshot placeholder: Add demo GIF showing cldev config init -->

---

## 目次

- [概要](#概要)
- [主な機能](#主な機能)
- [インストール](#インストール)
- [クイックスタート](#クイックスタート)
- [使用例](#使用例)
- [対応言語・技術スタック](#対応言語技術スタック)
- [設定システム](#設定システム)
- [コマンドリファレンス](#コマンドリファレンス)
- [開発](#開発)
- [コントリビューション](#コントリビューション)
- [ライセンス](#ライセンス)

---

## 概要

**cldev** は断片化したシェルスクリプトを統一的な開発ツールキットに置き換えます：

- **統合**: 9カテゴリ35コマンド（config、dev、git、quality、tech、ops、analysis、lr、todo）
- **高速化**: 約21msで起動（hyperfineベンチマーク、gh CLIの32msより1.5倍高速）
- **簡単インストール**: `cargo install cldev`
- **多言語対応**: 全出力を英語/日本語/中国語で提供（拡張可能なi18nシステム）
- **セキュア**: パストラバーサル防止、コマンドインジェクション保護
- **自動検出**: プロジェクトタイプ（Node.js、Rust、Go、Python等）を自動認識

### なぜcldevか？

**以前:**
```bash
# 3つの異なるスクリプトに分散したコマンド
~/.claude/scripts/claude validate
uc feature user-auth
~/.claude/learning-analytics/context-search.sh "encryption"
```

**cldev導入後:**
```bash
# インテリジェントなデフォルト設定を持つ単一の統一CLI
cldev config check
cldev dev feature user-auth
cldev lr find "encryption"
```

**改善点:**
- コマンド数15%削減（41 → 35）
- コマンド構文77%短縮
- インストール時間80%高速化
- 実行速度1.5倍高速（vs gh CLI）
- 完全な型安全性（Rust）
- 多言語対応（英語/日本語/簡体字中国語/繁体字中国語）

---

## 主な機能

### 🚀 パフォーマンス
- **高速起動**: 約21ms（hyperfineベンチマーク、コールドスタート約346ms）
- **コンパクトなバイナリ**: 3.3MB（gh CLIの51MBより93%小型）
- **最適化されたリリースビルド**: LTO、strip、codegen-units=1
- **効率的なリソース使用**: 最小限のメモリフットプリント

### 🌐 国際化
- **現在サポート**: 英語（en）、日本語（ja）、簡体字中国語（zh）、繁体字中国語（zh-TW）
- **ロードマップ**: 韓国語（ko） - Year 2 Q2、その他の言語はリクエストに応じて対応
- **自動検出**: `LANG`環境変数を使用
- **拡張可能**: JSONベースのi18nシステム（fluent-rsへのアップグレード可能）

### 🔒 セキュリティ
- **パストラバーサル防止**: セキュアなパス正規化
- **コマンドインジェクション保護**: 安全なコマンド実行
- **権限検証**: 設定ファイルのセキュリティチェック（600）
- **入力検証**: 包括的なサニタイゼーション

### 🎯 開発者体験
- **シェル補完**: Bash、Zsh、Fish、PowerShell
- **対話的セットアップ**: ガイド付き設定ウィザード
- **スマート自動検出**: Gitリモート、プロジェクトタイプ、技術スタック
- **リッチな出力**: カラー、フォーマット、絵文字対応（設定可能）
- **包括的なヘルプ**: 全コマンドに詳細な`--help`

### 🏗️ アーキテクチャ
- **モジュラー設計**: 関心の明確な分離
- **3層設定**: グローバル → 技術スタック → プロジェクト
- **型安全**: Rustのコンパイル時保証
- **拡張可能**: プラグイン対応のコマンドシステム

### 📚 学習記録システム（AI支援）
- **AI支援エラーマッチング**: レーベンシュタイン距離とパターン正規化を使用した類似エラー検索
- **ホットスポット検出**: 繰り返し問題が発生するファイルに対する事前警告
- **コンテキスト対応検索**: 複合スコアリング（ファイル40%、エラー30%、タグ20%、新規度10%）
- **組み込みナレッジベース**: TF-IDF駆動の検索、完全なUTF-8サポート
- **問題追跡**: インテリジェントな優先順位付けによる未解決問題の追跡
- **学習分析**: 詳細な内訳を含む統計とパターンの表示

一般的な開発CLIとは異なり、cldevには**AI支援**の検索可能な学習記録システムが含まれており、Claude Code統合向けに特別に設計されています。多くの開発者が別のツールでTIL（Today I Learned）リポジトリやエンジニアリングログを手動で管理していますが、cldevは以下を提供します：

**🔍 インテリジェントなエラーマッチング**
```bash
# AIがパターンを正規化して類似エラーを自動検出
cldev lr suggest "thread panicked at overflow in main.rs:42"
# マッチ: "thread panicked at overflow in lib.rs:123" (85%類似)
```

**⚠️ 予防的問題防止**
```bash
# 編集前にファイルのホットスポット状態を確認
cldev lr check-file src/auth/login.rs
# ⚠️  警告: このファイルは既知のホットスポットです！
# 過去の問題: JWT検証エラー（解決済み）、認証タイムアウト（未解決）
```

**🎯 コンテキストベースの発見**
```bash
# 現在のコンテキストに類似したセッションを検索
cldev lr similar session-abc123
# 類似したファイル、エラー、タグ、最近のアクティビティを持つセッションを返す
```

**基本的な使い方**
```bash
# 学習セッションを記録
cldev lr new "JWT認証の実装"

# 過去の解決策を検索
cldev lr find "authentication" --field topic

# 学習統計を表示
cldev lr stats --period week
```

---

## インストール

### オプション1: Cargo（Rustパッケージマネージャー）

```bash
# crates.ioからインストール（近日公開予定）
# cargo install cldev

# ソースからビルド
git clone https://github.com/sanae-abe/cldev.git
cd cldev
cargo install --path .
```

> **注意**: プリビルドバイナリとHomebrewインストールは将来のリリースで利用可能になります。詳細は[ロードマップ](docs/development/IMPLEMENTATION_PLAN.md)を参照してください。

### インストールの確認

```bash
cldev --version
# 出力: cldev 1.0.0
```

**📋 詳細な検証**: [ランタイムテストプラン](docs/development/RUNTIME_TEST_PLAN.md)を使用して、すべての機能が正しく動作していることを確認してください。

---

## クイックスタート

### 1. 設定の初期化（5分）

対話的なセットアップウィザードを実行：

```bash
cldev config init
```

以下を実行します：
- 言語設定の検出
- Claude Codeディレクトリの設定（`~/.claude`）
- プロジェクトルートの設定
- Git CLIツールの検出（gh/glab）
- シェル補完のインストール
- 設定ファイルの作成

**セッション例:**
```
cldev - 初期設定
━━━━━━━━━━━━━━━━━━━━━━━━━━

1. 言語 / Language
   > English / 日本語
   [日本語]

2. Claude Codeディレクトリ
   ✓ 検出: /Users/sanae/.claude

3. プロジェクトルート
   [~/projects]

4. Git CLI
   ✓ GitHub CLI (gh): 検出済み
   - GitLab CLI (glab): 見つかりません

5. シェル補完
   シェル: zsh
   追加先: ~/.zshrc

✓ 設定を保存: ~/.config/cldev/config.toml
✓ シェル補完を追加: ~/.zshrc

次: source ~/.zshrc
```

### 2. 設定の確認

```bash
cldev config check
# ✅ 設定ファイルは有効です
# 💡 次のステップ: cldev dev feature
```

### 3. 最初の機能開発を開始

```bash
cldev dev feature user-authentication
# 以下をガイドします：
# - ブランチ作成
# - 実装計画
# - テストスキャフォールディング
# - コミット準備
```

---

## 使用例

### 設定管理

```bash
# 設定の健全性チェック
cldev config check

# 詳細な検証付きチェック
cldev config check --detailed --validate

# エディタで設定を編集
cldev config edit

# 利用可能な全コマンドをリスト表示
cldev config list

# 詳細情報付きでコマンドをリスト表示
cldev config list --detailed

# カテゴリでコマンドをフィルタ
cldev config list --filter dev

# 月次メンテナンス（バックアップ、クリーンアップ、検証）
cldev config maintain --backup --cleanup
```

### 開発ワークフロー

```bash
# 新機能開発の開始
cldev dev feature payment-integration

# 本番環境の緊急問題対応
cldev dev urgent "API認証が失敗"

# 重大なバグ修正
cldev dev fix "ユーザーサービスのメモリリーク"

# 体系的なデバッグ
cldev dev debug "データベースクエリが遅い"

# 安全なリファクタリング
cldev dev refactor src/auth/

# パフォーマンス最適化
cldev dev optimize --focus "データベースクエリ"

# 調査とドキュメント化
cldev dev research "JWTベストプラクティス"
```

### Git操作

```bash
# 規約に沿ったコミット作成
cldev git commit "feat: OAuth2サポートを追加"

# 機能ブランチ作成（規約命名）
cldev git branch user-profile --type feature

# プルリクエスト作成（GitHub自動検出）
cldev git merge-request --title "ユーザー認証を追加"

# マージリクエスト作成（GitLab自動検出）
cldev git merge-request --title "メモリリークを修正"

# 推奨事項付きgitステータス表示
cldev git status --detailed
```

### コード品質

```bash
# リンター実行（プロジェクトタイプ自動検出）
cldev quality lint

# 自動修正付きリンター実行
cldev quality lint --fix

# コードフォーマット（Prettier/rustfmt/gofmt自動検出）
cldev quality format

# 変更なしでフォーマットチェック
cldev quality format --check

# テスト実行
cldev quality test

# 特定のテストパターンを実行
cldev quality test --pattern "auth*"

# カバレッジレポート付きで実行
cldev quality test --coverage

# 継続的テストのためのウォッチモード
cldev quality test --watch
```

### 技術スタック操作

```bash
# 開発サーバー起動（プロジェクトタイプ自動検出）
cldev tech start

# 特定のスタックで起動
cldev tech start web --port 3000
cldev tech start api --port 8080
cldev tech start mobile
cldev tech start ds  # データサイエンスノートブック

# デタッチモード
cldev tech start --detach
```

### 運用

```bash
# プロジェクトビルド（ビルドシステム自動検出）
cldev ops build

# バンドル分析付きビルド
cldev ops build --analyze

# クリーンビルド
cldev ops build --clean

# 環境へのデプロイ
cldev ops deploy production

# デプロイのドライラン
cldev ops deploy staging --dry-run

# 自動確認付きデプロイ
cldev ops deploy production --yes
```

### 分析・コードレビュー

```bash
# コードベース構造の分析
cldev analysis analyze --target structure

# パフォーマンス分析
cldev analysis analyze --target performance --detailed

# 例を含む技術概念の説明
cldev analysis explain "OAuth2フロー" --examples

# マージリクエストのレビュー
cldev analysis review-mr 42 --detailed

# セキュリティ重視のレビュー
cldev analysis review-mr 42 --security-focus

# パフォーマンス重視のレビュー
cldev analysis review-mr 42 --performance-focus

# セマンティック分析の実行（MCP統合）
cldev analysis serena --mode check
```

### 学習記録

```bash
# 新しい学習記録を作成
cldev lr new "Rustのライフタイムを理解する" --edit

# 学習記録を検索
cldev lr find "暗号化"

# 最近の記録
cldev lr find --recent 10

# 特定のフィールドで検索
cldev lr find "JWT" --field topic

# 編集前にファイルのホットスポット状態をチェック
cldev lr check-file src/auth/login.rs

# 類似エラーを検索（AI支援エラーマッチング）
cldev lr suggest "thread panicked at overflow" --threshold 0.7

# 特定のセッションに類似したセッションを検索
cldev lr similar session-abc123 --limit 5

# 統計を生成
cldev lr stats

# 週次統計
cldev lr stats --period week --detailed

# 問題パターンの分析
cldev lr problems

# 高優先度の問題
cldev lr problems --priority high --recent 20
```

### TODO管理

**Markdown形式（`todo.md`）による個人TODO管理**

```bash
# TODO項目を追加（対話式：優先度+タグ）
cldev todo manage add "レート制限を実装"

# 全TODOをリスト表示（優先度でグループ化）
cldev todo manage list

# TODOを完了（対話式選択）
cldev todo manage complete

# gitコミットと同期（マッチするTODOを自動完了）
cldev todo manage sync

# 対話モード（メニュー駆動）
cldev todo manage interactive
```

**機能:**
- ✅ Markdown形式（プロジェクトルートの `todo.md`） - あらゆるエディタで編集可能
- ✅ 優先度レベル（🔥 Critical、⚠️ High、📌 Medium、📝 Low）
- ✅ タグサポート（`#rust #performance`）
- ✅ Gitコミット統合（TODOの自動完了）
- ✅ プロジェクトローカルまたはグローバルストレージ

**todo.mdの例:**
```markdown
# Personal TODOs

## ⚠️ High
- [ ] Learning Record性能改善 #rust #performance (created: 2025-01-09)

## 📌 Medium
- [ ] TF-IDF検索精度向上 #search (created: 2025-01-09)

## ✅ Completed
- [x] READMEのコマンド数修正 (created: 2025-01-09, completed: 2025-01-09)
```

### シェル補完

```bash
# Zsh用の補完を生成
cldev completions zsh > ~/.zsh/completions/_cldev

# Bash用を生成
cldev completions bash > /usr/local/etc/bash_completion.d/cldev

# Fish用を生成
cldev completions fish > ~/.config/fish/completions/cldev.fish

# インストール手順を表示
cldev completions zsh --install
```

---

## 対応言語・技術スタック

### 言語

| 言語 | 検出 | リンティング | フォーマット | テスト |
|----------|-----------|---------|------------|---------|
| **JavaScript** | ✅ package.json | ESLint | Prettier | Jest/Vitest |
| **TypeScript** | ✅ tsconfig.json | ESLint | Prettier | Jest/Vitest |
| **Rust** | ✅ Cargo.toml | Clippy | rustfmt | cargo test |
| **Go** | ✅ go.mod | golangci-lint | gofmt/goimports | go test |
| **Python** | ✅ requirements.txt | pylint/ruff | black/ruff | pytest |
| **Ruby** | ✅ Gemfile | rubocop | rubocop | rspec |
| **Java** | ✅ pom.xml/build.gradle | checkstyle | google-java-format | JUnit |

### フレームワーク

| フレームワーク | 検出 | 開発サーバー | ビルド | デプロイ |
|-----------|-----------|------------|-------|--------|
| **React** | ✅ package.json | ✅ vite/next | ✅ | ✅ |
| **Vue** | ✅ package.json | ✅ vite | ✅ | ✅ |
| **Angular** | ✅ angular.json | ✅ ng serve | ✅ | ✅ |
| **Next.js** | ✅ next.config.js | ✅ next dev | ✅ | ✅ |
| **Express** | ✅ package.json | ✅ node | - | ✅ |
| **FastAPI** | ✅ requirements.txt | ✅ uvicorn | - | ✅ |
| **Rails** | ✅ Gemfile | ✅ rails s | - | ✅ |

### ビルドツール

- **Node.js**: npm、yarn、pnpm、bun
- **Rust**: cargo
- **Go**: go build、make
- **Python**: pip、poetry、pipenv
- **Java**: maven、gradle

### Gitプラットフォーム

- **GitHub**: `gh` CLIによる自動検出
- **GitLab**: `glab` CLIによる自動検出
- **リモート検出**: `.git/config`から自動検出

---

## 設定システム

### 3層階層

```
🌍 グローバル設定 (~/.config/cldev/config.toml)
    │ 全プロジェクトに適用される基本設定
    ▼
🔧 技術スタック設定 (~/.claude/stacks/*.md)
    │ 技術固有の設定（web/api/mobile/data-science）
    ▼
🎯 プロジェクト設定 (project/.claude/config.toml)
    │ プロジェクト固有の上書き
```

### 設定ファイル構造

**場所**:
- macOS: `~/Library/Application Support/cldev/config.toml`
- Linux: `~/.config/cldev/config.toml`
- Windows: `%APPDATA%\cldev\config.toml`

```toml
# cldev設定ファイル
version = "1.0.0"

[general]
language = "ja"  # en、ja、zh、またはzh-TW
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

### バージョン管理

cldevは設定ファイルにセマンティックバージョニングを使用します：

- **メジャーバージョン** (1.x.x): 破壊的変更、マイグレーション必須
- **マイナーバージョン** (x.1.x): 新機能、下位互換性あり
- **パッチバージョン** (x.x.1): バグ修正、完全互換

cldevは必要に応じて設定を自動的に検証および移行します。

---

## コマンドリファレンス

### コマンドカテゴリ

cldevは35コマンドを9つの論理カテゴリに整理します：

```
┌─────────────────────────────────────────────────────┐
│                   cldevコマンド                      │
├─────────────────────────────────────────────────────┤
│ config (6)     │ 設定管理                           │
│ dev (7)        │ 開発ワークフロー                    │
│ git (4)        │ Git操作                            │
│ quality (3)    │ コード品質・テスト                  │
│ tech (1)       │ 技術スタック操作                    │
│ ops (2)        │ ビルド・デプロイ                    │
│ analysis (4)   │ コード分析・レビュー                │
│ lr (7)         │ 学習記録                           │
│ todo (1)       │ タスク管理                         │
│ completions    │ シェル補完                         │
└─────────────────────────────────────────────────────┘
```

### 完全なコマンドリスト

#### Configコマンド (6)
```bash
cldev config init          # 対話的セットアップウィザード
cldev config check         # 設定を検証
cldev config edit          # エディタで設定を編集
cldev config list          # 全コマンドをリスト表示
cldev config maintain      # 月次メンテナンス
cldev config update-docs   # ドキュメント更新
```

#### Devコマンド (7)
```bash
cldev dev feature          # 新機能開発
cldev dev urgent           # 本番環境の緊急問題
cldev dev fix              # 重大なバグ修正
cldev dev debug            # 体系的デバッグ
cldev dev refactor         # 安全なリファクタリング
cldev dev optimize         # パフォーマンス最適化
cldev dev research         # 技術調査
```

#### Gitコマンド (4)
```bash
cldev git commit           # 規約に沿ったコミット
cldev git branch           # ブランチ作成
cldev git merge-request    # PR/MR作成（自動検出）
cldev git status           # 推奨事項付きステータス
```

#### Qualityコマンド (3)
```bash
cldev quality lint         # リンター実行
cldev quality format       # コードフォーマット
cldev quality test         # テスト実行
```

#### Techコマンド (1)
```bash
cldev tech start           # 開発環境起動（自動検出）
```

#### Opsコマンド (2)
```bash
cldev ops build            # プロジェクトビルド
cldev ops deploy           # 環境へデプロイ
```

#### Analysisコマンド (4)
```bash
cldev analysis analyze     # コード分析
cldev analysis explain     # 技術説明
cldev analysis review-mr   # MR/PRレビュー
cldev analysis serena      # セマンティック分析（MCP）
```

#### Learning Recordコマンド (7)
```bash
cldev lr new               # 学習記録作成
cldev lr find              # 記録検索
cldev lr stats             # 統計生成
cldev lr problems          # 問題パターン分析
cldev lr check-file        # ファイルホットスポット状態チェック
cldev lr suggest           # 類似エラー検索
cldev lr similar           # 類似セッション検索
```

#### Todoコマンド (1)
```bash
cldev todo manage          # TODO項目管理
```

#### シェル補完
```bash
cldev completions <shell>  # 補完生成
```

### グローバルオプション

全コマンドが以下のグローバルフラグをサポート：

```bash
--verbose, -v      # 詳細出力
--quiet, -q        # エラー以外の出力を抑制
--no-color         # カラー出力を無効化
--lang <LANG>      # 言語を上書き（en/ja/zh/zh-TW）
--help, -h         # ヘルプを表示
--version, -V      # バージョンを表示
```

---

## 開発

### 前提条件

- **Rust 1.83+**（[rustup](https://rustup.rs/)経由でインストール）
- **Git 2.30+**
- オプション: `gh`（GitHub CLI）、`glab`（GitLab CLI）

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/sanae-abe/cldev.git
cd cldev

# Gitフックのセットアップ（オプション、コントリビューター推奨）
./scripts/setup-git-hooks.sh

# デバッグモードでビルド
cargo build

# 最適化されたリリースバイナリをビルド
cargo build --release

# ローカルにインストール
cargo install --path .
```

### テストの実行

```bash
# 全テストを実行
cargo test

# 出力付きで実行
cargo test -- --nocapture

# 特定のテストを実行
cargo test test_config_load

# 統合テストのみ実行
cargo test --test '*'

# カバレッジレポート生成（cargo-tarpaulin必須）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### コード品質

```bash
# コードをフォーマット
cargo fmt

# フォーマットをチェック
cargo fmt -- --check

# リンターを実行
cargo clippy

# 厳格なチェックでリンターを実行
cargo clippy --all-targets --all-features -- -D warnings

# セキュリティ監査
cargo audit
```

### ベンチマーク

```bash
# 全ベンチマークを実行
cargo bench

# 特定のベンチマークを実行
cargo bench config_bench
```

#### 起動時間比較（実測値）

macOS 14.6、Apple M2 Proで[hyperfine](https://github.com/sharkdp/hyperfine)を使用して測定：

| ツール | 平均 | 最小 | 最大 | vs cldev |
|------|------|-----|-----|----------|
| **cldev** | 21.2ms ± 8.3ms | 11.3ms | 41.0ms | 1.0x（ベースライン） |
| **gh CLI** | 31.8ms ± 1.5ms | 29.0ms | 34.1ms | 1.5倍遅い |
| **glab** | 126.3ms ± 13.1ms | 111.5ms | 149.9ms | 6.0倍遅い |

**バイナリサイズ:**
- cldev: 3.3MB（strip済み、LTO最適化）
- gh CLI: 51MB

### プロジェクト構造

```
cldev/
├── Cargo.toml              # パッケージマニフェスト
├── Cargo.lock              # 依存関係ロックファイル
├── README.md               # プロジェクト概要（英語版）
├── README.ja.md            # プロジェクト概要（このファイル）
├── CONTRIBUTING.md         # コントリビューションガイドライン
├── CHANGELOG.md            # バージョン履歴
├── src/
│   ├── main.rs             # バイナリエントリポイント
│   ├── lib.rs              # ライブラリエクスポート
│   ├── cli/                # CLI引数解析と出力
│   │   ├── mod.rs          # CLIモジュールエクスポート
│   │   ├── args.rs         # コマンド定義（clap）
│   │   └── output.rs       # 出力フォーマット + i18n統合
│   ├── commands/           # コマンド実装
│   │   ├── mod.rs          # コマンドモジュールエクスポート
│   │   └── config/         # 設定コマンド
│   │       ├── mod.rs      # Configコマンドエクスポート
│   │       ├── init.rs     # 対話的初期化
│   │       ├── check.rs    # 設定検証
│   │       └── list.rs     # 設定表示
│   ├── core/               # コア機能
│   │   ├── mod.rs          # コアモジュールエクスポート
│   │   ├── config.rs       # 設定管理
│   │   ├── i18n.rs         # 国際化
│   │   ├── error.rs        # エラー型
│   │   └── security.rs     # セキュリティユーティリティ
│   └── i18n/               # i18nリソース
│       └── messages.json   # 翻訳カタログ（621キー、4言語）
├── tests/                  # 統合テスト
│   └── integration_test.rs # 統合テスト
├── examples/               # 使用例
│   └── i18n_demo.rs        # i18nデモンストレーション
├── completions/            # シェル補完スクリプト（生成）
│   ├── cldev.bash
│   ├── cldev.zsh
│   ├── cldev.fish
│   └── _cldev.ps1
└── docs/                   # ドキュメント
    ├── USER_GUIDE.md       # 完全なユーザードキュメント
    ├── DEVELOPER_GUIDE.md  # 開発者・コントリビューターガイド
    ├── guides/             # ユーザーガイド・チュートリアル
    ├── architecture/       # アーキテクチャ・設計
    ├── implementation/     # 実装詳細
    └── development/        # 開発計画
```

---

## コントリビューション

コントリビューションを歓迎します！詳細は[コントリビューションガイド](CONTRIBUTING.md)を参照してください。

### 開発ワークフロー

1. リポジトリを**フォーク**
2. フィーチャーブランチを**作成**（`git checkout -b feature/amazing-feature`）
3. 規約に沿った**コミット**（`git commit -m 'feat: add amazing feature'`）
4. フォークに**プッシュ**（`git push origin feature/amazing-feature`）
5. プルリクエストを**オープン**

### コミット規約

[Conventional Commits](https://www.conventionalcommits.org/)に従います：

```
feat: 新機能追加
fix: バグ修正
docs: ドキュメント更新
style: コードフォーマット
refactor: リファクタリング
perf: パフォーマンス改善
test: テスト追加
chore: 依存関係更新
```

### コードレビュープロセス

1. **自動チェック**: CIが合格必須（テスト、リント、フォーマット）
2. **セキュリティレビュー**: セキュリティ上の影響を全コードレビュー
3. **パフォーマンスレビュー**: パフォーマンスリグレッションなし
4. **ドキュメント**: 全パブリックAPIをドキュメント化
5. **テスト**: 新機能にテスト必須

---

## ライセンス

このプロジェクトはデュアルライセンスです：

- **MITライセンス**（[LICENSE-MIT](LICENSE-MIT)またはhttp://opensource.org/licenses/MIT）
- **Apache License 2.0**（[LICENSE-APACHE](LICENSE-APACHE)またはhttp://www.apache.org/licenses/LICENSE-2.0）

使用時にどちらかのライセンスを選択できます。

### コントリビューション

明示的に別段の定めがない限り、Apache-2.0ライセンスで定義されているように、作品に含めるために意図的に提出されたコントリビューションは、追加の条件なしに上記のようにデュアルライセンスされるものとします。

---

## ドキュメント

### クイックリンク

- **[ユーザーガイド](docs/USER_GUIDE.md)**: 完全なユーザードキュメント
- **[開発者ガイド](docs/DEVELOPER_GUIDE.md)**: コントリビューション・開発ガイド
- **[クイックスタート](docs/guides/QUICKSTART.md)**: 5分で始める
- **[コントリビューション](CONTRIBUTING.md)**: コントリビューション方法
- **[変更履歴](CHANGELOG.md)**: バージョン履歴と変更

### ドキュメント構造

```
docs/
├── USER_GUIDE.md              # 完全なユーザードキュメント
├── DEVELOPER_GUIDE.md         # 開発者・コントリビューターガイド
├── guides/                    # ユーザーガイド・チュートリアル
│   ├── QUICKSTART.md          # クイックスタートガイド
│   ├── CONFIG_USAGE_EXAMPLES.md    # 設定例
│   ├── INTERACTIVE_UI_DEMO.md      # 対話的UIウォークスルー
│   ├── i18n_quick_start.md         # i18nクイックスタート
│   ├── CORE_MODULES_QUICK_REFERENCE.md  # コアモジュールリファレンス
│   └── SUPPORTED_LANGUAGES.md      # 言語サポート
├── architecture/              # アーキテクチャ・設計
│   ├── i18n.md                # 国際化システム
│   ├── hierarchical-config-system.md  # 設定アーキテクチャ
│   ├── TECH_STACK_COMPARISON.md       # 技術分析
│   ├── RUST_BEST_PRACTICES_REVIEW.md  # Rustベストプラクティス
│   ├── SECURITY_IMPLEMENTATION.md     # セキュリティ設計
│   └── COMMAND_OPTIMIZATION_ANALYSIS.md  # コマンド最適化
├── implementation/            # 実装詳細
│   ├── DEVELOPMENT_HISTORY.md        # 完全な開発履歴
│   ├── COMMANDS_IMPLEMENTED.md       # コマンド実装状況
│   ├── IMPLEMENTATION_SUMMARY.md     # 実装サマリー
│   └── CORE_MODULES_IMPLEMENTATION.md  # コアモジュール詳細
└── development/              # 開発計画
    ├── IMPLEMENTATION_PLAN.md  # 実装ロードマップ
    ├── TODO.md                 # タスク追跡
    └── GTM_BUSINESS_STRATEGY.md  # Go-to-market戦略
```

### 役割別

**ユーザー向け:**
- [クイックスタートガイド](docs/guides/QUICKSTART.md)から始める
- 完全なドキュメントは[ユーザーガイド](docs/USER_GUIDE.md)を参照
- セットアップは[設定例](docs/guides/CONFIG_USAGE_EXAMPLES.md)を確認

**コントリビューター向け:**
- [コントリビューションガイドライン](CONTRIBUTING.md)を読む
- [開発者ガイド](docs/DEVELOPER_GUIDE.md)を学習
- [開発履歴](docs/implementation/DEVELOPMENT_HISTORY.md)をレビュー

**アーキテクト向け:**
- [アーキテクチャドキュメント](docs/architecture/)をレビュー
- [実装計画](docs/development/IMPLEMENTATION_PLAN.md)を学習
- [セキュリティ実装](docs/architecture/SECURITY_IMPLEMENTATION.md)を確認

---

## サポート

- **Issues**: [GitHub Issues](https://github.com/sanae-abe/cldev/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- **ドキュメント**: [docs/](docs/)

---

## 謝辞

以下を使用して構築：
- [clap](https://github.com/clap-rs/clap) - コマンドライン引数解析
- [serde](https://github.com/serde-rs/serde) - シリアライゼーションフレームワーク
- [tokio](https://tokio.rs/) - 非同期ランタイム
- [anyhow](https://github.com/dtolnay/anyhow) - エラーハンドリング
- [Cargo.toml](Cargo.toml)内のその他の素晴らしいRustクレート

モダンなCLIツールからインスパイア：
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [exa](https://github.com/ogham/exa)
- [bat](https://github.com/sharkdp/bat)
- [fd](https://github.com/sharkdp/fd)

---

**cldevチームが❤️を込めて作成**

*統一されたインテリジェントなCLIワークフローで開発者をエンパワーメント*
