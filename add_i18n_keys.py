#!/usr/bin/env python3
"""
Add dialoguer i18n keys to messages.json
"""
import json

# New keys extracted from dialoguer usage
new_keys = {
    # Refactor command
    "refactor-goal-readability": "Improve code readability/maintainability",
    "refactor-goal-dry": "Reduce code duplication (DRY principle)",
    "refactor-goal-performance": "Improve performance",
    "refactor-goal-simplify": "Simplify complex logic",
    "refactor-goal-extract": "Extract reusable components/utilities",
    "refactor-goal-type-safety": "Improve type safety",
    "refactor-goal-modern": "Update to modern patterns/best practices",
    "refactor-goal-debt": "Reduce technical debt",
    "refactor-goal-prepare": "Prepare for new features",
    "refactor-goal-smells": "Fix code smells",
    "refactor-goal-prompt": "Select refactoring goals (Space to select, Enter to confirm)",

    "refactor-scope-single-function": "Single function/method (low impact)",
    "refactor-scope-single-file": "Single file/module (medium impact)",
    "refactor-scope-multiple-files": "Multiple related files (high impact)",
    "refactor-scope-system-wide": "Cross-cutting concern (system-wide impact)",
    "refactor-scope-prompt": "Refactoring Scope",

    "refactor-file-prompt": "File",

    "refactor-test-question": "Does the target code have existing tests?",
    "refactor-continue-no-tests": "Continue without tests? (not recommended)",

    "refactor-pattern-extract-function": "Extract Function/Method",
    "refactor-pattern-extract-component": "Extract Component/Module",
    "refactor-pattern-inline": "Inline Function/Variable",
    "refactor-pattern-rename": "Rename (improve naming)",
    "refactor-pattern-move": "Move Function/Class",
    "refactor-pattern-polymorphism": "Replace Conditional with Polymorphism",
    "refactor-pattern-parameter-object": "Introduce Parameter Object",
    "refactor-pattern-constants": "Replace Magic Numbers with Constants",
    "refactor-pattern-decompose": "Decompose Conditional",
    "refactor-pattern-consolidate": "Consolidate Duplicate Code",
    "refactor-pattern-simplify": "Simplify Complex Expression",
    "refactor-pattern-guard": "Replace Nested Conditionals with Guard Clauses",
    "refactor-pattern-interface": "Extract Interface/Type",
    "refactor-pattern-pipeline": "Replace Loop with Pipeline (map/filter/reduce)",
    "refactor-pattern-prompt": "Select refactoring techniques to apply (Space to select, Enter to confirm)",

    "refactor-step-prompt": "Step {num}",

    "refactor-security-auth": "Authentication/authorization logic changes",
    "refactor-security-validation": "Input validation changes",
    "refactor-security-sanitization": "Data sanitization changes",
    "refactor-security-access": "Access control changes",
    "refactor-security-encryption": "Encryption/decryption logic changes",
    "refactor-security-api": "API endpoint changes",
    "refactor-security-none": "None of the above",
    "refactor-security-prompt": "Does this refactoring affect any of these security-sensitive areas? (Space to select, Enter to confirm)",

    "refactor-status-planning": "Planning (not started yet)",
    "refactor-status-progress": "In Progress (actively refactoring)",
    "refactor-status-testing": "Testing (refactoring done, verifying changes)",
    "refactor-status-review": "Review (ready for code review)",
    "refactor-status-completed": "Completed (merged)",
    "refactor-status-prompt": "Current Status",

    "refactor-measure-improvements": "Did you measure improvements?",
    "refactor-improvement-prompt": "Improvement",

    # Feature command
    "feature-problem-prompt": "❓ Problem Statement",
    "feature-users-prompt": "👥 Target Users",
    "feature-criterion-prompt": "Criterion {num}",

    "feature-type-ui": "New UI component/page",
    "feature-type-api": "New API endpoint/service",
    "feature-type-data": "Data model change",
    "feature-type-algorithm": "Algorithm/business logic",
    "feature-type-integration": "Integration with external service",
    "feature-type-performance": "Performance enhancement",
    "feature-type-tooling": "Developer tooling/infrastructure",
    "feature-type-security": "Security feature",
    "feature-type-accessibility": "Accessibility improvement",
    "feature-type-other": "Other",
    "feature-type-prompt": "Feature Type",

    "feature-complexity-small": "Small (< 1 day, ~1-2 files)",
    "feature-complexity-medium": "Medium (1-3 days, ~3-5 files)",
    "feature-complexity-large": "Large (3-7 days, ~6-10 files)",
    "feature-complexity-xlarge": "Extra Large (1-2 weeks, 10+ files)",
    "feature-complexity-prompt": "⚖️  Estimated Complexity",

    "feature-branch-create": "Create a new feature branch?",
    "feature-branch-name": "Branch name",

    "feature-design-ui": "UI/UX design (wireframes, mockups)",
    "feature-design-schema": "Database schema changes",
    "feature-design-api": "API contract/interface design",
    "feature-design-state": "State management approach",
    "feature-design-architecture": "Component architecture",
    "feature-design-security": "Security considerations",
    "feature-design-performance": "Performance considerations",
    "feature-design-testing": "Testing strategy",
    "feature-design-docs": "Documentation requirements",
    "feature-design-accessibility": "Accessibility requirements",
    "feature-design-prompt": "Select relevant design considerations (Space to select, Enter to confirm)",

    "feature-file-prompt": "File",

    "feature-dependencies-question": "Will this feature require new dependencies?",
    "feature-dependency-prompt": "Dependency",

    "feature-test-unit": "Unit tests (isolated component testing)",
    "feature-test-integration": "Integration tests (component interaction)",
    "feature-test-e2e": "E2E tests (full user flow)",
    "feature-test-visual": "Visual regression tests",
    "feature-test-performance": "Performance tests",
    "feature-test-accessibility": "Accessibility tests",
    "feature-test-security": "Security tests",
    "feature-test-manual": "Manual testing checklist",
    "feature-test-prompt": "Select required test types (Space to select, Enter to confirm)",

    "feature-doc-readme": "README updates",
    "feature-doc-api": "API documentation",
    "feature-doc-comments": "Code comments/JSDoc",
    "feature-doc-guide": "User guide/tutorial",
    "feature-doc-adr": "Architecture decision record (ADR)",
    "feature-doc-migration": "Migration guide (if breaking change)",
    "feature-doc-prompt": "Select documentation to create/update (Space to select, Enter to confirm)",

    "feature-status-planning": "Planning (requirements gathered, ready to start)",
    "feature-status-progress": "In Progress (actively implementing)",
    "feature-status-testing": "Testing (implementation done, testing in progress)",
    "feature-status-review": "Review (ready for code review)",
    "feature-status-completed": "Completed (merged and deployed)",
    "feature-status-prompt": "Current Status",

    "feature-learning-prompt": "What did you learn from implementing this feature?",

    # Optimize command
    "optimize-issue-page-load": "Slow page load / rendering",
    "optimize-issue-api": "API response time",
    "optimize-issue-database": "Database query performance",
    "optimize-issue-memory": "Memory usage / leaks",
    "optimize-issue-cpu": "CPU-intensive operations",
    "optimize-issue-network": "Network requests (too many, too large)",
    "optimize-issue-bundle": "Bundle size (JavaScript/CSS)",
    "optimize-issue-image": "Image/asset loading",
    "optimize-issue-animation": "Animation/scroll performance",
    "optimize-issue-search": "Search/filtering operations",
    "optimize-issue-prompt": "Select performance issues (Space to select, Enter to confirm)",

    "optimize-baseline-question": "Have you measured the current (baseline) performance?",
    "optimize-continue-no-baseline": "Continue without baseline? (not recommended)",
    "optimize-baseline-prompt": "Baseline metric",

    "optimize-bottleneck-images": "Large/unoptimized images",
    "optimize-bottleneck-javascript": "Excessive JavaScript execution",
    "optimize-bottleneck-rerenders": "Unnecessary re-renders (React/Vue)",
    "optimize-bottleneck-blocking": "Blocking/synchronous operations",
    "optimize-bottleneck-n-plus-one": "N+1 database queries",
    "optimize-bottleneck-indexes": "Missing indexes in database",
    "optimize-bottleneck-data": "Large data transfers",
    "optimize-bottleneck-algorithms": "Inefficient algorithms (O(n²) or worse)",
    "optimize-bottleneck-memory-leaks": "Memory leaks",
    "optimize-bottleneck-requests": "Too many network requests",
    "optimize-bottleneck-libraries": "Unoptimized third-party libraries",
    "optimize-bottleneck-css": "CSS layout thrashing",
    "optimize-bottleneck-prompt": "Identified bottlenecks (Space to select, Enter to confirm)",

    "optimize-technique-code-splitting": "Code splitting / lazy loading",
    "optimize-technique-images": "Image optimization (compression, WebP, lazy load)",
    "optimize-technique-memoization": "Memoization (React.memo, useMemo, useCallback)",
    "optimize-technique-virtualization": "Virtualization (react-window, virtual scrolling)",
    "optimize-technique-debounce": "Debounce/throttle expensive operations",
    "optimize-technique-bundle-reduction": "Reduce bundle size (tree shaking, remove unused)",
    "optimize-technique-css": "Optimize CSS (remove unused, critical CSS)",
    "optimize-technique-web-workers": "Web Workers (offload CPU work)",
    "optimize-technique-service-worker": "Service Worker / caching strategy",
    "optimize-technique-query": "Database query optimization (indexes, query rewrite)",
    "optimize-technique-caching": "Caching (Redis, in-memory cache)",
    "optimize-technique-pooling": "Connection pooling",
    "optimize-technique-async": "Async/await refactoring",
    "optimize-technique-batch": "Batch operations (reduce roundtrips)",
    "optimize-technique-cdn": "CDN for static assets",
    "optimize-technique-algorithm": "Algorithm improvement (better data structures)",
    "optimize-technique-parallel": "Parallel processing",
    "optimize-technique-complexity": "Reduce computational complexity",
    "optimize-technique-resource-reuse": "Resource pooling/reuse",
    "optimize-technique-prompt": "Select optimization techniques to apply (Space to select, Enter to confirm)",

    "optimize-step-prompt": "Step {num}",
    "optimize-file-prompt": "File",

    "optimize-target-prompt": "Performance target (e.g., 'Reduce load time to < 1s', '50% faster')",

    "optimize-status-planning": "Planning (analysis done, ready to optimize)",
    "optimize-status-progress": "In Progress (implementing optimizations)",
    "optimize-status-measuring": "Measuring (optimization done, collecting metrics)",
    "optimize-status-completed": "Completed (verified improvement)",
    "optimize-status-prompt": "Current Status",

    "optimize-after-prompt": "After metric",
    "optimize-improvement-prompt": "Improvement",

    # Research command
    "research-topic-prompt": "📚 Research Topic",

    "research-motivation-learn": "Learn new technology/framework",
    "research-motivation-solve": "Solve specific problem",
    "research-motivation-evaluate": "Evaluate alternatives/options",
    "research-motivation-best-practices": "Understand best practices",
    "research-motivation-performance": "Performance optimization research",
    "research-motivation-architecture": "Architecture/design decision",
    "research-motivation-security": "Security investigation",
    "research-motivation-compatibility": "Compatibility/integration research",
    "research-motivation-trends": "Industry trends/emerging tech",
    "research-motivation-personal": "Personal skill development",
    "research-motivation-prompt": "Why are you researching this? (Space to select, Enter to confirm)",

    "research-context-prompt": "💡 Context (what prompted this research?)",

    "research-question-prompt": "Question {num}",

    "research-scope-quick": "Quick investigation (< 1 hour)",
    "research-scope-moderate": "Moderate research (1-4 hours)",
    "research-scope-deep": "Deep dive (1-2 days)",
    "research-scope-extended": "Extended research (1 week+)",
    "research-scope-prompt": "Time commitment",

    "research-activity-docs": "Read documentation/official guides",
    "research-activity-blogs": "Read blog posts/articles",
    "research-activity-videos": "Watch videos/tutorials",
    "research-activity-source": "Read source code/examples",
    "research-activity-experiment": "Hands-on experimentation",
    "research-activity-poc": "Build proof-of-concept",
    "research-activity-benchmark": "Performance benchmarking",
    "research-activity-security": "Security analysis",
    "research-activity-community": "Community research (forums, GitHub issues)",
    "research-activity-compare": "Compare alternatives/competitors",
    "research-activity-papers": "Read academic papers",
    "research-activity-experts": "Consult with experts/team",
    "research-activity-prompt": "Select research activities (Space to select, Enter to confirm)",

    "research-resource-prompt": "Resource (URL or description)",

    "research-experiment-question": "Will you do hands-on experimentation?",
    "research-experiment-prompt": "Experiment",

    "research-status-planning": "Planning (defining research scope)",
    "research-status-progress": "In Progress (actively researching)",
    "research-status-experimenting": "Experimenting (hands-on testing)",
    "research-status-analyzing": "Analyzing (synthesizing findings)",
    "research-status-completed": "Completed (research finished)",
    "research-status-prompt": "Research Status",
}

# Load existing messages.json
with open('src/i18n/messages.json', 'r', encoding='utf-8') as f:
    messages = json.load(f)

# Add new keys to English
messages['en'].update(new_keys)

# Sort keys alphabetically
for lang in messages:
    messages[lang] = dict(sorted(messages[lang].items()))

# Save back to messages.json
with open('src/i18n/messages.json', 'w', encoding='utf-8') as f:
    json.dump(messages, f, ensure_ascii=False, indent=2)

print(f"✅ Added {len(new_keys)} keys to messages.json")
print(f"New totals:")
for lang in ['en', 'ja', 'zh', 'zh-TW']:
    print(f"  {lang:6s}: {len(messages[lang])} keys")
