#!/bin/bash
# Test script to verify --quiet option properly suppresses non-error output

set -e

echo "Testing --quiet option implementation..."
echo ""

# Test 1: Completions installation instructions
echo "Test 1: Completions installation instructions"
echo "  Normal mode (should show instructions):"
./target/debug/cldev completions bash --install 2>&1 | grep -q "Completion Installation" && echo "    ✓ Instructions shown" || echo "    ✗ Instructions NOT shown"

echo "  Quiet mode (should NOT show instructions):"
./target/debug/cldev --quiet completions bash --install 2>&1 | grep -q "Completion Installation" && echo "    ✗ Instructions shown (BUG)" || echo "    ✓ Instructions suppressed"

echo ""

# Test 2: Config check output
echo "Test 2: Config check (if config exists)"
if [ -f "$HOME/.config/cldev/config.toml" ]; then
    echo "  Normal mode (should show output):"
    OUTPUT=$(./target/debug/cldev config check 2>&1 | wc -l)
    if [ "$OUTPUT" -gt 5 ]; then
        echo "    ✓ Output shown ($OUTPUT lines)"
    else
        echo "    ✗ Output too short ($OUTPUT lines)"
    fi

    echo "  Quiet mode (should suppress most output):"
    QUIET_OUTPUT=$(./target/debug/cldev --quiet config check 2>&1 | wc -l)
    if [ "$QUIET_OUTPUT" -lt "$OUTPUT" ]; then
        echo "    ✓ Output suppressed ($QUIET_OUTPUT lines vs $OUTPUT lines)"
    else
        echo "    ✗ Output NOT suppressed ($QUIET_OUTPUT lines vs $OUTPUT lines)"
    fi
else
    echo "  Skipped (no config file)"
fi

echo ""
echo "Testing complete!"
echo ""
echo "Note: The --quiet flag should:"
echo "  - Suppress all success/info/warning messages"
echo "  - Still show error messages"
echo "  - Allow essential output (like completion scripts to stdout)"
