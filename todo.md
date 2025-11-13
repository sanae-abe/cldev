# cldev i18n Migration Tasks

## Phase 1: Git Commands (High Impact, Low Effort) - 1 day

### 1.1 branch.rs - Branch Type Descriptions
- [x] Add i18n keys for branch type descriptions (28-36行) | Priority: high | Context: git | Due: 2025-11-14
  - `branch-type-feature-desc`: "New feature development"
  - `branch-type-fix-desc`: "Bug fix"
  - `branch-type-hotfix-desc`: "Critical production fix"
  - `branch-type-refactor-desc`: "Code refactoring"
  - `branch-type-docs-desc`: "Documentation updates"
  - `branch-type-test-desc`: "Test additions or updates"

- [x] Add i18n keys for branch prompts (74行) | Priority: high | Context: git | Due: 2025-11-14
  - `branch-confirm-continue`: "Continue anyway? (y/N)"

- [x] Update branch.rs to use output.t() for all strings | Priority: high | Context: git | Due: 2025-11-14

- [x] Add Japanese translations to messages.json | Priority: high | Context: git | Due: 2025-11-14

- [x] Test branch command with both en/ja languages | Priority: high | Context: git | Due: 2025-11-14

### 1.2 commit.rs - Commit Type Descriptions
- [x] Add i18n keys for commit type descriptions (82-95行) | Priority: high | Context: git | Due: 2025-11-15
  - `commit-type-feat-desc`: "A new feature"
  - `commit-type-fix-desc`: "A bug fix"
  - `commit-type-docs-desc`: "Documentation only changes"
  - `commit-type-style-desc`: "Code style changes (formatting, etc.)"
  - `commit-type-refactor-desc`: "Code refactoring"
  - `commit-type-perf-desc`: "Performance improvements"
  - `commit-type-test-desc`: "Adding or updating tests"
  - `commit-type-build-desc`: "Build system or dependencies"
  - `commit-type-ci-desc`: "CI/CD configuration"
  - `commit-type-chore-desc`: "Other changes (maintenance, etc.)"
  - `commit-type-revert-desc`: "Revert a previous commit"

- [x] Update commit.rs to use output.t() for all strings | Priority: high | Context: git | Due: 2025-11-15

- [x] Add Japanese translations to messages.json | Priority: high | Context: git | Due: 2025-11-15

- [x] Test commit command with both en/ja languages | Priority: high | Context: git | Due: 2025-11-15

### 1.3 Phase 1 Integration Testing
- [x] Run full test suite for git commands | Priority: high | Context: test | Due: 2025-11-15

- [x] Verify no hardcoded strings remain in git commands | Priority: high | Context: test | Due: 2025-11-15

- [ ] Update documentation for Phase 1 completion | Priority: medium | Context: docs | Due: 2025-11-15

---

## Phase 1.5: Todo Next Command (Productivity Boost) - 0.5 days

### 1.5.1 Implement `todo next` Subcommand
- [ ] Design next task selection algorithm | Priority: high | Context: tooling | Due: 2025-11-15
  - Priority ranking: Critical > High > Medium > Low
  - Due date consideration: overdue > today > tomorrow > future
  - Status filtering: pending only (skip completed/in-progress)

- [ ] Implement `cldev todo next` command | Priority: high | Context: tooling | Due: 2025-11-15
  - Parse todo.md for all pending tasks
  - Sort by priority and due date
  - Display next task with full details (description, priority, context, due date)
  - Show task number for easy `cldev todo complete N`

- [ ] Add tests for next command | Priority: high | Context: test | Due: 2025-11-15
  - Test priority sorting
  - Test due date sorting
  - Test with empty todo list
  - Test with all tasks completed

- [ ] Update CLI args in src/cli/args.rs | Priority: high | Context: tooling | Due: 2025-11-15
  - Add `Next` variant to TodoCommand enum

- [ ] Update todo --help documentation | Priority: medium | Context: docs | Due: 2025-11-15

- [ ] Test next command with i18n migration tasks | Priority: medium | Context: test | Due: 2025-11-15

### 1.5.2 Implement `todo uncomplete` Subcommand
- [ ] Design uncomplete command behavior | Priority: medium | Context: tooling | Due: 2025-11-16
  - Revert completed task back to pending
  - Support task number or description pattern
  - Confirm before uncompleting to prevent accidents

- [ ] Implement `cldev todo uncomplete` command | Priority: medium | Context: tooling | Due: 2025-11-16
  - Parse todo.md for completed tasks
  - Match task by number or description pattern
  - Change status from completed to pending
  - Preserve all other task metadata (priority, context, due date)

- [ ] Add tests for uncomplete command | Priority: medium | Context: test | Due: 2025-11-16
  - Test uncompleting by task number
  - Test uncompleting by description pattern
  - Test error handling for non-existent tasks
  - Test confirmation prompt behavior

- [ ] Update CLI args in src/cli/args.rs | Priority: medium | Context: tooling | Due: 2025-11-16
  - Add `Uncomplete` variant to TodoCommand enum

- [ ] Update todo --help documentation | Priority: low | Context: docs | Due: 2025-11-16

---

## Phase 2: Interactive Commands (High Priority) - 3.5 days

### 2.1 feature.rs - Feature Command (2 days)
- [x] Analyze all hardcoded strings in feature.rs (52箇所) | Priority: high | Context: dev | Due: 2025-11-18

- [x] Create i18n key structure for feature command | Priority: high | Context: dev | Due: 2025-11-18
  - Requirements section
  - Acceptance criteria
  - Feature classification
  - Complexity estimation
  - Design planning
  - Testing strategy
  - Documentation requirements
  - Implementation status
  - Next steps guidance
  - Best practices

- [x] Add all feature command keys to messages.json (en) | Priority: high | Context: dev | Due: 2025-11-18

- [x] Add Japanese translations for feature command | Priority: high | Context: dev | Due: 2025-11-19

- [x] Update feature.rs to use output.t() | Priority: high | Context: dev | Due: 2025-11-19

- [x] Test feature command with both languages | Priority: high | Context: test | Due: 2025-11-19

### 2.2 fix.rs - Fix Command (1.5 days)
- [x] Analyze all hardcoded strings in fix.rs (32箇所) | Priority: high | Context: dev | Due: 2025-11-20

- [x] Create i18n key structure for fix command | Priority: high | Context: dev | Due: 2025-11-20
  - Bug classification
  - Reproducibility options
  - Root cause analysis
  - Fix patterns
  - Testing checklist
  - Commit message templates
  - Best practices

- [x] Add all fix command keys to messages.json (en) | Priority: high | Context: dev | Due: 2025-11-20

- [x] Add Japanese translations for fix command | Priority: high | Context: dev | Due: 2025-11-20

- [x] Update fix.rs to use output.t() | Priority: high | Context: dev | Due: 2025-11-21

- [x] Test fix command with both languages | Priority: high | Context: test | Due: 2025-11-21

### 2.3 Phase 2 Integration Testing
- [x] Run full test suite for dev commands | Priority: high | Context: test | Due: 2025-11-21

- [x] Verify no hardcoded strings remain in dev commands | Priority: high | Context: test | Due: 2025-11-21

- [x] Update documentation for Phase 2 completion | Priority: medium | Context: docs | Due: 2025-11-21

---

## Phase 3: Status Messages (Medium Priority) - 4 days

### 3.1 test.rs - Test Command
- [x] Analyze all hardcoded strings in test.rs (26箇所) | Priority: medium | Context: quality | Due: 2025-11-22

- [x] Create i18n key structure for test command | Priority: medium | Context: quality | Due: 2025-11-22

- [x] Add test command keys to messages.json (en/ja) | Priority: medium | Context: quality | Due: 2025-11-22

- [x] Update test.rs to use output.t() | Priority: medium | Context: quality | Due: 2025-11-25

- [x] Test command with both languages | Priority: medium | Context: test | Due: 2025-11-25

### 3.2 Other Commands
- [x] Survey remaining commands for hardcoded strings | Priority: medium | Context: all | Due: 2025-11-26
  - Total: 37 command files
  - Completed (15): branch, commit, feature, fix, test, format, lint, init, status, update_docs, maintain, review_mr, explain, merge_request, check
  - Remaining (0 with hardcoded strings): ✅ ALL COMMANDS COMPLETED
  - Priority order by hardcoded string count:
    1. ✅ init.rs (18 strings) - Already completed (uses output.i18n())
    2. ✅ test.rs (16 strings) - Already completed (Phase 3.1)
    3. ✅ status.rs (12 strings) - Completed (31 i18n keys added)
    4. ✅ update_docs.rs (12 strings) - Completed (36 i18n keys added)
    5. ✅ maintain.rs (11 strings) - Completed (26 i18n keys added)
    6. ✅ format.rs (7 strings) - Already completed
    7. ✅ review_mr.rs (7 strings) - Completed (6 i18n keys updated)
    8. ✅ explain.rs (4 strings) - Completed (4 i18n keys added)
    9. ✅ merge_request.rs (1 string) - Completed (1 i18n key added)
    10. ✅ check.rs (1 string) - Completed (1 i18n key added)

- [x] Prioritize commands by usage frequency | Priority: medium | Context: all | Due: 2025-11-26

- [x] Create i18n keys for remaining commands | Priority: medium | Context: all | Due: 2025-11-27

- [x] Add translations to messages.json | Priority: medium | Context: all | Due: 2025-11-27

- [x] Update remaining commands to use output.t() | Priority: medium | Context: all | Due: 2025-11-28

- [x] Test all commands with both languages | Priority: medium | Context: test | Due: 2025-11-28

### 3.3 Phase 3 Integration Testing
- [x] Run full test suite for all commands | Priority: medium | Context: test | Due: 2025-11-28
  - Total tests: 627 (151 lib + 88 CLI + 211 core + 55 config + 14 git + 52 learning + 45 security + 2 vuln + 9 doc)
  - All tests passed ✅
  - Removed obsolete zh/zh-TW language tests

- [x] Verify i18n coverage is complete | Priority: medium | Context: test | Due: 2025-11-28
  - en: 1079 keys ✅
  - ja: 1079 keys ✅
  - Both languages match perfectly

- [x] Update documentation for Phase 3 completion | Priority: medium | Context: docs | Due: 2025-11-28
  - Updated docs/i18n-implementation-progress.md with complete Phase 1-3 summary
  - Documented final statistics: 1079 keys per language (en/ja)
  - Documented test coverage: 627 tests all passing
  - Added implementation patterns and guidelines for future development

---

## Phase 4: Technical Content (Low Priority) - 0.5 days

### 4.1 Framework-Specific Messages
- [ ] Review technical messages in test.rs | Priority: low | Context: quality | Due: 2025-12-02

- [ ] Decide which technical messages need translation | Priority: low | Context: quality | Due: 2025-12-02

- [ ] Add selected technical messages to i18n | Priority: low | Context: quality | Due: 2025-12-02

- [ ] Test technical messages | Priority: low | Context: test | Due: 2025-12-02

---

## Final Tasks

- [x] Update CHANGELOG.md with i18n migration details | Priority: high | Context: docs | Due: 2025-12-03

- [x] Update USER_GUIDE.md with language switching instructions | Priority: high | Context: docs | Due: 2025-12-03

- [x] Create migration guide for contributors | Priority: medium | Context: docs | Due: 2025-12-03
  - Created docs/guides/I18N_MIGRATION_GUIDE.md with comprehensive patterns and best practices

- [x] Verify messages.json has exactly 900 keys for both en/ja | Priority: high | Context: test | Due: 2025-12-03
  - Note: Actual count is 1079 keys for both en/ja (verified ✅)

- [x] Run final i18n validation tests | Priority: high | Context: test | Due: 2025-12-03
  - All 627 tests passed (151 lib + 88 CLI + 211 core + 55 config + 14 git + 52 lr + 45 security + 2 vuln + 9 doc)

- [ ] Create PR for i18n migration | Priority: high | Context: git | Due: 2025-12-04

---

## Milestones

- **Week 1 (Nov 14-15)**: Phase 1 Complete - Git commands fully internationalized
- **Week 1.5 (Nov 15)**: Phase 1.5 Complete - `todo next` command implemented for productivity boost
- **Week 2 (Nov 18-21)**: Phase 2 Complete - Interactive commands (feature/fix) internationalized
- **Week 3 (Nov 22-28)**: Phase 3 Complete - All status messages internationalized
- **Week 4 (Dec 2-4)**: Phase 4 Complete - Final testing and documentation

---

## Notes

- Total estimated effort: 9.5 days (9 days i18n + 0.5 days todo next)
- i18n key naming convention: `{command}-{category}-{detail}`
- All translations must be added to both `en` and `ja` in messages.json
- Use `output.t("key")` for simple strings
- Use `output.t_format("key", "placeholder", "value")` for formatted strings
- Test each phase before moving to the next
- Maintain backward compatibility during migration
- Total tasks: 57 (49 i18n + 6 todo next + 2 integration)
