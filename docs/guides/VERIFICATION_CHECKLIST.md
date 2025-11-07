# cldev 動作確認チェックリスト

**最終更新**: 2025-11-07
**対象バージョン**: 1.0.0

このドキュメントは、cldevのインストール後に正しく動作しているか確認するためのチェックリストです。

---

## 📋 目次

- [インストール確認](#インストール確認)
- [基本機能確認](#基本機能確認)
- [設定確認](#設定確認)
- [コマンド動作確認](#コマンド動作確認)
- [統合機能確認](#統合機能確認)
- [トラブルシューティング](#トラブルシューティング)

---

## ✅ インストール確認

### 1. バイナリ確認

```bash
# cldevコマンドが利用可能か確認
which cldev
# 期待される出力: /usr/local/bin/cldev または ~/.cargo/bin/cldev

# バージョン確認
cldev --version
# 期待される出力: cldev 1.0.0
```

**チェック項目**:
- [ ] `cldev`コマンドが見つかる
- [ ] バージョンが正しく表示される（1.0.0）
- [ ] エラーが表示されない

### 2. ヘルプ表示確認

```bash
# ヘルプが正しく表示されるか確認
cldev --help

# サブコマンドのヘルプ確認
cldev config --help
cldev dev --help
```

**チェック項目**:
- [ ] ヘルプが正しく表示される
- [ ] 9カテゴリのコマンドが表示される
- [ ] グローバルオプションが表示される

---

## 🔧 基本機能確認

### 3. 初期設定実行

```bash
# 設定ウィザードを実行
cldev config init

# または既存設定がある場合はチェック
cldev config check
```

**期待される動作**:
- 言語選択が表示される（日本語/English）
- Claude Codeディレクトリが検出される
- プロジェクトルートの入力が求められる
- Git CLIツールの検出が行われる
- シェル補完の設定が提案される

**チェック項目**:
- [ ] 設定ウィザードが正常に起動する
- [ ] 言語選択が機能する
- [ ] ディレクトリパスが正しく検出される
- [ ] 設定ファイルが作成される（`~/.config/cldev/config.toml`）

### 4. 設定ファイル確認

```bash
# 設定ファイルの存在確認
ls -la ~/.config/cldev/config.tomlw

# 設定内容の確認
cldev config check --detailed
```

**チェック項目**:
- [ ] 設定ファイルが存在する
- [ ] パーミッションが600である
- [ ] 設定検証が成功する
- [ ] エラーや警告が表示されない

---

## 🎯 設定確認

### 5. 多言語機能確認

```bash
# 英語で実行
cldev config check --lang en

# 日本語で実行
cldev config check --lang ja
```

**チェック項目**:
- [ ] 英語メッセージが正しく表示される
- [ ] 日本語メッセージが正しく表示される
- [ ] 言語切り替えが機能する

### 6. 出力形式確認

```bash
# 色付き出力（デフォルト）
cldev config list

# 色なし出力
cldev config list --no-color

# 詳細出力
cldev config list --verbose

# 静かな出力
cldev config list --quiet
```

**チェック項目**:
- [ ] 色付き出力が正しく表示される
- [ ] `--no-color`で色が無効化される
- [ ] `--verbose`で詳細情報が表示される
- [ ] `--quiet`でエラー以外が抑制される

---

## 🚀 コマンド動作確認

### 7. Configコマンド（6個）

```bash
# 全コマンド一覧
cldev config list
# 期待: 29コマンドが9カテゴリで表示

# 詳細版
cldev config list --detailed
# 期待: 各コマンドの説明が表示

# カテゴリフィルタ
cldev config list --filter dev
# 期待: devカテゴリの7コマンドのみ表示

# 設定編集（エディタ起動確認のみ、すぐ閉じる）
# cldev config edit
```

**チェック項目**:
- [ ] `config list`: 全29コマンドが表示される
- [ ] `config list --detailed`: 説明が表示される
- [ ] `config list --filter`: フィルタが機能する
- [ ] `config check`: 設定検証が成功する
- [ ] `config edit`: エディタが起動する（オプション）

### 8. プロジェクト検出確認

既存のプロジェクトディレクトリで以下を実行：

```bash
# Node.jsプロジェクトで
cd /path/to/nodejs-project
cldev quality lint --help
# 期待: ESLintの説明が表示される

# Rustプロジェクトで
cd /path/to/rust-project
cldev quality lint --help
# 期待: Clippyの説明が表示される
```

**チェック項目**:
- [ ] Node.jsプロジェクトが正しく検出される
- [ ] Rustプロジェクトが正しく検出される
- [ ] プロジェクトタイプに応じたヘルプが表示される

### 9. Git統合確認

Gitリポジトリ内で実行：

```bash
# Git状態確認
cd /path/to/git-repo
cldev git status

# GitHub/GitLab CLI検出確認
cldev config check --detailed
# 期待: Git CLI検出状況が表示される
```

**チェック項目**:
- [ ] Gitリポジトリが検出される
- [ ] Git状態が正しく表示される
- [ ] `gh`または`glab`の検出が正しい

### 10. シェル補完確認

```bash
# 補完スクリプト生成
cldev completions bash > /tmp/cldev-completion-test.bash
cat /tmp/cldev-completion-test.bash | head -20

# または実際のシェルで確認（bashの例）
cldev comp[TAB]
# 期待: "completions"に補完される
```

**チェック項目**:
- [ ] 補完スクリプトが生成される
- [ ] スクリプトにコマンドが含まれる
- [ ] シェル補完が機能する（設定済みの場合）

---

## 🔗 統合機能確認

### 11. 学習記録機能

```bash
# テスト用学習記録作成
cldev lr new "テスト記録" --edit=false

# 記録検索
cldev lr find "テスト"

# 統計確認
cldev lr stats
```

**チェック項目**:
- [ ] 学習記録が作成される
- [ ] 検索が機能する
- [ ] 統計が表示される

### 12. セッションディレクトリ確認

```bash
# セッションディレクトリの確認
ls -la ~/.claude/learning-sessions/

# セッションファイルの内容確認（最新のもの）
cat $(ls -t ~/.claude/learning-sessions/*.json | head -1)
```

**チェック項目**:
- [ ] セッションディレクトリが存在する
- [ ] JSONファイルが作成される
- [ ] JSONが正しい形式である

---

## 📊 パフォーマンス確認

### 13. 起動速度測定

```bash
# 起動時間測定（macOS/Linux）
time cldev --version

# 複数回実行して平均確認
for i in {1..5}; do time cldev --version; done
```

**期待される結果**:
- 起動時間: < 50ms（目標: 5-10ms）

**チェック項目**:
- [ ] 起動時間が50ms未満
- [ ] 一貫して高速に起動する

### 14. メモリ使用量確認

```bash
# メモリ使用量確認（macOS）
/usr/bin/time -l cldev config list 2>&1 | grep "maximum resident"

# Linux
/usr/bin/time -v cldev config list 2>&1 | grep "Maximum resident"
```

**期待される結果**:
- メモリ使用量: < 50MB

**チェック項目**:
- [ ] メモリ使用量が妥当
- [ ] メモリリークがない

---

## 🔍 詳細機能確認

### 15. エラー処理確認

```bash
# 存在しないコマンド
cldev invalid-command
# 期待: エラーメッセージと類似コマンド提案

# 不正な引数
cldev config check --invalid-flag
# 期待: エラーメッセージとヘルプ表示

# 存在しない設定ファイル
rm ~/.config/cldev/config.toml
cldev config check
# 期待: 設定ファイルが見つからない旨のエラー
```

**チェック項目**:
- [ ] 適切なエラーメッセージが表示される
- [ ] ヘルプが表示される
- [ ] クラッシュしない

### 16. セキュリティ機能確認

```bash
# 設定ファイルのパーミッション確認
ls -la ~/.config/cldev/config.toml
# 期待: -rw------- (600)

# 不適切なパーミッションでの動作確認
chmod 644 ~/.config/cldev/config.toml
cldev config check
# 期待: セキュリティ警告が表示される

# パーミッション修復
chmod 600 ~/.config/cldev/config.toml
```

**チェック項目**:
- [ ] 設定ファイルのパーミッションが600
- [ ] 不適切なパーミッションで警告が表示される
- [ ] セキュリティチェックが機能する

---

## 🐛 トラブルシューティング

### よくある問題と解決方法

#### 問題1: `cldev: command not found`

**原因**: PATHが通っていない

**解決方法**:
```bash
# cargoでインストールした場合
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# または
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

#### 問題2: 設定ファイルが見つからない

**原因**: 初期設定が未実行

**解決方法**:
```bash
# 初期設定を実行
cldev config init
```

#### 問題3: シェル補完が機能しない

**原因**: 補完スクリプトが未インストール

**解決方法**:
```bash
# Bashの場合
cldev completions bash > ~/.bash_completion.d/cldev
source ~/.bashrc

# Zshの場合
cldev completions zsh > ~/.zsh/completions/_cldev
# .zshrcに以下を追加
# fpath=(~/.zsh/completions $fpath)
# autoload -Uz compinit && compinit
source ~/.zshrc
```

#### 問題4: 日本語が文字化けする

**原因**: ロケール設定が正しくない

**解決方法**:
```bash
# ロケール確認
locale

# LANG設定
export LANG=ja_JP.UTF-8
```

#### 問題5: Git CLIが検出されない

**原因**: `gh`または`glab`がインストールされていない

**解決方法**:
```bash
# GitHub CLIインストール（macOS）
brew install gh

# GitLab CLIインストール（macOS）
brew install glab
```

---

## ✅ 最終チェックリスト

すべての確認項目をチェックしてください：

### インストール
- [ ] コマンドが利用可能
- [ ] バージョン表示が正常
- [ ] ヘルプ表示が正常

### 設定
- [ ] 初期設定が完了
- [ ] 設定ファイルが作成された
- [ ] 設定検証が成功

### 多言語
- [ ] 英語表示が正常
- [ ] 日本語表示が正常
- [ ] 言語切り替えが機能

### 基本コマンド
- [ ] `config`コマンドが動作
- [ ] プロジェクト検出が機能
- [ ] Git統合が機能

### パフォーマンス
- [ ] 起動時間が50ms未満
- [ ] メモリ使用量が妥当

### セキュリティ
- [ ] パーミッションチェックが機能
- [ ] 適切なエラー処理

---

## 📝 問題報告

すべてのチェック項目が完了しない場合：

1. **GitHub Issues**: https://github.com/sanae-abe/cldev/issues
2. **詳細情報を含める**:
   - OS・バージョン（`uname -a`）
   - Rustバージョン（`rustc --version`）
   - cldevバージョン（`cldev --version`）
   - エラーメッセージ全文
   - 実行したコマンド

**問題報告テンプレート**:
```markdown
## 環境
- OS: macOS 14.0 (または Linux/Windows)
- Rust: 1.70.0
- cldev: 1.0.0

## 問題
[問題の説明]

## 再現手順
1. [手順1]
2. [手順2]
3. ...

## 期待される動作
[期待される動作]

## 実際の動作
[実際の動作・エラーメッセージ]
```

---

## 🎉 確認完了

すべてのチェック項目が完了したら、cldevは正常に動作しています！

**次のステップ**:
1. [ユーザーガイド](../USER_GUIDE.md)を読む
2. [クイックスタート](QUICKSTART.md)で実際のワークフローを試す
3. 実際のプロジェクトで使用開始

**フィードバック歓迎**:
- 改善提案: [GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- バグ報告: [GitHub Issues](https://github.com/sanae-abe/cldev/issues)

---

**最終更新**: 2025-11-07
**ドキュメントバージョン**: 1.0.0
