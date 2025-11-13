# review_mr.rs i18n Migration Task

**Status**: Not Started
**Priority**: Medium
**Type**: Feature Enhancement / Technical Debt
**Estimated Effort**: 4-6 hours

## Background

The `/i18n-check` command revealed that `src/commands/analysis/review_mr.rs` contains **145 hardcoded strings** that need to be migrated to the i18n system.

**Current Status**:
- All other 32 commands are fully internationalized (1,079 keys for en/ja)
- review_mr.rs is the only command with significant hardcoded strings remaining

**Impact**:
- Users cannot see MR/PR review output in Japanese
- Breaks consistency with other fully internationalized commands
- Not a critical bug - functionality works correctly

## Hardcoded String Categories

Based on analysis of `src/commands/analysis/review_mr.rs:590-665`:

### 1. Section Headers (8 strings)
- "--- Summary ---" (line 598)
- "--- Security Issues ({}) ---" (line 607)
- "--- Performance Issues ({}) ---" (line 624)
- "--- Quality Issues ({}) ---" (line 641)
- "--- Recommendations ---" (line 660)
- "--- Review Decision ---" (line 668)

### 2. Summary Fields (4 strings)
- "Files changed: {}" (line 599)
- "Lines added: +{}" (line 600)
- "Lines removed: -{}" (line 601)
- "Overall risk: {:?}" (line 602)

### 3. Issue Details (~133 strings)
- Security issue formatting
- Performance issue formatting
- Quality issue formatting
- Severity labels
- Recommendation prefixes ("💡")
- "... and {} more" (line 652)

## Migration Plan

### Phase 1: Add i18n Keys to messages.json (2 hours)

**English keys** (`src/i18n/messages.json`):
```json
{
  "review-mr-summary-header": "--- Summary ---",
  "review-mr-summary-files-changed": "Files changed: {count}",
  "review-mr-summary-lines-added": "Lines added: +{count}",
  "review-mr-summary-lines-removed": "Lines removed: -{count}",
  "review-mr-summary-overall-risk": "Overall risk: {risk}",

  "review-mr-security-header": "--- Security Issues ({count}) ---",
  "review-mr-security-issue-format": "[{severity}] {category} at {file}:{line} - {description}",
  "review-mr-security-recommendation": "    💡 {text}",

  "review-mr-performance-header": "--- Performance Issues ({count}) ---",
  "review-mr-performance-issue-format": "[{severity}] {type} at {file}:{line} - {description}",
  "review-mr-performance-suggestion": "    💡 {text}",

  "review-mr-quality-header": "--- Quality Issues ({count}) ---",
  "review-mr-quality-issue-format": "[{severity}] {type} at {file}:{line} - {description}",
  "review-mr-quality-more": "    ... and {count} more",

  "review-mr-recommendations-header": "--- Recommendations ---",
  "review-mr-decision-header": "--- Review Decision ---"
}
```

**Japanese translations**:
```json
{
  "review-mr-summary-header": "--- サマリー ---",
  "review-mr-summary-files-changed": "変更ファイル数: {count}",
  "review-mr-summary-lines-added": "追加行数: +{count}",
  "review-mr-summary-lines-removed": "削除行数: -{count}",
  "review-mr-summary-overall-risk": "総合リスク: {risk}",

  "review-mr-security-header": "--- セキュリティ問題 ({count}件) ---",
  "review-mr-security-issue-format": "[{severity}] {category} ({file}:{line}) - {description}",
  "review-mr-security-recommendation": "    💡 {text}",

  "review-mr-performance-header": "--- パフォーマンス問題 ({count}件) ---",
  "review-mr-performance-issue-format": "[{severity}] {type} ({file}:{line}) - {description}",
  "review-mr-performance-suggestion": "    💡 {text}",

  "review-mr-quality-header": "--- 品質問題 ({count}件) ---",
  "review-mr-quality-issue-format": "[{severity}] {type} ({file}:{line}) - {description}",
  "review-mr-quality-more": "    ... 他 {count} 件",

  "review-mr-recommendations-header": "--- 推奨事項 ---",
  "review-mr-decision-header": "--- レビュー判定 ---"
}
```

### Phase 2: Update review_mr.rs Code (2 hours)

**Before** (line 598):
```rust
output.info("--- Summary ---");
output.info(&format!("Files changed: {}", review.summary.files_changed));
```

**After**:
```rust
output.info(&output.t("review-mr-summary-header"));
output.info(&output.t_format(
    "review-mr-summary-files-changed",
    "count",
    &review.summary.files_changed.to_string()
));
```

### Phase 3: Testing (1 hour)

```bash
# Build and test
cargo build --release --bin cldev --quiet

# Test English output
./target/release/cldev --lang en analysis review-mr 123

# Test Japanese output
./target/release/cldev --lang ja analysis review-mr 123

# Verify key counts match
python3 -c "
import json
data = json.load(open('src/i18n/messages.json'))
en_count = len(data['en'])
ja_count = len(data['ja'])
print(f'en: {en_count} keys, ja: {ja_count} keys')
assert en_count == ja_count, 'Key mismatch!'
"
```

### Phase 4: Validation (1 hour)

- Run full test suite: `cargo test --all-features`
- Verify both languages with `--help` output
- Check for any remaining hardcoded strings
- Update i18n-implementation-progress.md

## Acceptance Criteria

- [ ] All hardcoded strings in review_mr.rs migrated to i18n keys
- [ ] Both English and Japanese translations added to messages.json
- [ ] Key counts match between en/ja (should be ~1095 keys after migration)
- [ ] All 627 tests passing
- [ ] Both language help outputs verified
- [ ] No hardcoded strings detected in `/i18n-check` scan

## Dependencies

- Requires understanding of existing i18n system in `src/core/i18n.rs`
- Reference implementation: `src/commands/config/check.rs`, `src/commands/git/status.rs`
- Follow patterns from `docs/guides/I18N_MIGRATION_GUIDE.md`

## Related Issues

- i18n check report: 92.75/100 quality score
- Technical debt from incomplete i18n migration
- Consistency issue with other fully internationalized commands

## Notes

- This is **NOT a bug** - functionality works correctly in English
- Categorized as **feature enhancement** or **technical debt**
- Should be prioritized after critical bugs and before new features
- Good candidate for contributor onboarding task (well-documented pattern)

---

**Created**: 2025-11-13
**Last Updated**: 2025-11-13
**Assigned**: Unassigned
**Milestone**: v1.1.0 (post-release cleanup)
