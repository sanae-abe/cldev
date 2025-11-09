//! Auto-capture logic for learning records
//!
//! Analyzes session context and determines whether to create a learning record automatically.
//! Uses composite scoring based on errors, duration, files, commands, and todos.

use crate::core::{sanitize_text, LearningRecordV3, RecordStatus, SessionContext};
use chrono::Local;

/// Recommendation level for auto-capture
#[derive(Debug, Clone, PartialEq)]
pub enum RecordLevel {
    /// Full record with user editing (score >= 0.7)
    Full,
    /// Background indexing only (score 0.3-0.7)
    Background,
    /// Skip recording (score < 0.3)
    Skip,
}

/// Auto-capture recommendation
#[derive(Debug, Clone)]
pub struct RecordRecommendation {
    pub level: RecordLevel,
    pub score: f64,
    pub reason: String,
}

/// Analyze session and recommend recording level
pub fn analyze_session(ctx: &SessionContext) -> RecordRecommendation {
    let mut score = 0.0;
    let mut reasons = Vec::new();

    // 1. Error count (weight: 0.3)
    let error_count = ctx.unresolved_errors_count();
    let error_score = match error_count {
        0 => 0.0,
        1..=2 => 0.15,
        3..=5 => 0.25,
        _ => 0.3,
    };
    score += error_score;
    if error_count > 0 {
        reasons.push(format!("{} errors", error_count));
    }

    // 2. Duration (weight: 0.25)
    let duration = ctx.duration_minutes();
    let duration_score = match duration {
        0..=5 => 0.0,
        6..=15 => 0.1,
        16..=30 => 0.2,
        _ => 0.25,
    };
    score += duration_score;
    if duration > 10 {
        reasons.push(format!("{}min work", duration));
    }

    // 3. File changes (weight: 0.2)
    let file_count = ctx.unique_files_modified();
    let file_score = match file_count {
        0..=2 => 0.0,
        3..=5 => 0.1,
        6..=10 => 0.15,
        _ => 0.2,
    };
    score += file_score;
    if file_count > 3 {
        reasons.push(format!("{} files", file_count));
    }

    // 4. Command failures (weight: 0.15)
    let failed_count = ctx.failed_commands_count();
    let command_score = match failed_count {
        0 => 0.0,
        1..=2 => 0.08,
        3..=5 => 0.12,
        _ => 0.15,
    };
    score += command_score;
    if failed_count > 1 {
        reasons.push(format!("{} failures", failed_count));
    }

    // 5. Todo completion (weight: 0.1)
    let todo_count = ctx.completed_todos_count();
    let todo_score = match todo_count {
        0 => 0.0,
        1..=2 => 0.05,
        _ => 0.1,
    };
    score += todo_score;
    if todo_count > 0 {
        reasons.push(format!("{} todos", todo_count));
    }

    // Determine level
    let level = if score >= 0.7 {
        RecordLevel::Full
    } else if score >= 0.3 {
        RecordLevel::Background
    } else {
        RecordLevel::Skip
    };

    let reason = if reasons.is_empty() {
        "Minimal activity".to_string()
    } else {
        reasons.join(", ")
    };

    RecordRecommendation {
        level,
        score,
        reason,
    }
}

/// Generate markdown for Level 2 (semi-auto) record
pub fn generate_level2_markdown(
    ctx: &SessionContext,
    rec: &RecordRecommendation,
) -> LearningRecordV3 {
    let timestamp = Local::now();
    let id = format!(
        "auto-{}-{}",
        timestamp.format("%Y%m%d-%H%M%S"),
        ctx.session_id
    );

    // Extract errors
    let error_summary = if ctx.errors_encountered.is_empty() {
        "No errors encountered.".to_string()
    } else {
        ctx.errors_encountered
            .iter()
            .take(3)
            .map(|e| {
                format!(
                    "- **{}**: {}",
                    e.error_type,
                    sanitize_text(&e.message).sanitized
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Extract todos
    let todo_summary = if ctx.todo_history.is_empty() {
        "No todos tracked.".to_string()
    } else {
        ctx.todo_history
            .iter()
            .rev()
            .take(5)
            .map(|t| {
                format!(
                    "- [{}] {}",
                    if matches!(t.status, crate::core::TodoStatus::Completed) {
                        "x"
                    } else {
                        " "
                    },
                    t.content
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // Extract file changes
    let file_changes = if ctx.files_modified.is_empty() {
        "No files modified.".to_string()
    } else {
        ctx.files_modified
            .iter()
            .take(10)
            .map(|f| {
                format!(
                    "- `{}` (+{} -{} lines)",
                    f.file_path, f.lines_added, f.lines_deleted
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let markdown_body = format!(
        r#"# 【自動生成】セッション記録

**信頼度**: {:.0}% | **推奨理由**: {}

## 🔍 検出された問題

{}

## 📝 実行された作業（TodoWrite履歴から抽出）

{}

## 📊 変更されたファイル

{}

## 💡 AIによる学びの抽出

*（以下は自動抽出の提案です。人間が確認・編集してください）*

- エラーパターンから学習可能な点を抽出
- 類似問題の再発防止策を検討
- 効率化できた作業フローを記録

---

<!-- 以下、人間が追記する領域 -->

## ✍️ 追加メモ

（ここに追加のメモを記入してください）

## 🔗 関連リンク

（関連するドキュメント、PRなどのリンク）
"#,
        rec.score * 100.0,
        rec.reason,
        error_summary,
        todo_summary,
        file_changes
    );

    let mut record = LearningRecordV3::new(id, markdown_body);
    record.auto_generated = true;
    record.confidence = Some(rec.score);
    record.duration_min = Some(ctx.duration_minutes());
    record.status = if ctx.unresolved_errors_count() > 0 {
        RecordStatus::Pending
    } else {
        RecordStatus::Resolved
    };

    // Extract tags from errors
    let mut tags: Vec<String> = ctx
        .errors_encountered
        .iter()
        .map(|e| e.error_type.to_lowercase())
        .collect();
    tags.sort();
    tags.dedup();
    record.tags = tags;

    record
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{CommandRecord, ErrorCapture, FileModification, ModificationType};

    fn create_test_context() -> SessionContext {
        SessionContext::new("test-session".to_string())
    }

    #[test]
    fn test_analyze_minimal_session() {
        let ctx = create_test_context();
        let rec = analyze_session(&ctx);
        assert_eq!(rec.level, RecordLevel::Skip);
        assert!(rec.score < 0.3);
    }

    #[test]
    fn test_analyze_with_errors() {
        let mut ctx = create_test_context();
        for i in 0..3 {
            ctx.add_error(ErrorCapture {
                timestamp: Local::now(),
                error_type: "TestError".to_string(),
                message: format!("Error {}", i),
                context: None,
                resolved: false,
            });
        }
        let rec = analyze_session(&ctx);
        assert!(rec.score >= 0.25);
        assert!(rec.reason.contains("errors"));
    }

    #[test]
    fn test_analyze_background_level() {
        let mut ctx = create_test_context();
        // Add moderate activity to reach 0.3-0.7 range
        // Add 2 errors (0.15) + 6 files (0.15) = 0.30
        for i in 0..2 {
            ctx.add_error(ErrorCapture {
                timestamp: Local::now(),
                error_type: "TestError".to_string(),
                message: format!("Test error {}", i),
                context: None,
                resolved: false,
            });
        }
        for i in 0..6 {
            ctx.add_file_modification(FileModification {
                file_path: format!("test{}.rs", i),
                modification_type: ModificationType::Modified,
                lines_added: 10,
                lines_deleted: 5,
                timestamp: Local::now(),
            });
        }
        let rec = analyze_session(&ctx);
        assert!(rec.score >= 0.3 && rec.score < 0.7);
        assert_eq!(rec.level, RecordLevel::Background);
    }

    #[test]
    fn test_generate_level2_markdown() {
        let mut ctx = create_test_context();
        ctx.add_error(ErrorCapture {
            timestamp: Local::now(),
            error_type: "CompileError".to_string(),
            message: "Test compile error".to_string(),
            context: None,
            resolved: false,
        });

        let rec = RecordRecommendation {
            level: RecordLevel::Full,
            score: 0.85,
            reason: "3 errors, 45min work".to_string(),
        };

        let record = generate_level2_markdown(&ctx, &rec);
        assert!(record.auto_generated);
        assert_eq!(record.confidence, Some(0.85));
        assert!(record.markdown_body.contains("CompileError"));
        assert!(record.markdown_body.contains("85%"));
    }
}
