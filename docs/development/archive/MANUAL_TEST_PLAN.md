# cldev 実機テスト計画書（Phase 6 Phase 2）

**テスト対象**: Learning Record新規コマンド（check-file/suggest/similar）
**テスト日時**: 2025-11-09
**テスト実施者**: 開発者
**バイナリ**: target/release/cldev (3.2MB)

---

## 📋 テスト概要

### テスト対象コマンド

#### 既存コマンド（基本動作確認）
- `cldev lr new` - 学習記録作成
- `cldev lr find` - 検索
- `cldev lr stats` - 統計表示

#### 新規コマンド（Phase 6 Phase 2）⭐
- `cldev lr check-file <file>` - ホットスポット警告
- `cldev lr suggest <error>` - エラー類似問題提案
- `cldev lr similar <session-id>` - 類似セッション検索

### テストデータ準備

テスト用の学習記録を5件作成（多様なシナリオ）

---

## 🧪 テスト手順

### Phase 1: 環境確認（所要時間: 2分）

#### 1.1 バイナリ確認
```bash
# バイナリ存在確認
ls -lh target/release/cldev

# バージョン確認
./target/release/cldev --version

# ヘルプ表示
./target/release/cldev lr --help
```

**期待結果**:
- ✅ バイナリが存在（約3.2MB）
- ✅ バージョン情報表示（1.0.0）
- ✅ lr サブコマンド一覧表示（7コマンド）

**チェックリスト**:
- [ ] バイナリ存在確認
- [ ] バージョン表示OK
- [ ] ヘルプ表示OK（check-file/suggest/similar表示）

---

### Phase 2: テストデータ準備（所要時間: 5分）

#### 2.1 データベース初期化

```bash
# 既存データクリア（テスト用）
rm -rf ~/.cldev/learning_records.db
rm -rf ~/.cldev/learning_records/*.md

# ディレクトリ再作成
mkdir -p ~/.cldev/learning_records
```

#### 2.2 テストデータ作成（5件）

**データ1: Rust コンパイルエラー（src/main.rs）**
```bash
./target/release/cldev lr new \
  --title "Rust borrow checker error" \
  --type debug \
  --description "cannot borrow as mutable" \
  --root-cause "Tried to borrow immutable variable as mutable" \
  --solution "Added mut keyword to variable declaration" \
  --learning "Always declare variables as mut when mutation is needed" \
  --files "src/main.rs" \
  --tags "rust,borrow-checker,compiler-error" \
  --resolved
```

**データ2: Tokio ランタイムエラー（src/main.rs）**
```bash
./target/release/cldev lr new \
  --title "Tokio runtime panic" \
  --type debug \
  --description "thread 'tokio-runtime-worker' panicked" \
  --root-cause "Tried to spawn task on dropped runtime" \
  --solution "Ensure runtime lives long enough" \
  --learning "Use Arc<Runtime> for shared runtime access" \
  --files "src/main.rs,src/runtime.rs" \
  --tags "rust,tokio,async,panic" \
  --resolved
```

**データ3: SQL エラー（src/core/learning_db.rs）**
```bash
./target/release/cldev lr new \
  --title "SQLite constraint violation" \
  --type debug \
  --description "UNIQUE constraint failed: sessions.id" \
  --root-cause "Generated duplicate session ID" \
  --solution "Added millisecond precision to timestamp" \
  --learning "Use UUID or high-resolution timestamps for IDs" \
  --files "src/core/learning_db.rs" \
  --tags "rust,sqlite,database" \
  --resolved
```

**データ4: テストエラー（未解決、src/main.rs）**
```bash
./target/release/cldev lr new \
  --title "Test failure in integration test" \
  --type debug \
  --description "assertion failed: expected 5, got 3" \
  --root-cause "Mock data count mismatch" \
  --solution "Investigating..." \
  --learning "Need to verify test data setup" \
  --files "src/main.rs,tests/integration_test.rs" \
  --tags "rust,testing,integration-test"
```

**データ5: Clippy 警告（src/core/learning_db.rs）**
```bash
./target/release/cldev lr new \
  --title "Clippy warning: unused field" \
  --type debug \
  --description "field session_id is never read" \
  --root-cause "CompositeScore struct has unused field" \
  --solution "Add #[allow(dead_code)] or use the field" \
  --learning "Remove unused fields or document why they exist" \
  --files "src/core/learning_db.rs" \
  --tags "rust,clippy,warning" \
  --resolved
```

**チェックリスト**:
- [ ] データ1作成成功
- [ ] データ2作成成功
- [ ] データ3作成成功
- [ ] データ4作成成功（未解決）
- [ ] データ5作成成功

---

### Phase 3: 基本コマンド動作確認（所要時間: 3分）

#### 3.1 lr find - 検索機能

```bash
# キーワード検索
./target/release/cldev lr find "rust"

# タグ検索
./target/release/cldev lr find --tag "tokio"

# ファイル検索
./target/release/cldev lr find --file "src/main.rs"

# 未解決のみ
./target/release/cldev lr find --unresolved
```

**期待結果**:
- ✅ "rust" 検索で5件ヒット
- ✅ "tokio" タグで1件ヒット（データ2）
- ✅ "src/main.rs" で3件ヒット（データ1,2,4）
- ✅ 未解決で1件ヒット（データ4）

**チェックリスト**:
- [ ] キーワード検索OK（5件）
- [ ] タグ検索OK（1件）
- [ ] ファイル検索OK（3件）
- [ ] 未解決フィルタOK（1件）

#### 3.2 lr stats - 統計表示

```bash
./target/release/cldev lr stats
```

**期待結果**:
- ✅ 総セッション数: 5
- ✅ 解決済み: 4
- ✅ 未解決: 1
- ✅ タグ統計表示（rust: 5, tokio: 1, sqlite: 1, ...）

**チェックリスト**:
- [ ] セッション数正確（5件）
- [ ] 解決/未解決カウント正確（4/1）
- [ ] タグ統計表示OK

---

### Phase 4: 新規コマンド動作確認⭐（所要時間: 10分）

#### 4.1 lr check-file - ホットスポット警告

**テスト1: ホットスポットファイル（src/main.rs）**
```bash
./target/release/cldev lr check-file src/main.rs
```

**期待結果**:
```
⚠️  WARNING: This file is a HOTSPOT with past issues!

📊 Hotspot Statistics:
  • Sessions: 3
  • Avg Score: [数値]
  • Last Access: 2025-11-09 [時刻]

🔍 Recent Issues (last 5):
  1. Rust borrow checker error [Resolved]
  2. Tokio runtime panic [Resolved]
  3. Test failure in integration test [Unresolved]
```

**チェックリスト**:
- [ ] 警告メッセージ表示
- [ ] セッション数正確（3件）
- [ ] 過去問題一覧表示（最大5件）
- [ ] 解決状態表示正確

**テスト2: 問題のないファイル**
```bash
./target/release/cldev lr check-file src/cli/args.rs
```

**期待結果**:
```
✅ No past issues found for this file.
```

**チェックリスト**:
- [ ] 問題なしメッセージ表示

**テスト3: 存在しないファイル（エラーハンドリング）**
```bash
./target/release/cldev lr check-file nonexistent.rs
```

**期待結果**:
```
✅ No past issues found for this file.
または
⚠️  File not found, but checking database...
```

**チェックリスト**:
- [ ] エラーハンドリング正常

---

#### 4.2 lr suggest - エラー類似問題提案

**テスト1: 類似エラー検索（閾値デフォルト0.7）**
```bash
./target/release/cldev lr suggest "cannot borrow as mutable"
```

**期待結果**:
```
🔍 Found N similar problems:

1. [Score: 0.XX] Rust borrow checker error
   Description: cannot borrow as mutable
   Files: src/main.rs
   Tags: rust, borrow-checker, compiler-error
   Status: ✅ Resolved

   Root Cause: Tried to borrow immutable variable as mutable
   Solution: Added mut keyword to variable declaration
```

**チェックリスト**:
- [ ] 類似エラー検出（データ1がヒット）
- [ ] スコア表示（0.7以上）
- [ ] 詳細情報表示（解決策含む）

**テスト2: 類似エラー検索（別パターン）**
```bash
./target/release/cldev lr suggest "thread panicked"
```

**期待結果**:
- ✅ データ2（Tokio runtime panic）がヒット
- ✅ スコア0.7以上

**チェックリスト**:
- [ ] 正しいエラーマッチング
- [ ] スコア計算正確

**テスト3: 閾値変更（厳しく）**
```bash
./target/release/cldev lr suggest "panic" --threshold 0.9
```

**期待結果**:
- ✅ 閾値が高いため、ヒット数減少

**チェックリスト**:
- [ ] 閾値オプション機能

**テスト4: マッチなし**
```bash
./target/release/cldev lr suggest "completely unrelated error message xyz123"
```

**期待結果**:
```
ℹ️  No similar problems found above threshold 0.7
```

**チェックリスト**:
- [ ] マッチなしメッセージ表示

**テスト5: 結果数制限**
```bash
./target/release/cldev lr suggest "rust" --limit 2
```

**期待結果**:
- ✅ 最大2件のみ表示

**チェックリスト**:
- [ ] limit オプション機能

---

#### 4.3 lr similar - 類似セッション検索

**テスト1: セッションID取得**
```bash
# まず最新セッションのIDを取得
SESSION_ID=$(./target/release/cldev lr find "Rust borrow" --limit 1 | grep -o 'session-[0-9a-f-]*' | head -1)
echo "Target Session ID: $SESSION_ID"
```

**テスト2: 類似セッション検索**
```bash
./target/release/cldev lr similar "$SESSION_ID"
```

**期待結果**:
```
🎯 Target Session:
  • Title: Rust borrow checker error
  • Type: Debug
  • Date: 2025-11-09 [時刻]
  • Files: src/main.rs
  • Tags: rust, borrow-checker, compiler-error

🔗 Similar Sessions (by composite score):

1. [Score: 0.XX] Tokio runtime panic
   Type: Debug | Date: 2025-11-09
   Files: src/main.rs, src/runtime.rs
   Tags: rust, tokio, async, panic
   Status: ✅ Resolved

2. [Score: 0.XX] Test failure in integration test
   ...
```

**チェックリスト**:
- [ ] ターゲットセッション情報表示
- [ ] 類似セッションリスト表示
- [ ] スコア順ソート
- [ ] 複合スコアリング動作（ファイルマッチ優先）

**テスト3: 結果数制限**
```bash
./target/release/cldev lr similar "$SESSION_ID" --limit 2
```

**期待結果**:
- ✅ 最大2件のみ表示

**チェックリスト**:
- [ ] limit オプション機能

**テスト4: 存在しないセッションID（エラーハンドリング）**
```bash
./target/release/cldev lr similar "nonexistent-session-id"
```

**期待結果**:
```
❌ Error: Session not found: nonexistent-session-id
```

**チェックリスト**:
- [ ] エラーハンドリング正常

---

### Phase 5: エラーハンドリング確認（所要時間: 5分）

#### 5.1 不正な引数

```bash
# 引数なし
./target/release/cldev lr check-file

# 不正なオプション
./target/release/cldev lr suggest "error" --invalid-option

# 不正な閾値（範囲外）
./target/release/cldev lr suggest "error" --threshold 1.5
```

**期待結果**:
- ✅ 適切なエラーメッセージ表示
- ✅ ヘルプメッセージ表示

**チェックリスト**:
- [ ] 引数なしエラー
- [ ] 不正オプションエラー
- [ ] 範囲外値エラー

#### 5.2 データベースエラー

```bash
# データベースファイル権限変更
chmod 000 ~/.cldev/learning_records.db
./target/release/cldev lr find "test"

# 権限戻す
chmod 644 ~/.cldev/learning_records.db
```

**期待結果**:
- ✅ データベースアクセスエラー表示

**チェックリスト**:
- [ ] 権限エラーハンドリング

---

### Phase 6: パフォーマンス確認（所要時間: 3分）

#### 6.1 検索速度

```bash
# 時間計測
time ./target/release/cldev lr find "rust"
time ./target/release/cldev lr suggest "panic"
time ./target/release/cldev lr similar "$SESSION_ID"
```

**期待結果**:
- ✅ find: < 100ms（5件データ）
- ✅ suggest: < 200ms（類似度計算）
- ✅ similar: < 300ms（複合スコアリング）

**チェックリスト**:
- [ ] find速度OK（< 100ms）
- [ ] suggest速度OK（< 200ms）
- [ ] similar速度OK（< 300ms）

---

## 📊 テスト結果記録表

### 実施日時
**日付**: _________________
**実施者**: _________________

### 結果サマリー

| Phase | テスト項目 | 通過 | 失敗 | スキップ | 備考 |
|-------|-----------|------|------|----------|------|
| Phase 1 | 環境確認 | ___ / 3 | ___ | ___ | |
| Phase 2 | データ準備 | ___ / 5 | ___ | ___ | |
| Phase 3 | 基本コマンド | ___ / 7 | ___ | ___ | |
| Phase 4.1 | check-file | ___ / 6 | ___ | ___ | |
| Phase 4.2 | suggest | ___ / 9 | ___ | ___ | |
| Phase 4.3 | similar | ___ / 7 | ___ | ___ | |
| Phase 5 | エラー処理 | ___ / 4 | ___ | ___ | |
| Phase 6 | パフォーマンス | ___ / 3 | ___ | ___ | |
| **合計** | **全項目** | **___ / 44** | **___** | **___** | |

### 合格基準
- ✅ **合格**: 40/44以上（90%以上）
- ⚠️ **条件付き合格**: 35/44以上（80%以上）、重大バグなし
- ❌ **不合格**: 35/44未満、または重大バグ検出

---

## 🐛 発見した問題の記録

### 問題1
- **重要度**: [ ] 高 [ ] 中 [ ] 低
- **発生箇所**: _________________
- **現象**: _________________
- **再現手順**: _________________
- **対処方針**: _________________

### 問題2
- **重要度**: [ ] 高 [ ] 中 [ ] 低
- **発生箇所**: _________________
- **現象**: _________________
- **再現手順**: _________________
- **対処方針**: _________________

（必要に応じて追加）

---

## ✅ テスト完了チェックリスト

- [ ] 全Phaseのテスト実施完了
- [ ] テスト結果記録表記入完了
- [ ] 発見した問題を記録（なければ「なし」と記入）
- [ ] パフォーマンス測定値記録
- [ ] 合格/不合格判定完了
- [ ] 次のアクション決定（合格なら配布準備、不合格なら修正）

---

## 📝 次のアクション

### テスト合格時
1. [ ] TODO.md更新（Phase 6 Phase 2実機テスト完了）
2. [ ] README.md更新（コマンド数・機能説明）
3. [ ] crates.io公開準備開始

### テスト不合格時
1. [ ] 問題修正
2. [ ] 再テスト実施
3. [ ] 合格まで繰り返し

---

**作成日**: 2025-11-09
**最終更新**: 2025-11-09
