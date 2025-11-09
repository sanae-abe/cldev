# アーカイブファイル

このディレクトリには、統合・更新されたファイルの旧バージョンが保管されています。

## アーカイブされたファイル

### 2025-01-09 - テスト計画書統合

**理由**: 2つのテスト計画書を1つに統合し、Phase 7機能を追加

**統合前**:
1. `MANUAL_TEST_PLAN.md` (2025-11-09作成)
   - Phase 6 Phase 2特化版
   - 44テスト項目（学習記録新コマンド中心）
   - check-file/suggest/similarのテスト詳細

2. `VERIFICATION_CHECKLIST.md` (2025-11-08作成)
   - 一般動作確認版
   - 16チェック項目（基本機能中心）
   - インストール〜セキュリティまで網羅

**統合後**:
- `../RUNTIME_TEST_PLAN.md` (2025-01-09作成)
- 57テスト項目（P0: 33, P1: 13, P2: 11）
- Phase 7機能追加（session/sanitizer/auto_capture）
- 優先度別整理（必須/重要/推奨）

**参照先**: `docs/development/RUNTIME_TEST_PLAN.md`

---

**作成日**: 2025-01-09
