# cldev - Claude Development CLI

[![Build Status](https://github.com/sanae-abe/cldev/workflows/CI/badge.svg)](https://github.com/sanae-abe/cldev/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-brightgreen.svg)](Cargo.toml)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**cldev**は、Claude Codeでの開発ワークフローを管理する統合CLIツールです。32の重要な開発コマンドを、型安全で超高速なRustバイナリに統合し、完全な多言語対応（英語・日本語）を提供します。

[English](README.md) | 日本語

<!-- スクリーンショットプレースホルダー: cldev config init のデモGIFを追加 -->

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
- [貢献](#貢献)
- [ライセンス](#ライセンス)

---

## 概要

**cldev**は、断片化したシェルスクリプトを統合された開発ツールキットに置き換えます：

- **統合**: 9カテゴリ32コマンドを単一ツールに（config、dev、git、quality、tech、ops、analysis、learning、todo）
- **高速化**: 起動時間90%高速化（5-10ms vs 50-100ms bash）
- **簡単**: インストール簡単（`cargo install cldev` または `brew install cldev`）
- **多言語**: 全出力を多言語化（英語・日本語、拡張可能なi18nシステム）
- **セキュア**: セキュアな操作（パストラバーサル防止、コマンドインジェクション保護）
- **自動検出**: プロジェクトタイプ自動検出（Node.js、Rust、Go、Python等）

### なぜcldev？

**Before（従来）:**
```bash
# 3つの異なるスクリプト集に分散したコマンド
~/.claude/scripts/claude validate
uc feature user-auth
~/.claude/learning-analytics/context-search.sh "encryption"
```

**After（cldev）:**
```bash
# インテリジェントなデフォルト設定を持つ単一の統合CLI
cldev config check
cldev dev feature user-auth
cldev lr find "encryption"
```

**改善点:**
- コマンド数22%削減（41 → 32）
- コマンド構文77%短縮
- インストール時間80%高速化
- 実行時間90%高速化
- 完全な型安全性（Rust）
- 完全なi18n対応

---

## 主な機能

### 🚀 パフォーマンス
- **超高速起動**: 5-10ms（bashの50-100msと比較）
- **最適化されたリリースビルド**: LTO、strip、codegen-units=1
- **効率的なリソース使用**: 最小限のメモリフットプリント

### 🌐 国際化
- **多言語サポート**: 英語（en）と日本語（ja）
- **自動検出**: `LANG`環境変数を使用
- **拡張可能**: JSONベースのi18nシステム（fluent-rsへアップグレード可能）

### 🔒 セキュリティ
- **パストラバーサル防止**: 安全なパス正規化
- **コマンドインジェクション保護**: 安全なコマンド実行
- **パーミッション検証**: 設定ファイルのセキュリティチェック（600）
- **入力検証**: 包括的なサニタイゼーション

### 🎯 開発者体験
- **シェル補完**: Bash、Zsh、Fish、PowerShell
- **対話的セットアップ**: ガイド付き設定ウィザード
- **スマート自動検出**: Gitリモート、プロジェクトタイプ、技術スタック
- **リッチな出力**: 色付き、整形済み、絵文字強化（設定可能）
- **包括的なヘルプ**: 全コマンドの詳細な`--help`

### 🏗️ アーキテクチャ
- **モジュラー設計**: 関心事の明確な分離
- **3層設定**: グローバル → 技術スタック → プロジェクト
- **型安全**: Rustのコンパイル時保証
- **拡張可能**: プラグイン対応コマンドシステム

---

## インストール

### オプション1: Cargo（Rustパッケージマネージャ）

```bash
# crates.ioからインストール
cargo install cldev

# ソースからビルド
git clone https://github.com/sanae-abe/cldev.git
cd cldev
cargo install --path .
```

### オプション2: Homebrew（macOS/Linux）

```bash
# tap追加（近日公開）
brew tap sanae-abe/cldev
brew install cldev
```

### オプション3: ビルド済みバイナリ

お使いのプラットフォーム用の最新リリースをダウンロード：

- [Linux x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [Linux aarch64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS x86_64](https://github.com/sanae-abe/cldev/releases/latest)
- [macOS aarch64 (Apple Silicon)](https://github.com/sanae-abe/cldev/releases/latest)
- [Windows x86_64](https://github.com/sanae-abe/cldev/releases/latest)

```bash
# 解凍してインストール
tar xzf cldev-*-x86_64-unknown-linux-gnu.tar.gz
sudo mv cldev /usr/local/bin/
```

### インストール確認

```bash
cldev --version
# 出力: cldev 1.0.0
```

**📋 詳細な動作確認**: [動作確認チェックリスト](docs/guides/VERIFICATION_CHECKLIST.md)で全機能の動作を確認できます。

---

## クイックスタート

### 1. 設定の初期化（5分）

対話的セットアップウィザードを実行：

```bash
cldev config init
```

これにより以下が実行されます：
- 言語設定の検出
- Claude Codeディレクトリの設定（`~/.claude`）
- プロジェクトルートの設定
- Git CLIツールの検出（gh/glab）
- シェル補完のインストール
- 設定ファイルの作成

**セッション例:**
```
cldev - 初期セットアップ
━━━━━━━━━━━━━━━━━━━━━━━━━━


1. Language / 言語
   > English / 日本語
   [日本語]

2. Claude Code ディレクトリ
   ✓ 検出: /Users/sanae/.claude

3. プロジェクトルート
   [~/projects]

4. Git CLI
   ✓ GitHub CLI (gh): 検出済み
   - GitLab CLI (glab): 見つかりません

5. シェル補完
   シェル: zsh
   追加先: ~/.zshrc

✓ 設定保存: ~/.config/cldev/config.toml
✓ シェル補完追加: ~/.zshrc

次のステップ: source ~/.zshrc
```

### 2. 設定の確認

```bash
cldev config check
# ✅ 設定ファイルは有効です
# 💡 次のステップ: cldev dev feature
```

### 3. 最初の機能開発

```bash
cldev dev feature user-authentication
# 以下をガイド:
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

# 詳細検証付きチェック
cldev config check --detailed --validate

# エディタで設定を編集
cldev config edit

# 全コマンド一覧
cldev config list

# 詳細情報付きでコマンド一覧
cldev config list --detailed

# カテゴリでフィルタ
cldev config list --filter dev

# 月次メンテナンス（バックアップ、クリーンアップ、検証）
cldev config maintain --backup --cleanup
```

### 開発ワークフロー

```bash
# 新機能開発開始
cldev dev feature payment-integration

# 緊急本番問題対応
cldev dev urgent "API認証が失敗しています"

# 重要なバグ修正
cldev dev fix "ユーザーサービスのメモリリーク"

# 体系的デバッグ
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
# Conventional Commit形式でコミット
cldev git commit "feat: OAuth2サポート追加"

# フィーチャーブランチ作成（慣習的な命名規則）
cldev git branch user-profile --type feature

# Pull Request作成（GitHub自動検出）
cldev git merge-request --title "ユーザー認証追加"

# Merge Request作成（GitLab自動検出）
cldev git merge-request --title "メモリリーク修正"

# 推奨アクション付きgit status
cldev git status --detailed
```

### コード品質

```bash
# リンター実行（プロジェクトタイプ自動検出）
cldev quality lint

# 自動修正付きリンター実行
cldev quality lint --fix

# コード整形（自動検出: Prettier/rustfmt/gofmt）
cldev quality format

# 変更なしで整形チェック
cldev quality format --check

# テスト実行
cldev quality test

# 特定のテストパターン実行
cldev quality test --pattern "auth*"

# カバレッジレポート付き実行
cldev quality test --coverage

# 継続的テストのウォッチモード
cldev quality test --watch
```

### 技術スタック操作

```bash
# 開発サーバー起動（プロジェクトタイプ自動検出）
cldev tech start

# 特定スタック起動
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

# 環境へデプロイ
cldev ops deploy production

# デプロイのドライラン
cldev ops deploy staging --dry-run

# 自動確認付きデプロイ
cldev ops deploy production --yes
```

### 分析・コードレビュー

```bash
# コードベース構造分析
cldev analysis analyze --target structure

# パフォーマンス分析
cldev analysis analyze --target performance --detailed

# 技術概念を例付きで説明
cldev analysis explain "OAuth2フロー" --examples

# マージリクエストレビュー
cldev analysis review-mr 42 --detailed

# セキュリティ重視レビュー
cldev analysis review-mr 42 --security-focus

# パフォーマンス重視レビュー
cldev analysis review-mr 42 --performance-focus

# セマンティック分析実行（MCP統合）
cldev analysis serena --mode check
```

### 学習記録

```bash
# 新規学習記録作成
cldev lr new "Rustライフタイムの理解" --edit

# 学習記録検索
cldev lr find "暗号化"

# 最近の記録
cldev lr find --recent 10

# 特定フィールドで検索
cldev lr find "JWT" --field topic

# 統計生成
cldev lr stats

# 週次統計
cldev lr stats --period week --detailed

# 問題パターン分析
cldev lr problems

# 高優先度の問題
cldev lr problems --priority high --recent 20
```

### Todo管理

```bash
# Todoアイテム追加
cldev todo manage add "レート制限実装"

# 全Todo一覧
cldev todo manage list

# Todo完了
cldev todo manage complete 3
```

### シェル補完

```bash
# Zsh用補完生成
cldev completions zsh > ~/.zsh/completions/_cldev

# Bash用生成
cldev completions bash > /usr/local/etc/bash_completion.d/cldev

# Fish用生成
cldev completions fish > ~/.config/fish/completions/cldev.fish

# インストール手順表示
cldev completions zsh --install
```

---

## 対応言語・技術スタック

### 言語

| 言語 | 検出 | リンティング | 整形 | テスト |
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

### 3層階層構造

```
🌍 グローバル設定 (~/.config/cldev/config.toml)
    │ 全プロジェクトに適用される基本設定
    ▼
🔧 技術スタック設定 (~/.claude/stacks/*.md)
    │ 技術固有の設定（web/api/mobile/data-science）
    ▼
🎯 プロジェクト設定 (project/.claude/config.toml)
    │ プロジェクト固有のオーバーライド
```

### 設定ファイル構造

**場所**: `~/.config/cldev/config.toml`

```toml
# cldev 設定ファイル
version = "1.0.0"

[general]
language = "ja"  # en または ja
claude_dir = "/Users/sanae.abe/.claude"
projects_dir = "/Users/sanae.abe/projects"

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
sessions_dir = "/Users/sanae.abe/.claude/learning-sessions"
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

cldevは設定ファイルにセマンティックバージョニングを使用：

- **メジャーバージョン**（1.x.x）: 破壊的変更、移行が必要
- **マイナーバージョン**（x.1.x）: 新機能、後方互換性あり
- **パッチバージョン**（x.x.1）: バグ修正、完全互換

cldevは必要に応じて設定を自動検証・移行します。

---

## コマンドリファレンス

### コマンドカテゴリ

cldevは32のコマンドを9つの論理カテゴリに整理：

```
┌─────────────────────────────────────────────────────┐
│                   cldev コマンド                    │
├─────────────────────────────────────────────────────┤
│ config (6)     │ 設定管理                           │
│ dev (7)        │ 開発ワークフロー                   │
│ git (4)        │ Git操作                            │
│ quality (3)    │ コード品質・テスト                 │
│ tech (1)       │ 技術スタック操作                   │
│ ops (2)        │ ビルド・デプロイ                   │
│ analysis (4)   │ コード分析・レビュー               │
│ lr (4)         │ 学習記録                           │
│ todo (1)       │ タスク管理                         │
└─────────────────────────────────────────────────────┘
```

### 完全なコマンド一覧

#### Configコマンド（6個）
```bash
cldev config init          # 対話的セットアップウィザード
cldev config check         # 設定検証
cldev config edit          # エディタで設定編集
cldev config list          # 全コマンド一覧
cldev config maintain      # 月次メンテナンス
cldev config update-docs   # ドキュメント更新
```

#### Devコマンド（7個）
```bash
cldev dev feature          # 新機能開発
cldev dev urgent           # 緊急本番問題
cldev dev fix              # 重要バグ修正
cldev dev debug            # 体系的デバッグ
cldev dev refactor         # 安全なリファクタリング
cldev dev optimize         # パフォーマンス最適化
cldev dev research         # 技術調査
```

#### Gitコマンド（4個）
```bash
cldev git commit           # Conventional Commit
cldev git branch           # ブランチ作成
cldev git merge-request    # PR/MR作成（自動検出）
cldev git status           # 推奨アクション付きステータス
```

#### Qualityコマンド（3個）
```bash
cldev quality lint         # リンター実行
cldev quality format       # コード整形
cldev quality test         # テスト実行
```

#### Techコマンド（1個）
```bash
cldev tech start           # 開発環境起動（自動検出）
```

#### Opsコマンド（2個）
```bash
cldev ops build            # プロジェクトビルド
cldev ops deploy           # 環境へデプロイ
```

#### Analysisコマンド（4個）
```bash
cldev analysis analyze     # コード分析
cldev analysis explain     # 技術説明
cldev analysis review-mr   # MR/PRレビュー
cldev analysis serena      # セマンティック分析（MCP）
```

#### 学習記録コマンド（4個）
```bash
cldev lr new               # 学習記録作成
cldev lr find              # 記録検索
cldev lr stats             # 統計生成
cldev lr problems          # 問題パターン分析
```

#### Todoコマンド（1個）
```bash
cldev todo manage          # Todoアイテム管理
```

#### シェル補完
```bash
cldev completions <shell>  # 補完生成
```

### グローバルオプション

全コマンドで以下のグローバルフラグをサポート：

```bash
--verbose, -v      # 詳細出力
--quiet, -q        # エラー以外の出力を抑制
--no-color         # 色付き出力を無効化
--lang <LANG>      # 言語オーバーライド（en/ja）
--help, -h         # ヘルプ表示
--version, -V      # バージョン表示
```

---

## 開発

### 前提条件

- **Rust 1.70+**（[rustup](https://rustup.rs/)でインストール）
- **Git 2.30+**
- オプション: `gh`（GitHub CLI）、`glab`（GitLab CLI）

### ソースからビルド

```bash
# リポジトリクローン
git clone https://github.com/sanae-abe/cldev.git
cd cldev

# デバッグモードでビルド
cargo build

# 最適化されたリリースバイナリビルド
cargo build --release

# ローカルインストール
cargo install --path .
```

### テスト実行

```bash
# 全テスト実行
cargo test

# 出力付きで実行
cargo test -- --nocapture

# 特定テスト実行
cargo test test_config_load

# 統合テストのみ実行
cargo test --test '*'

# カバレッジレポート生成（cargo-tarpaulin必要）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### コード品質

```bash
# コード整形
cargo fmt

# 整形チェック
cargo fmt -- --check

# リンター実行
cargo clippy

# 厳格チェック付きリンター実行
cargo clippy --all-targets --all-features -- -D warnings

# セキュリティ監査
cargo audit
```

### ベンチマーク

```bash
# 全ベンチマーク実行
cargo bench

# 特定ベンチマーク実行
cargo bench config_bench
```

---

## 貢献

貢献を歓迎します！詳細は[Contributing Guide](CONTRIBUTING.md)をご覧ください。

### 開発ワークフロー

1. リポジトリを**フォーク**
2. フィーチャーブランチを**作成**（`git checkout -b feature/amazing-feature`）
3. Conventional Commitsで**コミット**（`git commit -m 'feat: 素晴らしい機能追加'`）
4. フォークに**プッシュ**（`git push origin feature/amazing-feature`）
5. Pull Requestを**作成**

### コミット規約

[Conventional Commits](https://www.conventionalcommits.org/)に従います：

```
feat: 新機能追加
fix: バグ修正
docs: ドキュメント更新
style: コード整形
refactor: リファクタリング
perf: パフォーマンス改善
test: テスト追加
chore: 依存関係更新
```

---

## ライセンス

このプロジェクトはデュアルライセンスです：

- **MIT License**（[LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT）
- **Apache License 2.0**（[LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0）

どちらかのライセンスを選択できます。

---

## ドキュメント

### クイックリンク

- **[ユーザーガイド](docs/USER_GUIDE.md)**: 完全なユーザードキュメント
- **[開発者ガイド](docs/DEVELOPER_GUIDE.md)**: 貢献・開発ガイド
- **[クイックスタート](docs/guides/QUICKSTART.md)**: 5分で始める
- **[貢献方法](CONTRIBUTING.md)**: 貢献の仕方
- **[変更履歴](CHANGELOG.md)**: バージョン履歴

---

## サポート

- **Issue**: [GitHub Issues](https://github.com/sanae-abe/cldev/issues)
- **Discussion**: [GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- **ドキュメント**: [docs/](docs/)

---

## 謝辞

以下を使用して構築：
- [clap](https://github.com/clap-rs/clap) - コマンドライン引数解析
- [serde](https://github.com/serde-rs/serde) - シリアライゼーションフレームワーク
- [tokio](https://tokio.rs/) - 非同期ランタイム
- [anyhow](https://github.com/dtolnay/anyhow) - エラー処理
- その他素晴らしいRustクレート（[Cargo.toml](Cargo.toml)参照）

以下の現代的CLIツールにインスパイア：
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [exa](https://github.com/ogham/exa)
- [bat](https://github.com/sharkdp/bat)
- [fd](https://github.com/sharkdp/fd)

---

**cldevチームが❤️を込めて作成**

*統合されたインテリジェントなCLIワークフローで開発者をエンパワー*
