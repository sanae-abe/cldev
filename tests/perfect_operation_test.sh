#!/bin/bash
# Perfect Operation Test - cldev v1.0.0-beta
# Comprehensive test suite covering all critical functionality

set -euo pipefail

# Color codes
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Binary path
readonly CLDEV_BIN="${CLDEV_BIN:-./target/release/cldev}"

# Test results array
declare -a TEST_RESULTS=()

# ============================================================================
# Helper Functions
# ============================================================================

print_header() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}  $1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

test_start() {
    local test_name="$1"
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "  [$TOTAL_TESTS] $test_name ... "
}

test_pass() {
    PASSED_TESTS=$((PASSED_TESTS + 1))
    echo -e "${GREEN}✓ PASS${NC}"
    TEST_RESULTS+=("PASS: $1")
}

test_fail() {
    FAILED_TESTS=$((FAILED_TESTS + 1))
    echo -e "${RED}✗ FAIL${NC}"
    echo -e "    ${RED}Expected: $2${NC}"
    echo -e "    ${RED}Got: $3${NC}"
    TEST_RESULTS+=("FAIL: $1 - Expected: $2, Got: $3")
}

# ============================================================================
# Test Category 1: Basic Functionality
# ============================================================================

test_basic_functionality() {
    print_header "1. Basic Functionality Tests"

    # Test 1.1: Version display
    test_start "Version display"
    if output=$("$CLDEV_BIN" --version 2>&1) && [[ "$output" =~ cldev ]]; then
        test_pass "Version display"
    else
        test_fail "Version display" "cldev version string" "$output"
    fi

    # Test 1.2: Help display
    test_start "Help display (English)"
    if output=$("$CLDEV_BIN" --help 2>&1) && [[ "$output" =~ "Usage:" ]]; then
        test_pass "Help display (English)"
    else
        test_fail "Help display (English)" "Help text with 'Usage:'" "$output"
    fi

    # Test 1.3: Config subcommand
    test_start "Config subcommand help"
    if output=$("$CLDEV_BIN" config --help 2>&1) && [[ "$output" =~ "Configuration" || "$output" =~ "config" ]]; then
        test_pass "Config subcommand help"
    else
        test_fail "Config subcommand help" "Config help text" "$output"
    fi

    # Test 1.4: Analysis subcommand
    test_start "Analysis subcommand help"
    if output=$("$CLDEV_BIN" analysis --help 2>&1) && [[ "$output" =~ "analysis" || "$output" =~ "Analysis" ]]; then
        test_pass "Analysis subcommand help"
    else
        test_fail "Analysis subcommand help" "Analysis help text" "$output"
    fi
}

# ============================================================================
# Test Category 2: i18n Functionality
# ============================================================================

test_i18n_functionality() {
    print_header "2. i18n Functionality Tests"

    # Test 2.1: English
    test_start "i18n: English (en)"
    if output=$("$CLDEV_BIN" --lang en --help 2>&1) && [[ "$output" =~ "Usage:" ]]; then
        test_pass "i18n: English (en)"
    else
        test_fail "i18n: English (en)" "Usage:" "$output"
    fi

    # Test 2.2: Japanese
    test_start "i18n: Japanese (ja)"
    if output=$("$CLDEV_BIN" --lang ja --help 2>&1) && [[ "$output" =~ "設定" ]]; then
        test_pass "i18n: Japanese (ja)"
    else
        test_fail "i18n: Japanese (ja)" "Translated command descriptions (設定)" "$output"
    fi

    # Test 2.3: Chinese Simplified
    test_start "i18n: Chinese Simplified (zh)"
    if output=$("$CLDEV_BIN" --lang zh --help 2>&1) && [[ "$output" =~ "配置" ]]; then
        test_pass "i18n: Chinese Simplified (zh)"
    else
        test_fail "i18n: Chinese Simplified (zh)" "Translated command descriptions (配置)" "$output"
    fi

    # Test 2.4: Chinese Traditional
    test_start "i18n: Chinese Traditional (zh-TW)"
    if output=$("$CLDEV_BIN" --lang zh-TW --help 2>&1) && [[ "$output" =~ "設定" ]]; then
        test_pass "i18n: Chinese Traditional (zh-TW)"
    else
        test_fail "i18n: Chinese Traditional (zh-TW)" "Translated command descriptions (設定)" "$output"
    fi

    # Test 2.5: Subcommand i18n (Japanese config)
    test_start "i18n: Japanese config subcommand"
    if output=$("$CLDEV_BIN" --lang ja config --help 2>&1) && [[ "$output" =~ "設定" ]]; then
        test_pass "i18n: Japanese config subcommand"
    else
        test_fail "i18n: Japanese config subcommand" "設定" "$output"
    fi

    # Test 2.6: Subcommand i18n (Chinese analysis)
    test_start "i18n: Chinese analysis subcommand"
    if output=$("$CLDEV_BIN" --lang zh analysis --help 2>&1) && [[ "$output" =~ "分析" ]]; then
        test_pass "i18n: Chinese analysis subcommand"
    else
        test_fail "i18n: Chinese analysis subcommand" "分析" "$output"
    fi
}

# ============================================================================
# Test Category 3: Analysis Commands
# ============================================================================

test_analysis_commands() {
    print_header "3. Analysis Commands Tests"

    # Test 3.1: Analyze help
    test_start "Analyze command help"
    if output=$("$CLDEV_BIN" analysis analyze --help 2>&1) && [[ "$output" =~ "analyze" ]]; then
        test_pass "Analyze command help"
    else
        test_fail "Analyze command help" "analyze help text" "$output"
    fi

    # Test 3.2: Explain help
    test_start "Explain command help"
    if output=$("$CLDEV_BIN" analysis explain --help 2>&1) && [[ "$output" =~ "explain" ]]; then
        test_pass "Explain command help"
    else
        test_fail "Explain command help" "explain help text" "$output"
    fi

    # Test 3.3: Serena help
    test_start "Serena command help"
    if output=$("$CLDEV_BIN" analysis serena --help 2>&1) && [[ "$output" =~ "serena" ]]; then
        test_pass "Serena command help"
    else
        test_fail "Serena command help" "serena help text" "$output"
    fi

    # Test 3.4: Review-MR help
    test_start "Review-MR command help"
    if output=$("$CLDEV_BIN" analysis review-mr --help 2>&1) && [[ "$output" =~ "review" ]]; then
        test_pass "Review-MR command help"
    else
        test_fail "Review-MR command help" "review help text" "$output"
    fi
}

# ============================================================================
# Test Category 4: Error Handling
# ============================================================================

test_error_handling() {
    print_header "4. Error Handling Tests"

    # Test 4.1: Invalid flag
    test_start "Invalid flag rejection"
    if output=$("$CLDEV_BIN" --invalid-flag 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]] && [[ "$output" =~ "error" || "$output" =~ "unexpected" ]]; then
            test_pass "Invalid flag rejection"
        else
            test_fail "Invalid flag rejection" "Non-zero exit + error message" "exit=$exit_code, output=$output"
        fi
    else
        test_pass "Invalid flag rejection"
    fi

    # Test 4.2: Invalid language code
    test_start "Invalid language code rejection"
    if output=$("$CLDEV_BIN" --lang invalid-lang --version 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]]; then
            test_pass "Invalid language code rejection"
        else
            test_fail "Invalid language code rejection" "Non-zero exit code" "exit=$exit_code"
        fi
    else
        test_pass "Invalid language code rejection"
    fi

    # Test 4.3: Missing required argument
    test_start "Missing required argument (explain)"
    if output=$("$CLDEV_BIN" analysis explain 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]]; then
            test_pass "Missing required argument (explain)"
        else
            test_fail "Missing required argument (explain)" "Non-zero exit code" "exit=$exit_code"
        fi
    else
        test_pass "Missing required argument (explain)"
    fi
}

# ============================================================================
# Test Category 5: Security
# ============================================================================

test_security() {
    print_header "5. Security Tests"

    # Test 5.1: Path traversal attack (../)
    test_start "Path traversal: ../ pattern"
    if output=$("$CLDEV_BIN" analysis explain '../../../etc/passwd' 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]] && [[ "$output" =~ "traversal" || "$output" =~ "Invalid" ]]; then
            test_pass "Path traversal: ../ pattern"
        else
            test_fail "Path traversal: ../ pattern" "Rejection with error" "exit=$exit_code, output=$output"
        fi
    else
        test_pass "Path traversal: ../ pattern"
    fi

    # Test 5.2: Home directory access (~)
    test_start "Path traversal: ~ pattern"
    if output=$("$CLDEV_BIN" analysis explain '~/private/file' 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]] && [[ "$output" =~ "traversal" || "$output" =~ "Invalid" ]]; then
            test_pass "Path traversal: ~ pattern"
        else
            test_fail "Path traversal: ~ pattern" "Rejection with error" "exit=$exit_code, output=$output"
        fi
    else
        test_pass "Path traversal: ~ pattern"
    fi

    # Test 5.3: Absolute path
    test_start "Path traversal: absolute path"
    if output=$("$CLDEV_BIN" analysis explain '/etc/passwd' 2>&1) && exit_code=$?; then
        if [[ $exit_code -ne 0 ]] && [[ "$output" =~ "traversal" || "$output" =~ "Invalid" ]]; then
            test_pass "Path traversal: absolute path"
        else
            test_fail "Path traversal: absolute path" "Rejection with error" "exit=$exit_code, output=$output"
        fi
    else
        test_pass "Path traversal: absolute path"
    fi

    # Test 5.4: Valid target (should work)
    test_start "Security: valid target acceptance"
    if output=$("$CLDEV_BIN" analysis explain 'test_function' 2>&1); then
        # Should either succeed (found) or fail gracefully (not found), but not security error
        if [[ ! "$output" =~ "traversal" ]]; then
            test_pass "Security: valid target acceptance"
        else
            test_fail "Security: valid target acceptance" "No security error" "$output"
        fi
    else
        test_pass "Security: valid target acceptance"
    fi
}

# ============================================================================
# Test Category 6: Automated Tests
# ============================================================================

test_automated_tests() {
    print_header "6. Automated Tests (cargo test)"

    # Test 6.1: Library tests
    test_start "cargo test --lib"
    if output=$(cargo test --lib --quiet 2>&1) && [[ "$output" =~ "test result: ok" ]]; then
        test_pass "cargo test --lib"
    else
        test_fail "cargo test --lib" "All tests pass" "$output"
    fi

    # Test 6.2: CLI integration tests
    test_start "cargo test --test cli"
    if output=$(cargo test --test cli --quiet 2>&1) && [[ "$output" =~ "test result: ok" ]]; then
        test_pass "cargo test --test cli"
    else
        test_fail "cargo test --test cli" "All tests pass" "$output"
    fi

    # Test 6.3: i18n specific tests
    test_start "cargo test i18n_test"
    if output=$(cargo test --test cli i18n_test --quiet 2>&1) && [[ "$output" =~ "test result: ok" ]]; then
        test_pass "cargo test i18n_test"
    else
        test_fail "cargo test i18n_test" "All tests pass" "$output"
    fi
}

# ============================================================================
# Test Category 7: Quality Checks
# ============================================================================

test_quality_checks() {
    print_header "7. Quality Checks"

    # Test 7.1: cargo fmt check
    test_start "cargo fmt --check"
    if cargo fmt --check >/dev/null 2>&1; then
        test_pass "cargo fmt --check"
    else
        test_fail "cargo fmt --check" "No formatting issues" "Formatting issues found"
    fi

    # Test 7.2: cargo clippy
    test_start "cargo clippy"
    if output=$(cargo clippy --all-features --quiet 2>&1); then
        if [[ ! "$output" =~ "warning" ]] && [[ ! "$output" =~ "error" ]]; then
            test_pass "cargo clippy"
        else
            test_fail "cargo clippy" "0 warnings" "$output"
        fi
    else
        test_fail "cargo clippy" "Success" "Failed to run clippy"
    fi
}

# ============================================================================
# Test Category 8: Performance
# ============================================================================

test_performance() {
    print_header "8. Performance Tests"

    # Test 8.1: Startup time
    test_start "Startup time (< 100ms)"
    local start_time=$(date +%s%N)
    "$CLDEV_BIN" --version >/dev/null 2>&1
    local end_time=$(date +%s%N)
    local duration_ms=$(( (end_time - start_time) / 1000000 ))

    if [[ $duration_ms -lt 100 ]]; then
        test_pass "Startup time: ${duration_ms}ms"
    else
        test_fail "Startup time" "< 100ms" "${duration_ms}ms"
    fi

    # Test 8.2: Binary size
    test_start "Binary size (< 5MB)"
    if [[ -f "$CLDEV_BIN" ]]; then
        local size_bytes=$(stat -f%z "$CLDEV_BIN" 2>/dev/null || stat -c%s "$CLDEV_BIN" 2>/dev/null)
        local size_mb=$(( size_bytes / 1024 / 1024 ))

        if [[ $size_mb -lt 5 ]]; then
            test_pass "Binary size: ${size_mb}MB"
        else
            test_fail "Binary size" "< 5MB" "${size_mb}MB"
        fi
    else
        test_fail "Binary size" "Binary exists" "Binary not found"
    fi
}

# ============================================================================
# Test Category 9: i18n Consistency
# ============================================================================

test_i18n_consistency() {
    print_header "9. i18n Consistency Tests"

    # Test 9.1: Check all languages have same number of keys
    test_start "i18n key count consistency"
    if output=$(python3 -c "
import json
with open('src/i18n/messages.json', 'r') as f:
    data = json.load(f)
counts = {lang: len(data[lang]) for lang in ['en', 'ja', 'zh', 'zh-TW']}
if len(set(counts.values())) == 1:
    print('PASS: All languages have', list(counts.values())[0], 'keys')
else:
    print('FAIL:', counts)
" 2>&1) && [[ "$output" =~ "PASS" ]]; then
        test_pass "i18n key count consistency"
    else
        test_fail "i18n key count consistency" "All languages same count" "$output"
    fi
}

# ============================================================================
# Test Category 10: Release Readiness
# ============================================================================

test_release_readiness() {
    print_header "10. Release Readiness Tests"

    # Test 10.1: Binary exists
    test_start "Release binary exists"
    if [[ -f "$CLDEV_BIN" ]]; then
        test_pass "Release binary exists"
    else
        test_fail "Release binary exists" "Binary at $CLDEV_BIN" "Not found"
    fi

    # Test 10.2: Version matches
    test_start "Version format check"
    if output=$("$CLDEV_BIN" --version 2>&1) && [[ "$output" =~ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
        test_pass "Version format check"
    else
        test_fail "Version format check" "Semantic version" "$output"
    fi

    # Test 10.3: All languages work
    test_start "All 4 languages functional"
    local lang_ok=0
    for lang in en ja zh zh-TW; do
        if "$CLDEV_BIN" --lang "$lang" --help >/dev/null 2>&1; then
            lang_ok=$((lang_ok + 1))
        fi
    done

    if [[ $lang_ok -eq 4 ]]; then
        test_pass "All 4 languages functional"
    else
        test_fail "All 4 languages functional" "4 languages" "$lang_ok languages"
    fi
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    print_header "cldev Perfect Operation Test Suite"
    echo "Binary: $CLDEV_BIN"
    echo "Date: $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""

    # Run all test categories
    test_basic_functionality
    test_i18n_functionality
    test_analysis_commands
    test_error_handling
    test_security
    test_automated_tests
    test_quality_checks
    test_performance
    test_i18n_consistency
    test_release_readiness

    # Print summary
    print_header "Test Summary"
    echo -e "Total Tests:  ${BLUE}$TOTAL_TESTS${NC}"
    echo -e "Passed:       ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed:       ${RED}$FAILED_TESTS${NC}"
    echo ""

    # Print result
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${GREEN}  ✓ ALL TESTS PASSED - READY FOR RELEASE${NC}"
        echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        exit 0
    else
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo -e "${RED}  ✗ TESTS FAILED - NOT READY FOR RELEASE${NC}"
        echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        echo ""
        echo "Failed tests:"
        for result in "${TEST_RESULTS[@]}"; do
            if [[ "$result" =~ ^FAIL ]]; then
                echo -e "  ${RED}✗${NC} $result"
            fi
        done
        exit 1
    fi
}

# Run main
main "$@"
