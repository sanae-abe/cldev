# cldev CLI Testing with cli-testing-specialist

**補完的自動テストツール** - 既存のCLIテスト(`cli-testing.yml`)に加えて、自動化された包括的テストを提供

---

## 📑 目次

- [概要](#概要)
- [セットアップ](#セットアップ)
- [ローカルでのテスト実行](#ローカルでのテスト実行)
- [CI/CD統合](#cicd統合)
- [既存テストとの関係](#既存テストとの関係)
- [トラブルシューティング](#トラブルシューティング)

---

## 概要

cli-testing-specialist は cldev CLI の品質を自動検証する補完的なテストフレームワークです。

### 既存のCLIテストとの違い

| 項目 | 既存CLIテスト (`cli-testing.yml`) | cli-testing-specialist |
|------|----------------------------------|----------------------|
| **テスト方法** | 手動作成の統合テスト | 自動生成されたBATSテスト |
| **カバレッジ** | 重要機能の厳密なテスト | 広範囲の網羅的テスト |
| **保守性** | コード変更時に手動更新 | CLI変更時に自動再生成 |
| **焦点** | 機能の正確性・i18n・回帰テスト | セキュリティ・入力検証・エッジケース |
| **実行頻度** | PR・プッシュ時 | 日次スケジュール実行推奨 |

### 主な機能

- ✅ **自動解析**: cldev のオプション・サブコマンドを自動抽出
- ✅ **包括テスト**: 8カテゴリ 45-47 テストケースを自動生成
- ✅ **セキュリティ**: OWASP準拠のセキュリティスキャン
- ✅ **CI/CD統合**: GitHub Actions で自動実行
- ✅ **4種類レポート**: Markdown, JSON, HTML, JUnit XML

---

## セットアップ

### 1. 前提条件

```bash
# Rust (stable)
rustc --version  # 1.70.0+

# BATS (テスト実行用)
## macOS
brew install bats-core

## Ubuntu/Debian
sudo apt-get install bats

# jq (レポート表示用、オプション)
brew install jq  # macOS
sudo apt-get install jq  # Ubuntu
```

### 2. cli-testing-specialist のインストール

```bash
# GitHubから最新版をインストール
cargo install --git https://github.com/sanae-abe/cli-testing-specialist --tag v1.0.2 cli-testing-specialist

# インストール確認
cli-testing-specialist --version
# cli-testing-specialist 1.0.2
```

---

## ローカルでのテスト実行

### クイックスタート（3ステップ）

```bash
# 1. cldev をビルド
cargo build --release

# 2. CLI解析 + テスト生成 + 実行（一括）
cli-testing-specialist analyze target/release/cldev -o cldev-analysis.json
cli-testing-specialist generate cldev-analysis.json -o cldev-tests -c all
cli-testing-specialist run cldev-tests -f all -o reports

# 3. レポート確認
open reports/cldev-tests-report.html  # macOS
# または
cat reports/cldev-tests-report.md
```

### 詳細手順

#### Step 1: CLI解析

```bash
# cldev の構造を解析
cli-testing-specialist analyze \
  target/release/cldev \
  --output cldev-analysis.json

# 解析結果確認
jq -r '.binary_name + " v" + .version' cldev-analysis.json
jq '.global_options | length' cldev-analysis.json  # オプション数
jq '.subcommands | length' cldev-analysis.json     # サブコマンド数
```

#### Step 2: テスト生成

```bash
# 全カテゴリのテストを生成（デフォルト: directory-traversal除外）
cli-testing-specialist generate \
  cldev-analysis.json \
  --output cldev-tests \
  --categories all

# 生成されたテストファイル確認
ls -lh cldev-tests/
# basic.bats
# security.bats
# input-validation.bats
# ...
```

#### Step 3: テスト実行

```bash
# 全フォーマットでレポート生成
cli-testing-specialist run \
  cldev-tests \
  --format all \
  --output reports \
  --timeout 60

# 生成されたレポート
ls -lh reports/
# cldev-tests-report.html  # ブラウザで表示
# cldev-tests-report.json  # CI/CD連携
# cldev-tests-report.md    # GitHubで表示
# cldev-tests-junit.xml    # JUnit統合
```

### 特定カテゴリのみ実行

```bash
# セキュリティテストのみ
cli-testing-specialist generate \
  cldev-analysis.json \
  -o security-tests \
  -c security,input-validation

cli-testing-specialist run \
  security-tests \
  -f markdown,json \
  -o security-reports
```

---

## CI/CD統合

### GitHub Actions 設定

`.github/workflows/cli-testing-specialist.yml` が自動で設定されています。

**特徴**:
- ✅ Ubuntu/macOS マトリックステスト
- ✅ セキュリティ専用ジョブ
- ✅ テスト失敗時にCI fail
- ✅ レポートアーティファクト保存（30日間）
- ✅ 日次スケジュール実行（00:00 UTC）

### 実行スケジュール

```yaml
on:
  push:
    branches: [main, develop]    # プッシュ時
  pull_request:
    branches: [main, develop]    # PR時
  schedule:
    - cron: '0 0 * * *'           # 日次00:00 UTC
  workflow_dispatch:              # 手動実行
```

### CI実行確認

```bash
# ローカルでCI再現
cargo build --release
cli-testing-specialist analyze target/release/cldev -o analysis.json
cli-testing-specialist generate analysis.json -o tests -c all
cli-testing-specialist run tests -f all -o reports --timeout 60

# 結果確認
jq '.success_rate' reports/cldev-tests-report.json
```

---

## 既存テストとの関係

### 補完関係

```
既存CLIテスト (cli-testing.yml)
├── 機能の正確性テスト（手動作成）
│   ├── バージョン表示
│   ├── ヘルプコマンド
│   ├── config コマンド群
│   └── i18n 4言語対応
├── 回帰テスト（バグ修正の検証）
│   ├── Serena UTF-8エラー
│   ├── i18n Japanese表示
│   └── i18n Serena表示
└── パフォーマンステスト
    ├── 起動時間 < 100ms
    └── バイナリサイズ < 5MB

cli-testing-specialist (cli-testing-specialist.yml)
├── セキュリティテスト（自動生成）
│   ├── コマンドインジェクション
│   ├── パストラバーサル
│   ├── NULL byteインジェクション
│   └── TOCTOU攻撃
├── 入力検証テスト
│   ├── 数値オプション検証
│   ├── パスオプション検証
│   └── 列挙型オプション検証
└── エッジケーステスト
    ├── 特殊文字パス
    ├── 深い階層
    └── Unicode処理
```

### 推奨テスト戦略

1. **開発時**: 既存CLIテスト（手動作成）で重要機能を厳密にテスト
2. **PR時**: 両方のテストを実行（包括的品質保証）
3. **日次**: cli-testing-specialist でセキュリティ・エッジケースを継続監視

---

## テストカテゴリ

| カテゴリ | テスト内容 | テスト数 | デフォルト |
|---------|-----------|---------|----------|
| **basic** | ヘルプ、バージョン、終了コード | 10 | ✅ |
| **help** | 全サブコマンドヘルプ | 動的 | ✅ |
| **security** | インジェクション、機密漏洩、TOCTOU | 25 | ✅ |
| **path** | 特殊文字パス、深い階層、Unicode | 20 | ✅ |
| **multi-shell** | bash/zsh互換性 | 12 | ✅ |
| **input-validation** | 数値/パス/列挙型オプション検証 | 25 | ✅ |
| **destructive-ops** | 確認プロンプト、--yes/--force | 16 | ✅ |
| **performance** | 起動時間、メモリ使用量 | 6 | ✅ |
| **directory-traversal** | 大量ファイル、深い階層、シンボリックリンクループ | 12 | ❌ |

**デフォルト**: 8カテゴリ（45-47テスト）
**--include-intensive**: 9カテゴリ（53-55テスト）

---

## トラブルシューティング

### BATS テスト失敗

```bash
# 個別に BATS ファイルを実行
bats cldev-tests/security.bats

# 詳細ログ付き
bats -t cldev-tests/security.bats
```

### タイムアウトエラー

```bash
# タイムアウトを延長（デフォルト: 60秒）
cli-testing-specialist run cldev-tests -f json -o reports --timeout 120
```

### CI でのテスト失敗

```bash
# GitHub Actions ログから該当箇所確認
# Artifacts から cli-test-reports-ubuntu-latest をダウンロード
# cldev-tests-report.md を確認

# ローカルで再現
cargo build --release
cli-testing-specialist analyze target/release/cldev -o analysis.json
cli-testing-specialist generate analysis.json -o tests -c all
cli-testing-specialist run tests -f json -o reports
```

---

## FAQ

### Q1: 既存のCLIテストと競合しませんか？

**A**: 競合しません。補完関係です:
- 既存テスト: 重要機能の厳密な検証（手動作成・高精度）
- cli-testing-specialist: 広範囲の網羅的テスト（自動生成・高カバレッジ）

### Q2: どちらのテストを優先すべきですか？

**A**: 両方重要ですが、優先度は:
1. **開発時**: 既存CLIテスト（機能の正確性）
2. **リリース前**: 両方（包括的品質保証）
3. **セキュリティ重視**: cli-testing-specialist（OWASPスキャン）

### Q3: テスト生成にどれくらい時間がかかりますか？

**A**: cldev の場合:
- 解析: 100-200ms
- テスト生成: 1-2秒
- テスト実行: 30-60秒（カテゴリ数による）

---

## 参考リンク

- **cli-testing-specialist**: https://github.com/sanae-abe/cli-testing-specialist
- **BATS**: https://github.com/bats-core/bats-core
- **cldev**: https://github.com/sanae-abe/cldev
- **既存CLIテストガイド**: [CLI-TESTING-GUIDE.md](CLI-TESTING-GUIDE.md)
