#!/bin/sh
#
# Setup script for Git hooks
# Run this after cloning the repository to install pre-commit hooks
#

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "Setting up Git hooks for cldev..."

# Create pre-commit hook
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/sh
#
# Pre-commit hook that automatically formats Rust code with cargo fmt
# and stages the formatted files before commit.
#

# Check if there are any staged Rust files
if ! git diff --cached --name-only | grep -q '\.rs$'; then
    # No Rust files staged, skip formatting
    exit 0
fi

# Run cargo fmt
echo "Running cargo fmt..."
if ! cargo fmt --all; then
    echo "Error: cargo fmt failed"
    exit 1
fi

# Get the list of staged Rust files
STAGED_RUST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$')

# If there are staged Rust files, re-stage them after formatting
if [ -n "$STAGED_RUST_FILES" ]; then
    echo "Re-staging formatted Rust files..."
    echo "$STAGED_RUST_FILES" | xargs git add
fi

echo "✓ Rust files formatted successfully"
exit 0
EOF

# Make the hook executable
chmod +x "$HOOKS_DIR/pre-commit"

echo "✓ Pre-commit hook installed successfully"
echo ""
echo "The hook will automatically run 'cargo fmt' before each commit."
echo "To bypass the hook temporarily, use: git commit --no-verify"
