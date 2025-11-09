# 学習記録システムV3実装計画書

**プロジェクト**: cldev Learning Record System V3
**作成日**: 2025-01-09
**作成者**: Claude Code
**バージョン**: 1.0.0
**目標**: AIファーストから人間ファースト（AI活用）設計への転換

---

## 📋 目次

1. [プロジェクト概要](#プロジェクト概要)
2. [設計哲学の転換](#設計哲学の転換)
3. [Phase 1: 最優先機能（1-2週間）](#phase-1-最優先機能1-2週間)
4. [Phase 2: 高優先度機能（2-3週間）](#phase-2-高優先度機能2-3週間)
5. [Phase 3: 中優先度機能（1-2ヶ月）](#phase-3-中優先度機能1-2ヶ月)
6. [マイグレーション戦略](#マイグレーション戦略)
7. [実装スケジュール](#実装スケジュール)
8. [成功指標（KPI）](#成功指標kpi)
9. [リスク管理](#リスク管理)

---

## プロジェクト概要

### 背景

現行の学習記録システム（V2）は**AIが読みやすい構造化データ**を重視した設計となっており、以下の問題が発生しています：

- ❌ YAMLフロントマターが冗長（全体の70%）
- ❌ 手動入力の心理的ハードルが高い
- ❌ 記録習慣化が困難
- ❌ AI活用の価値が不明確

### 目標

**「人間が自然に書き、AIが活用する知識ベース」への転換**

- ✅ ユーザー入力負荷を**80%削減**
- ✅ 学習記録作成率を**0% → 50%以上**に向上
- ✅ AIセッション自動記録機能の実装
- ✅ 機密情報の自動マスキング（セキュリティリスク0件）

### スコープ

| 対象 | 詳細 |
|------|------|
| **実装言語** | Rust 1.70+ |
| **対象ユーザー** | cldev利用開発者（個人・チーム） |
| **対象環境** | macOS, Linux, Windows |
| **既存システム** | Learning Record V2（後方互換性維持） |

---

## 設計哲学の転換

### Before（AIファースト）

```yaml
---
session_meta:
  id: debug_20250109_143022_456
  session_type: debug
  priority: high
  timestamp: 2025-01-09T14:30:22+09:00
  duration_minutes: 45
  resolved: true
problem:
  title: "Rust clippy error"
  description: "..."
  severity: error
  error_signatures:
    - error_type: "CompileError"
      pattern: "no method named `calculate_hotspot_score`"
solution:
  summary: "..."
  root_cause: "..."
  steps: [...]
  verification: [...]
context:
  tags: ["rust", "clippy"]
  files_affected: [...]
  dependencies: [...]
learnings:
  - insight: "..."
    category: "rust-pattern"
    reusability: high
---

# Session Notes
Add your notes here...
```

**問題点**:
- 人間が読むには冗長（70%がYAML）
- 手入力が困難な構造化フィールド
- 記入欄がほぼ空欄

### After（人間ファースト）

```markdown
---
id: 2025-01-09-rust-clippy-fix
created: 2025-01-09 14:30
tags: [rust, clippy, learning_db]
status: resolved
---

# Rust clippy エラー修正：learning_db.rsのメソッド不足

## 何が起きたか
`cargo clippy`実行時に「no method named `calculate_hotspot_score`」エラー。

## どう解決したか
1. `learning_db.rs:411`に`calculate_hotspot_score`メソッドを実装
2. Priority、recency、file_count、unresolved状態の4要素でスコア計算
3. `cargo clippy`で確認 → 成功

## 学んだこと
Rustのメソッド実装忘れはコンパイル時に検出されるが、clippy使うとより早く気づける。

## 関連ファイル
- src/core/learning_db.rs (主要変更)
```

**改善点**:
- ✅ 最小限のYAML（10%）
- ✅ Markdown本文が主役（90%）
- ✅ 人間の思考に沿った構造（時系列・因果関係）

### 新テンプレート形式（3レベル）

#### Level 1: 手動記録
- **ユースケース**: ユーザーが明示的に記録したい重要な問題
- **入力方法**: `cldev lr new`で手動作成
- **構造**: 最小限のYAML + 自由記述Markdown

#### Level 2: 半自動生成（推奨）
- **ユースケース**: セッション終了時にAIが自動提案
- **入力方法**: `cldev session end`で自動生成 → ユーザー編集
- **構造**: AIが80%生成、ユーザーが20%編集
- **特徴**: 信頼度スコア表示、人間の追記領域明示

#### Level 3: フルバックグラウンド
- **ユースケース**: 検索インデックスのみ（表示しない）
- **入力方法**: 完全自動（ユーザー介入不要）
- **構造**: 最小限のメタデータのみ

---

## Phase 1: 最優先機能（1-2週間）

### 目標
**ユーザー負荷を80%削減し、記録習慣化を実現**

### 実装タスク

#### 1.1 セッションコンテキスト追跡

**ファイル**: `src/core/session_context.rs`（新規作成）

**実装内容**:
```rust
pub struct SessionContext {
    pub session_id: String,
    pub start_time: DateTime<Local>,
    pub command_history: Vec<CommandRecord>,
    pub todo_history: Vec<TodoSnapshot>,
    pub errors_encountered: Vec<ErrorCapture>,
    pub files_modified: Vec<FileModification>,
    pub tool_usage: Vec<ToolUsage>,
}
```

**主要機能**:
- コマンド履歴の記録（exit code、実行時間）
- エラー発生の自動検出
- TodoWrite統合（進捗追跡）
- ファイル変更の追跡

**期間**: 3日
**優先度**: P0（Critical）
**依存**: なし

#### 1.2 機密情報自動マスキング

**ファイル**: `src/core/sanitizer.rs`（新規作成）

**実装内容**:
```rust
lazy_static! {
    static ref SECRET_PATTERNS: Vec<(Regex, &'static str)> = vec![
        (Regex::new(r"(?i)(api[_-]?key|apikey)\s*[:=]\s*['\"]?([a-zA-Z0-9_\-]{20,})['\"]?").unwrap(),
         "$1: [REDACTED_API_KEY]"),
        // ... 他のパターン
    ];
}

pub fn sanitize_text(text: &str) -> SanitizationResult;
```

**対応パターン**:
- APIキー（api_key, apikey）
- パスワード（password, passwd, pwd）
- トークン（bearer, token）
- メールアドレス
- IPアドレス
- AWSアクセスキー
- GitHub/GitLab トークン

**期間**: 2日
**優先度**: P0（Critical - セキュリティ）
**依存**: なし

#### 1.3 自動記録判定ロジック

**ファイル**: `src/commands/lr/auto_capture.rs`（新規作成）

**実装内容**:
```rust
pub fn analyze_session(ctx: &SessionContext) -> RecordRecommendation {
    let mut score = 0.0;

    // エラー発生回数（重み: 0.3）
    // 作業時間（重み: 0.25）
    // ファイル変更数（重み: 0.2）
    // コマンド複雑性（重み: 0.15）
    // Todo完了数（重み: 0.1）

    // score >= 0.7 → Level 2（半自動生成）
    // score >= 0.3 → Level 3（バックグラウンド）
    // score < 0.3  → Skip
}
```

**判定基準**:

| スコア | 推奨レベル | 条件例 |
|--------|----------|--------|
| ≥ 0.7 | Level 2（フル記録） | エラー3回以上、30分以上作業、5ファイル以上変更 |
| 0.3-0.7 | Level 3（背景記録） | エラー1-2回、10-30分作業 |
| < 0.3 | Skip | Trivialな作業 |

**期間**: 2日
**優先度**: P0（Critical）
**依存**: 1.1（SessionContext）

#### 1.4 Markdown生成（V3形式）

**ファイル**: `src/commands/lr/auto_capture.rs`（拡張）

**実装内容**:
```rust
fn generate_level2_markdown(ctx: &SessionContext, rec: &RecordRecommendation) -> String {
    format!(
        r#"---
id: auto-{timestamp}
created: {created}
auto_generated: true
confidence: {confidence:.2}
tags: {tags}
status: {status}
duration_min: {duration}
---

# 【自動生成】{title}

## 🔍 検出された問題
{error_summary}

## 📝 実行された作業（TodoWrite履歴から抽出）
{todo_summary}

## 📊 変更されたファイル
{file_changes}

## 💡 AIによる学びの抽出
{learnings}

---

<!-- 以下、人間が追記する領域 -->
## ✍️ 追加メモ



## 🔗 関連リンク

"#)
}
```

**自動抽出情報**:
- エラーサマリー（type、message、context、解決状態）
- Todo履歴（完了/進行中/保留）
- ファイル変更（追加/削除/変更行数）
- 学びの推測（エラーパターンから）

**期間**: 3日
**優先度**: P0（Critical）
**依存**: 1.3（auto_capture）

#### 1.5 新フォーマット対応（V3）

**ファイル**: `src/core/learning_record_v3.rs`（新規作成）

**実装内容**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecordV3 {
    pub id: String,
    pub created: DateTime<Local>,
    pub auto_generated: bool,
    pub confidence: Option<f64>,
    pub tags: Vec<String>,
    pub status: RecordStatus,
    pub duration_min: Option<i64>,
    pub markdown_body: String,
}

impl LearningRecordV3 {
    pub fn to_markdown_file(&self) -> String;
    pub fn from_markdown_file(content: &str) -> Result<Self>;
}
```

**V2との差分**:
- YAMLフロントマター: 12フィールド → 7フィールド（42%削減）
- Markdown本文: 固定テンプレート → 自由記述
- ファイルサイズ: 平均370行 → 20-30行（92%削減）

**期間**: 2日
**優先度**: P0（Critical）
**依存**: なし

#### 1.6 CLI統合

**ファイル**: `src/cli/session.rs`（新規作成）

**新規コマンド**:
```bash
cldev session start              # セッション開始
cldev session end                # セッション終了＋記録提案
cldev session status             # 現在のセッション状態

# 既存コマンドの拡張
cldev lr new                     # Level 1手動記録
cldev lr new --auto              # Level 2半自動記録
cldev lr review <session-id>     # 自動生成記録のレビュー
```

**セッション終了フロー**:
```
cldev session end
↓
セッション分析（スコア計算）
↓
スコア >= 0.7 → "このセッションを記録しますか？(Y/n)"
              → Y → Markdownドラフト生成 → エディタで編集 → 保存
              → n → スキップ
↓
スコア 0.3-0.7 → "バックグラウンドで記録しました"
↓
スコア < 0.3 → スキップ
```

**期間**: 3日
**優先度**: P1（High）
**依存**: 1.3, 1.4（auto_capture, Markdown生成）

### Phase 1 成果物

| 成果物 | 内容 |
|--------|------|
| **新規ファイル** | 5ファイル（session_context.rs, sanitizer.rs, auto_capture.rs, learning_record_v3.rs, session.rs） |
| **実装行数** | 約2,500行（推定） |
| **テストコード** | 約500行（各機能のユニットテスト） |
| **ドキュメント** | 本計画書、API仕様書 |

### Phase 1 完了条件

- [ ] セッション追跡が正常動作（コマンド履歴、エラー検出、Todo統合）
- [ ] 機密情報マスキングテスト全合格（7パターン以上）
- [ ] 自動記録判定の精度検証（手動検証10セッション）
- [ ] V3形式の読み書きテスト全合格
- [ ] `cldev session end`が正常動作
- [ ] 学習記録作成率が20%以上に向上（手動測定）

---

## Phase 2: 高優先度機能（2-3週間）

### 目標
**価値の可視化と長期運用の持続性確保**

### 実装タスク

#### 2.1 記録推奨度判定（ML強化版）

**ファイル**: `src/core/record_recommender.rs`（新規作成）

**実装内容**:
```rust
pub struct MLRecommender {
    model: Option<DecisionTreeModel>,
}

impl MLRecommender {
    pub fn train_from_history(&mut self) -> Result<()>;
    pub fn predict(&self, ctx: &SessionContext) -> RecordRecommendation;
}
```

**機械学習アプローチ**:
- アルゴリズム: 決定木（Rustで実装可能、外部依存なし）
- 学習データ: 過去の学習記録（ユーザーが保存したセッション = 正例）
- 特徴量:
  - エラー発生回数
  - 作業時間
  - ファイル変更数
  - コマンド失敗率
  - Todo完了数
  - ファイルホットスポットスコア

**期間**: 5日
**優先度**: P1（High）
**依存**: Phase 1完了

#### 2.2 ROIダッシュボード

**ファイル**: `src/commands/lr/stats.rs`（拡張）

**新規コマンド**:
```bash
cldev lr stats --roi
```

**表示内容**:
```
📊 学習記録 ROI ダッシュボード

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📈 基本統計
   総記録数:         45 件
   解決済み:         38 件 (84%)
   平均解決時間:     32 分

⏱️  推定効果
   過去30日の時間短縮: 12 時間
   (類似問題の再解決時間を50%削減と仮定)

🔥 最も参照された記録 (Top 5)
   1. Rust clippy エラー修正 (12 回参照)
   2. Docker build失敗対応 (8 回参照)
   3. TypeScript型エラー解決 (7 回参照)
   ...
```

**時間短縮効果の計算式**:
```
時間短縮 = Σ(参照された記録の元の解決時間 × 0.5)
```

**期間**: 3日
**優先度**: P1（High）
**依存**: Phase 1完了

#### 2.3 データ保持期限・アーカイブ機能

**ファイル**: `src/commands/lr/maintain.rs`（新規作成）

**実装内容**:
```rust
pub struct RetentionPolicy {
    pub resolved_days: u32,    // デフォルト: 90日
    pub unresolved_days: u32,  // デフォルト: 365日
    pub auto_archive: bool,
}

pub fn apply_retention_policy(policy: &RetentionPolicy) -> Result<MaintenanceReport>;
```

**新規コマンド**:
```bash
cldev lr maintain                  # メンテナンス実行
cldev lr maintain --dry-run        # プレビューのみ
cldev lr maintain --archive-all    # 手動アーカイブ
```

**機能**:
- 解決済みセッションの自動アーカイブ（90日経過後）
- 未解決セッションの警告表示（365日経過）
- アーカイブファイルの圧縮保存
- データベースサイズの管理

**期間**: 4日
**優先度**: P1（High）
**依存**: Phase 1完了

### Phase 2 成果物

| 成果物 | 内容 |
|--------|------|
| **新規ファイル** | 2ファイル（record_recommender.rs, maintain.rs） |
| **拡張ファイル** | 1ファイル（stats.rs） |
| **実装行数** | 約1,800行（推定） |
| **テストコード** | 約350行 |

### Phase 2 完了条件

- [ ] ML推奨度判定の精度が80%以上（手動検証）
- [ ] ROIダッシュボードが正確な統計を表示
- [ ] アーカイブ機能が正常動作（90日以上の記録を自動処理）
- [ ] データベースサイズが適切に管理される

---

## Phase 3: 中優先度機能（1-2ヶ月）

### 目標
**拡張性向上と組織レベル活用の実現**

### 実装タスク

#### 3.1 プラグインアーキテクチャ導入

**ファイル**: `src/core/plugin.rs`（新規作成）

**実装内容**:
```rust
pub trait LearningRecordPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn on_session_create(&self, record: &LearningRecordV3) -> Result<()>;
    fn on_session_update(&self, record: &LearningRecordV3) -> Result<()>;
    fn on_session_query(&self, query: &str) -> Result<Vec<PluginSuggestion>>;
}
```

**プラグイン例**:
- Slack通知プラグイン
- Notion同期プラグイン
- Jira連携プラグイン

**期間**: 7日
**優先度**: P2（Medium）
**依存**: Phase 1完了

#### 3.2 Slackプラグイン実装

**ファイル**: `src/plugins/slack_notifier.rs`（新規作成）

**実装内容**:
```rust
pub struct SlackNotificationPlugin {
    webhook_url: String,
}

impl LearningRecordPlugin for SlackNotificationPlugin {
    fn on_session_create(&self, record: &LearningRecordV3) -> Result<()> {
        if record.confidence.unwrap_or(0.0) >= 0.8 {
            self.send_slack_message(&format!(
                "🎓 新しい学習記録: {} (信頼度: {:.0}%)",
                record.id,
                record.confidence.unwrap() * 100.0
            ))?;
        }
        Ok(())
    }
}
```

**期間**: 3日
**優先度**: P2（Medium）
**依存**: 3.1（プラグインシステム）

#### 3.3 マルチモーダル対応

**ファイル**: `src/core/learning_record_v3.rs`（拡張）

**実装内容**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub attachment_type: AttachmentType,
    pub path: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttachmentType {
    Image,
    Video,
    Audio,
    Document,
}

// LearningRecordV3に追加
impl LearningRecordV3 {
    pub attachments: Vec<Attachment>,
}
```

**ユースケース**:
- スクリーンショット添付
- エラー画面のキャプチャ
- 動画による問題再現手順
- 音声メモの文字起こし

**期間**: 5日
**優先度**: P3（Low）
**依存**: Phase 1完了

### Phase 3 成果物

| 成果物 | 内容 |
|--------|------|
| **新規ファイル** | 2ファイル（plugin.rs, slack_notifier.rs） |
| **拡張ファイル** | 1ファイル（learning_record_v3.rs） |
| **実装行数** | 約1,200行（推定） |
| **テストコード** | 約250行 |

### Phase 3 完了条件

- [ ] プラグインシステムが正常動作
- [ ] Slackプラグインが通知を送信
- [ ] 画像・動画添付が機能する
- [ ] ユーザー満足度が80%以上（アンケート）

---

## マイグレーション戦略

### 既存データ（V2）から新形式（V3）への移行

#### 移行方針

1. **後方互換性維持**: V2形式の既存記録も引き続き読み込み可能
2. **段階的移行**: 新規記録のみV3形式で作成
3. **オンデマンド変換**: ユーザーの明示的な操作でV2→V3変換

#### 実装内容

**ファイル**: `src/commands/lr/migrate.rs`（新規作成）

```rust
pub fn migrate_v2_to_v3(v2_record: &LearningRecordV2) -> Result<LearningRecordV3> {
    let markdown_body = generate_v3_markdown_from_v2(v2_record);

    Ok(LearningRecordV3 {
        id: v2_record.session_meta.id.clone(),
        created: v2_record.session_meta.timestamp,
        auto_generated: false,
        confidence: None,
        tags: v2_record.context.tags.clone(),
        status: map_v2_status_to_v3(&v2_record.session_meta),
        duration_min: v2_record.session_meta.duration_minutes,
        markdown_body,
    })
}
```

#### 移行コマンド

```bash
# 全V2記録をV3形式に変換
cldev lr migrate --all

# 特定の記録のみ変換
cldev lr migrate <session-id>

# 変換プレビュー（実際には変換しない）
cldev lr migrate --all --dry-run
```

#### 移行スケジュール

| フェーズ | 期間 | 内容 |
|---------|------|------|
| Phase 1完了後 | 2日 | migrate.rs実装 |
| Phase 2期間中 | 1日 | 既存データの移行テスト |
| Phase 3前 | - | 全ユーザーへ移行案内 |

---

## 実装スケジュール

### タイムライン（全体8週間）

```
Week 1-2: Phase 1（最優先機能）
  Day 1-3:   セッションコンテキスト追跡
  Day 2-4:   機密情報マスキング（並行）
  Day 4-6:   自動記録判定
  Day 7-9:   Markdown生成（V3形式）
  Day 10-11: 新フォーマット対応
  Day 12-14: CLI統合

Week 3-4: Phase 2（高優先度機能）
  Day 15-19: ML推奨度判定
  Day 20-22: ROIダッシュボード
  Day 23-26: アーカイブ機能
  Day 27-28: Phase 2テスト

Week 5-8: Phase 3（中優先度機能）
  Day 29-35: プラグインシステム
  Day 36-38: Slackプラグイン
  Day 39-43: マルチモーダル対応
  Day 44-46: Phase 3テスト
  Day 47-56: バッファ期間
```

### 実装優先順位マトリクス

| 機能 | ユーザー価値 | 実装難易度 | 優先度 | 期間 | 依存関係 |
|------|------------|----------|--------|------|---------|
| セッション自動追跡 | 🔴 Critical | 🟡 Medium | P0 | 3日 | なし |
| 機密情報マスキング | 🔴 Critical | 🟢 Low | P0 | 2日 | なし |
| V3フォーマット対応 | 🔴 Critical | 🟡 Medium | P0 | 5日 | なし |
| 自動記録判定 | 🔴 Critical | 🟡 Medium | P0 | 2日 | セッション追跡 |
| CLI統合 | 🟡 High | 🟢 Low | P1 | 3日 | 自動記録判定 |
| ROIダッシュボード | 🟡 High | 🟢 Low | P1 | 3日 | Phase 1完了 |
| アーカイブ機能 | 🟡 High | 🟡 Medium | P1 | 4日 | Phase 1完了 |
| ML推奨度判定 | 🟢 Medium | 🔴 High | P2 | 5日 | Phase 1完了 |
| プラグインシステム | 🟢 Medium | 🔴 High | P2 | 7日 | Phase 1完了 |
| マルチモーダル | 🟢 Low | 🟡 Medium | P3 | 5日 | Phase 1完了 |

---

## 成功指標（KPI）

### Phase 1完了時（2週間後）

| 指標 | 目標値 | 測定方法 |
|------|--------|---------|
| 学習記録作成率 | 50%以上 | セッション数 vs 記録数 |
| ユーザー手動入力時間 | 80%削減 | 記録作成時間の計測 |
| 機密情報漏洩リスク | 0件 | セキュリティテスト |
| セッション追跡精度 | 95%以上 | 手動検証 |

### Phase 2完了時（4週間後）

| 指標 | 目標値 | 測定方法 |
|------|--------|---------|
| 学習記録の参照頻度 | 週3回以上 | アクセスログ |
| 推定時間短縮効果 | 月10時間以上 | ROIダッシュボード |
| データベースサイズ | 50MB以下 | アーカイブ機能稼働 |
| ML推奨度判定精度 | 80%以上 | 手動検証 |

### Phase 3完了時（8週間後）

| 指標 | 目標値 | 測定方法 |
|------|--------|---------|
| プラグイン追加数 | 3種類以上 | 実装済みプラグイン |
| マルチモーダル記録率 | 10%以上 | 添付ファイル数 |
| ユーザー満足度 | 80%以上 | アンケート |
| システム安定性 | 99%以上 | エラー発生率 |

---

## リスク管理

### 技術的リスク

| リスク | 影響度 | 発生確率 | 対策 |
|--------|--------|---------|------|
| セッション追跡のパフォーマンス低下 | 🔴 High | 🟡 Medium | バックグラウンド処理、非同期化 |
| V2→V3移行時のデータ損失 | 🔴 High | 🟢 Low | 十分なテスト、dry-runモード |
| 機密情報マスキングの漏れ | 🔴 High | 🟡 Medium | パターンの定期更新、ホワイトリスト |
| ML判定の精度不足 | 🟡 Medium | 🟡 Medium | ルールベースへのフォールバック |

### スケジュールリスク

| リスク | 影響度 | 発生確率 | 対策 |
|--------|--------|---------|------|
| Phase 1の遅延 | 🔴 High | 🟡 Medium | 優先度の高い機能に集中 |
| テストカバレッジ不足 | 🟡 Medium | 🟡 Medium | TDD採用、CI/CD統合 |
| ドキュメント未整備 | 🟡 Medium | 🟢 Low | 実装と並行してドキュメント作成 |

### ユーザー受容リスク

| リスク | 影響度 | 発生確率 | 対策 |
|--------|--------|---------|------|
| 自動記録への抵抗感 | 🟡 Medium | 🟡 Medium | オプトイン方式、明確な説明 |
| V3形式への不慣れ | 🟢 Low | 🟡 Medium | 移行ガイド、サンプル提供 |
| プライバシー懸念 | 🔴 High | 🟢 Low | 機密情報マスキング、ローカル保存 |

---

## 次のアクション

### 即座に実行すべきタスク

1. **ブランチ作成**
   ```bash
   git checkout -b feature/learning-record-v3-human-first
   ```

2. **ディレクトリ構造作成**
   ```bash
   mkdir -p src/commands/lr/{auto_capture,session_tracker}
   mkdir -p src/core/{session_context,sanitizer,learning_record_v3}
   mkdir -p src/cli/session
   ```

3. **依存関係追加**（Cargo.toml）
   ```toml
   [dependencies]
   lazy_static = "1.4"
   regex = "1.10"
   ```

4. **初期ファイル作成**
   ```bash
   touch src/core/session_context.rs
   touch src/core/sanitizer.rs
   touch src/core/learning_record_v3.rs
   touch src/commands/lr/auto_capture.rs
   touch src/cli/session.rs
   ```

5. **プロジェクト管理**
   ```bash
   cldev todo add "Phase 1: セッションコンテキスト追跡実装"
   cldev todo add "Phase 1: 機密情報マスキング実装"
   cldev todo add "Phase 1: V3フォーマット対応"
   ```

---

## 参考資料

- [現行システム（V2）仕様](../implementation/learning_record_v2.md)
- [反復レビュー結果](./ITERATIVE_REVIEW_LEARNING_RECORD.md)
- [セキュリティガイドライン](../security/SECURITY_GUIDELINES.md)

---

**承認**:
**レビュー日**:
**承認日**:
