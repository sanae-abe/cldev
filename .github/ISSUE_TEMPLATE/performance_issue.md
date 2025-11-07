---
name: Performance Issue
about: Report a performance problem or regression
title: '[PERF] '
labels: performance
assignees: ''
---

## Performance Issue Description

<!-- A clear description of the performance problem -->

## Benchmark Results

<!-- Include benchmark results showing the performance issue -->

### Before

```
# Benchmark output or metrics
```

### After (if applicable)

```
# Benchmark output or metrics
```

## Steps to Reproduce

<!-- How to reproduce the performance issue -->

1.
2.
3.

## Expected Performance

<!-- What performance level do you expect? -->

## Actual Performance

<!-- What performance are you experiencing? -->

## Environment

**cldev Version:**

```
cldev --version
```

**Operating System:**

- OS:
- Architecture:
- CPU:
- RAM:

**Rust Version:**

```
rustc --version
```

## Profiling Data

<!-- If you've profiled the issue, include relevant data -->

### CPU Profile

```
# Output from cargo flamegraph or similar
```

### Memory Profile

```
# Output from valgrind, heaptrack, or similar
```

## Regression Information

<!-- If this is a regression, when did you first notice it? -->

- Last working version:
- First broken version:
- Suspected commit:

## Impact

<!-- How does this impact your workflow? -->

- [ ] Critical - Blocking production use
- [ ] High - Significantly impacts workflow
- [ ] Medium - Noticeable but workable
- [ ] Low - Minor annoyance

## Possible Solution

<!-- If you have ideas on how to fix the performance issue -->

## Additional Context

<!-- Any other relevant information -->

---

**Generated with Claude Code**

Co-Authored-By: Claude <noreply@anthropic.com>
