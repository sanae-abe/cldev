#!/bin/bash
# Fix quality commands i18n - Replace t_with with t_with_vars

for file in src/commands/quality/format.rs src/commands/quality/test.rs; do
    # Add HashMap import if not present
    if ! grep -q "use std::collections::HashMap;" "$file"; then
        sed -i '' '1i\
use std::collections::HashMap;\
' "$file"
    fi

    # Replace t_with_vars patterns systematically
    perl -i -pe '
        # Pattern 1: Single variable like ("type", value)
        s/output\.t_with_vars\("([^"]+)",\s*&\[\("([^"]+)",\s*([^)]+)\)\]\)/{ let mut vars = HashMap::new(); vars.insert("$2", $3); output.t_with_vars("$1", \&vars) }/g;

        # Pattern 2: Two variables
        s/output\.t_with_vars\("([^"]+)",\s*&\[\("([^"]+)",\s*([^)]+)\),\s*\("([^"]+)",\s*([^)]+)\)\]\)/{ let mut vars = HashMap::new(); vars.insert("$2", $3); vars.insert("$4", $5); output.t_with_vars("$1", \&vars) }/g;
    ' "$file"
done

echo "Fixed i18n in quality commands"
