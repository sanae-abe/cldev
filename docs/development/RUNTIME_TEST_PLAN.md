# cldev 実機動作テスト計画書（統合版）

**バージョン**: 1.0.0
**対象リリース**: Phase 7完了（v1.0.0）
**最終更新**: 2025-01-09
**テスト実施者**: _________________
**実施日時**: _________________

---

## 📋 目次

- [テスト概要](#テスト概要)
- [テスト環境準備](#テスト環境準備)
- [P0: 必須テスト（30分）](#p0-必須テスト)
- [P1: 重要テスト（20分）](#p1-重要テスト)
- [P2: 推奨テスト（15分）](#p2-推奨テスト)
- [テスト結果記録](#テスト結果記録)
- [トラブルシューティング](#トラブルシューティング)

---

## 🎯 テスト概要

### 対象機能

**Phase 7完了機能** ⭐:
- セッション自動記録（session start/end/status）
- 機密情報マスキング（7パターン対応）
- 自動記録判定（5要素スコアリング）
- V3形式学習記録（YAML 10% + Markdown 90%）

**Phase 6完了機能**:
- 学習記録データベース（SQLite + FTS5）
- TF-IDF検索エンジン
- エラー類似度検索
- 7コマンド（new/find/stats/problems/check-file/suggest/similar）

**コア機能**:
- 35コマンド（9カテゴリ）
- 4言語対応（en/ja/zh/zh-TW）
- プロジェクト自動検出（12言語）
- Git統合

### テスト合格基準

| レベル | 条件 | 判定 |
|--------|------|------|
| **合格** | P0: 100%, P1: 90%以上 | ✅ リリース可能 |
| **条件付き合格** | P0: 100%, P1: 80%以上 | ⚠️ 軽微な問題のみ |
| **不合格** | P0 < 100% または重大バグ | ❌ 修正必須 |

### テスト実施時間

- **最小構成**: P0のみ（30分）
- **推奨構成**: P0 + P1（50分）
- **完全版**: P0 + P1 + P2（65分）

---

## 🔧 テスト環境準備

### 前提条件

```bash
# Rustバージョン確認
rustc --version
# 期待: rustc 1.70.0以上

# リリースビルド
cargo build --release

# バイナリ確認
ls -lh target/release/cldev
# 期待: 約1.5-2.0MB

# バージョン確認
./target/release/cldev --version
# 期待: cldev 1.0.0
```

**チェックリスト**:
- [ ] Rust 1.70.0以上
- [ ] リリースビルド成功（警告0件）
- [ ] バイナリサイズ 1.5-2.0MB
- [ ] バージョン表示OK

### テストデータ準備

```bash
# 既存データをバックアップ
mkdir -p ~/cldev-test-backup
cp -r ~/.cldev ~/cldev-test-backup/

# テスト用に初期化
rm -rf ~/.cldev/learning_records.db
rm -rf ~/.cldev/learning_records/*.md
mkdir -p ~/.cldev/learning_records

# セッション初期化
rm -rf ~/.cldev/sessions/
mkdir -p ~/.cldev/sessions/
```

**チェックリスト**:
- [ ] 既存データバックアップ完了
- [ ] テスト環境初期化完了

---

## ✅ P0: 必須テスト（30分）

### P0-1: 基本動作確認（5分）

#### 1.1 コマンド実行確認

```bash
# バージョン表示
./target/release/cldev --version

# ヘルプ表示
./target/release/cldev --help

# コマンド一覧
./target/release/cldev config list
```

**期待結果**:
- ✅ バージョン: cldev 1.0.0
- ✅ ヘルプが正しく表示
- ✅ 35コマンドが9カテゴリで表示

**チェックリスト**:
- [ ] バージョン表示OK
- [ ] ヘルプ表示OK
- [ ] コマンド一覧35個表示

#### 1.2 多言語機能確認

```bash
# 英語
./target/release/cldev --lang en --help

# 日本語
./target/release/cldev --lang ja --help

# 中国語（簡体字）
./target/release/cldev --lang zh --help

# 中国語（繁体字）
./target/release/cldev --lang zh-TW --help
```

**期待結果**:
- ✅ 4言語すべてで正しく表示

**チェックリスト**:
- [ ] 英語表示OK
- [ ] 日本語表示OK
- [ ] 簡体字表示OK
- [ ] 繁体字表示OK

---

### P0-2: 設定機能（5分）

#### 2.1 初期設定

```bash
# 設定初期化
./target/release/cldev config init
# 対話的にすべて選択（デフォルト値でOK）

# 設定確認
./target/release/cldev config check --detailed
```

**期待結果**:
- ✅ 設定ウィザードが起動
- ✅ 設定ファイルが作成される
- ✅ 検証が成功

**チェックリスト**:
- [ ] 設定ウィザード起動OK
- [ ] 設定ファイル作成OK
- [ ] 検証成功

#### 2.2 設定ファイル確認

```bash
# macOS
ls -la ~/Library/Application\ Support/cldev/config.toml

# Linux
ls -la ~/.config/cldev/config.toml

# パーミッション確認
stat -f %A ~/Library/Application\ Support/cldev/config.toml  # macOS
stat -c %a ~/.config/cldev/config.toml  # Linux
# 期待: 600
```

**チェックリスト**:
- [ ] 設定ファイル存在
- [ ] パーミッション600

---

### P0-3: Phase 7機能 - セッション記録（10分）⭐

#### 3.1 セッション開始

```bash
# セッション開始
./target/release/cldev session start "Test session for runtime verification"

# ステータス確認
./target/release/cldev session status
```

**期待結果**:
```
✅ Session started: session-20250109-HHMMSS
📝 Title: Test session for runtime verification
⏰ Start Time: 2025-01-09 HH:MM:SS
```

**チェックリスト**:
- [ ] セッション開始成功
- [ ] セッションID生成
- [ ] ステータス表示OK

#### 3.2 セッション中の操作

```bash
# いくつかコマンド実行（追跡対象）
./target/release/cldev config list
./target/release/cldev lr stats

# 再度ステータス確認
./target/release/cldev session status
```

**期待結果**:
- ✅ コマンド履歴が追跡される
- ✅ ステータスに反映

**チェックリスト**:
- [ ] コマンド追跡OK
- [ ] ステータス更新OK

#### 3.3 機密情報マスキング確認

セッション中に以下のような文字列を含む操作を想定:

```bash
# 環境変数にテスト用の機密情報を設定（実際には使用しない）
export TEST_API_KEY="sk-test-1234567890abcdef"
export TEST_PASSWORD="MyPassword123!"

# セッションに記録される可能性のある操作
echo "API_KEY: $TEST_API_KEY"  # このコマンド履歴が記録される

# セッション終了
./target/release/cldev session end --summary "Tested masking feature"
```

**期待結果**:
- セッションファイルを確認した際、以下がマスキングされている:
  - `sk-test-***` (APIキー)
  - `MyPassword***` (パスワード)

**セッションファイル確認**:
```bash
# 最新セッションファイル確認
cat $(ls -t ~/.cldev/sessions/*.md | head -1)
```

**チェックリスト**:
- [ ] APIキーがマスキング
- [ ] パスワードがマスキング
- [ ] セッション終了成功

#### 3.4 V3形式確認

```bash
# 最新セッションファイルの形式確認
cat $(ls -t ~/.cldev/sessions/*.md | head -1) | head -20
```

**期待結果**:
```markdown
---
session_id: session-20250109-HHMMSS
title: Test session for runtime verification
start_time: 2025-01-09T...
end_time: 2025-01-09T...
status: completed
---

# Test session for runtime verification

## Summary
Tested masking feature

## Commands Executed
- cldev config list
- cldev lr stats
...
```

**チェックリスト**:
- [ ] YAMLフロントマター（簡潔版）
- [ ] Markdown本文
- [ ] コマンド履歴記録

---

### P0-4: 学習記録基本機能（10分）

#### 4.1 学習記録作成

```bash
# テストデータ1: Rust エラー
./target/release/cldev lr new \
  --title "Rust borrow checker test" \
  --type debug \
  --description "cannot borrow as mutable" \
  --root-cause "immutable variable" \
  --solution "added mut keyword" \
  --learning "use mut for mutable variables" \
  --files "src/main.rs" \
  --tags "rust,borrow-checker" \
  --resolved

# テストデータ2: 未解決問題
./target/release/cldev lr new \
  --title "Integration test failure" \
  --type debug \
  --description "assertion failed" \
  --root-cause "investigating" \
  --solution "TBD" \
  --learning "need more debugging" \
  --files "tests/integration.rs" \
  --tags "rust,testing"
```

**期待結果**:
- ✅ 2件の学習記録が作成される
- ✅ データベースに保存される

**チェックリスト**:
- [ ] 学習記録1作成成功
- [ ] 学習記録2作成成功

#### 4.2 検索機能

```bash
# キーワード検索
./target/release/cldev lr find "rust"

# タグ検索
./target/release/cldev lr find --tag "borrow-checker"

# ファイル検索
./target/release/cldev lr find --file "src/main.rs"

# 未解決のみ
./target/release/cldev lr find --unresolved
```

**期待結果**:
- ✅ "rust": 2件ヒット
- ✅ "borrow-checker": 1件ヒット
- ✅ "src/main.rs": 1件ヒット
- ✅ 未解決: 1件ヒット

**チェックリスト**:
- [ ] キーワード検索OK（2件）
- [ ] タグ検索OK（1件）
- [ ] ファイル検索OK（1件）
- [ ] 未解決フィルタOK（1件）

#### 4.3 統計表示

```bash
./target/release/cldev lr stats
```

**期待結果**:
```
📊 Learning Records Statistics
  Total Sessions: 2
  Resolved: 1
  Unresolved: 1

📁 Files:
  src/main.rs: 1 session(s)
  tests/integration.rs: 1 session(s)

🏷️  Tags:
  rust: 2
  borrow-checker: 1
  testing: 1
```

**チェックリスト**:
- [ ] セッション数正確（2件）
- [ ] 解決/未解決正確（1/1）
- [ ] ファイル統計表示
- [ ] タグ統計表示

---

## 🔍 P1: 重要テスト（20分）

### P1-1: Phase 6高度検索機能（10分）

#### 1.1 ホットスポット警告

```bash
# テストデータ追加（src/main.rsの問題を増やす）
./target/release/cldev lr new \
  --title "Another src/main.rs issue" \
  --type debug \
  --description "compilation error" \
  --root-cause "syntax error" \
  --solution "fixed typo" \
  --learning "check syntax carefully" \
  --files "src/main.rs" \
  --tags "rust,compile-error" \
  --resolved

# ホットスポットチェック
./target/release/cldev lr check-file src/main.rs
```

**期待結果**:
```
⚠️  WARNING: This file is a HOTSPOT with past issues!

📊 Hotspot Statistics:
  • Sessions: 2
  • Avg Score: [数値]
  • Last Access: 2025-01-09

🔍 Recent Issues (last 5):
  1. Rust borrow checker test [Resolved]
  2. Another src/main.rs issue [Resolved]
```

**チェックリスト**:
- [ ] 警告メッセージ表示
- [ ] セッション数正確（2件）
- [ ] 問題一覧表示

#### 1.2 エラー類似検索

```bash
# 類似エラー検索
./target/release/cldev lr suggest "cannot borrow"
```

**期待結果**:
```
🔍 Found 1 similar problem(s):

1. [Score: 0.XX] Rust borrow checker test
   Description: cannot borrow as mutable
   Files: src/main.rs
   Status: ✅ Resolved

   Root Cause: immutable variable
   Solution: added mut keyword
```

**チェックリスト**:
- [ ] 類似エラー検出
- [ ] スコア表示
- [ ] 詳細情報表示

#### 1.3 類似セッション検索

```bash
# セッションID取得
SESSION_ID=$(./target/release/cldev lr find "borrow" --limit 1 | grep -o 'session-[0-9-]*' | head -1)

# 類似セッション検索
./target/release/cldev lr similar "$SESSION_ID"
```

**期待結果**:
- ✅ ターゲットセッション情報表示
- ✅ 類似セッション一覧表示
- ✅ スコア順ソート

**チェックリスト**:
- [ ] ターゲット情報表示
- [ ] 類似セッション表示
- [ ] スコアソート正常

---

### P1-2: プロジェクト検出（5分）

```bash
# Rustプロジェクトで実行（cldevプロジェクト自体）
cd /Users/sanae.abe/projects/cldev
./target/release/cldev quality lint --help

# 別のプロジェクトタイプで確認（可能なら）
# Node.jsプロジェクト
# cd /path/to/nodejs-project
# cldev quality lint --help
```

**期待結果**:
- ✅ Rustプロジェクト検出
- ✅ Clippy使用が提案される

**チェックリスト**:
- [ ] プロジェクトタイプ検出OK
- [ ] 適切なツール提案

---

### P1-3: Git統合（5分）

```bash
# Git状態確認
./target/release/cldev git status

# GitHub/GitLab CLI検出
./target/release/cldev config check --detailed | grep -A 5 "Git CLI"
```

**期待結果**:
- ✅ Git状態が表示される
- ✅ CLI検出状況が表示される

**チェックリスト**:
- [ ] Git状態表示OK
- [ ] CLI検出OK

---

## 📈 P2: 推奨テスト（15分）

### P2-1: パフォーマンス測定（5分）

#### 1.1 起動速度

```bash
# 起動時間測定（5回平均）
for i in {1..5}; do
  time ./target/release/cldev --version
done
```

**期待結果**:
- ✅ 起動時間: < 25ms

**チェックリスト**:
- [ ] 平均起動時間 < 25ms

#### 1.2 検索速度

```bash
# 検索速度測定
time ./target/release/cldev lr find "rust"
time ./target/release/cldev lr suggest "error"
time ./target/release/cldev lr similar "$SESSION_ID"
```

**期待結果**:
- ✅ find: < 100ms
- ✅ suggest: < 200ms
- ✅ similar: < 300ms

**チェックリスト**:
- [ ] find速度OK
- [ ] suggest速度OK
- [ ] similar速度OK

---

### P2-2: エラーハンドリング（5分）

```bash
# 引数なしエラー
./target/release/cldev lr check-file

# 不正なオプション
./target/release/cldev lr suggest "error" --invalid-option

# 存在しないセッションID
./target/release/cldev lr similar "nonexistent-id"
```

**期待結果**:
- ✅ 適切なエラーメッセージ
- ✅ ヘルプ表示

**チェックリスト**:
- [ ] 引数なしエラーOK
- [ ] 不正オプションエラーOK
- [ ] 存在しないIDエラーOK

---

### P2-3: シェル補完（5分）

```bash
# 補完スクリプト生成
./target/release/cldev completions bash > /tmp/cldev-completion.bash
cat /tmp/cldev-completion.bash | head -20

# 各シェル用生成確認
./target/release/cldev completions zsh > /tmp/cldev-completion.zsh
./target/release/cldev completions fish > /tmp/cldev-completion.fish
./target/release/cldev completions powershell > /tmp/cldev-completion.ps1
```

**期待結果**:
- ✅ 4種類の補完スクリプト生成成功

**チェックリスト**:
- [ ] Bash補完生成OK
- [ ] Zsh補完生成OK
- [ ] Fish補完生成OK
- [ ] PowerShell補完生成OK

---

## 📊 テスト結果記録表

### 実施情報

**実施日時**: _________________
**実施者**: _________________
**環境**:
- OS: _________________
- Rustバージョン: _________________
- バイナリサイズ: _________________

### 結果サマリー

| カテゴリ | テスト項目数 | 通過 | 失敗 | スキップ | 備考 |
|---------|-------------|------|------|----------|------|
| **P0-1** 基本動作 | 7 | ___ | ___ | ___ | |
| **P0-2** 設定 | 4 | ___ | ___ | ___ | |
| **P0-3** セッション | 11 | ___ | ___ | ___ | ⭐ Phase 7 |
| **P0-4** 学習記録 | 11 | ___ | ___ | ___ | |
| **P1-1** 高度検索 | 9 | ___ | ___ | ___ | Phase 6 |
| **P1-2** プロジェクト検出 | 2 | ___ | ___ | ___ | |
| **P1-3** Git統合 | 2 | ___ | ___ | ___ | |
| **P2-1** パフォーマンス | 4 | ___ | ___ | ___ | |
| **P2-2** エラー処理 | 3 | ___ | ___ | ___ | |
| **P2-3** シェル補完 | 4 | ___ | ___ | ___ | |
| **合計** | **57** | **___** | **___** | **___** | |

### P0必須テスト結果

- **P0通過率**: ___ / 33 (___%）
- **合格基準**: 33 / 33 (100%)
- **判定**: [ ] 合格 / [ ] 不合格

### P1重要テスト結果

- **P1通過率**: ___ / 13 (___%）
- **合格基準**: 12 / 13 (90%以上)
- **判定**: [ ] 合格 / [ ] 条件付き合格 / [ ] 不合格

### P2推奨テスト結果

- **P2通過率**: ___ / 11 (___%）
- **実施有無**: [ ] 実施 / [ ] スキップ

---

## 🐛 発見した問題の記録

### 問題1

- **重要度**: [ ] 高（P0失敗） [ ] 中（P1失敗） [ ] 低（P2失敗）
- **発生箇所**: _________________
- **現象**: _________________
- **再現手順**:
  1. _________________
  2. _________________
- **対処方針**: _________________

### 問題2

（必要に応じて追加）

---

## 🔧 トラブルシューティング

### よくある問題

#### 問題1: `cldev: command not found`

**解決方法**:
```bash
# PATHに追加
export PATH="$HOME/.cargo/bin:$PATH"

# またはフルパスで実行
./target/release/cldev --version
```

#### 問題2: セッションが開始できない

**原因**: セッションディレクトリが存在しない

**解決方法**:
```bash
mkdir -p ~/.cldev/sessions/
```

#### 問題3: 学習記録が保存されない

**原因**: データベースディレクトリの権限問題

**解決方法**:
```bash
chmod 755 ~/.cldev
chmod 644 ~/.cldev/learning_records.db
```

#### 問題4: 多言語表示が文字化け

**原因**: ロケール設定

**解決方法**:
```bash
export LANG=ja_JP.UTF-8
export LC_ALL=ja_JP.UTF-8
```

---

## ✅ 最終判定

### 合格条件

- [x] **P0必須テスト**: 33 / 33 (100%)
- [ ] **P1重要テスト**: 12 / 13 (90%以上)
- [ ] **重大バグなし**

### 判定結果

- [ ] **✅ 合格** - リリース可能
- [ ] **⚠️ 条件付き合格** - 軽微な修正後リリース
- [ ] **❌ 不合格** - 修正必須

### 次のアクション

**合格時**:
1. [ ] TODO.md更新（実機テスト完了）
2. [ ] README.md更新
3. [ ] crates.io公開準備

**不合格時**:
1. [ ] 問題修正
2. [ ] 再テスト実施
3. [ ] 合格まで繰り返し

---

## 📝 備考

### テスト環境のクリーンアップ

テスト完了後、以下を実行して環境を元に戻す:

```bash
# テストデータ削除
rm -rf ~/.cldev/learning_records.db
rm -rf ~/.cldev/learning_records/*.md
rm -rf ~/.cldev/sessions/*.md

# バックアップから復元
cp -r ~/cldev-test-backup/.cldev ~/
rm -rf ~/cldev-test-backup
```

### 参考ドキュメント

- [ユーザーガイド](../USER_GUIDE.md)
- [開発者ガイド](../DEVELOPER_GUIDE.md)
- [TODO](./TODO.md)

---

**作成日**: 2025-01-09
**最終更新**: 2025-01-09
**バージョン**: 1.0.0
