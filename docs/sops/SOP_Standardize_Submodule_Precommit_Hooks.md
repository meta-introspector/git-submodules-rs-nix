# SOP: Standardizing Pre-commit Hooks for All Submodules

## Objective
To establish a consistent and automated process for applying and maintaining pre-commit Git hooks across all submodules within the project, ensuring code quality and adherence to standards.

## Audience
Project maintainers, submodule developers, and contributors.

## Prerequisites
*   Basic understanding of Git and Git hooks.
*   Familiarity with shell scripting.
*   `shellcheck` installed and available in the development environment.

## Procedure

### 1. Centralized Pre-commit Hook Template
Maintain a centralized template for the desired pre-commit hook in the main repository. This template will serve as the single source of truth for all submodule pre-commit hooks.

**Example Template (`.githooks/pre-commit-template`):**
```bash
#!/usr/bin/env bash

# This hook runs shellcheck on all staged shell scripts.

# Get all staged shell scripts
# This looks for files that are staged for commit and have a .sh extension
# or contain a shebang line indicating they are bash scripts.
# It also excludes files in .git/hooks/ to avoid shellcheck on itself.
SHELL_SCRIPTS=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.sh$|^(#!)?/(usr/bin/env )?bash' | grep -v '\.git/hooks/')

if [ -z "$SHELL_SCRIPTS" ]; then
  echo "No shell scripts to shellcheck."
  exit 0
fi

echo "Running shellcheck on staged shell scripts..."

# Run shellcheck on each script
for SCRIPT in $SHELL_SCRIPTS; do
  if ! shellcheck "$SCRIPT"; then
    echo "Shellcheck failed for $SCRIPT. Aborting commit."
    exit 1
  fi
done

echo "Shellcheck passed for all staged shell scripts."
exit 0
```

### 2. Automated Hook Deployment (Initial Setup)
For each submodule, during its initial setup or when a new submodule is added:

a. **Copy the template:** Copy the centralized `pre-commit-template` to the submodule's `.git/hooks/pre-commit` directory.
   ```bash
   cp /path/to/main/repo/.githooks/pre-commit-template /path/to/submodule/.git/hooks/pre-commit
   ```
b. **Make executable:** Ensure the copied hook is executable.
   ```bash
   chmod +x /path/to/submodule/.git/hooks/pre-commit
   ```

### 3. Maintaining Hook Consistency (Updates)
To update pre-commit hooks across all submodules when the centralized template changes:

a. **Iterate through submodules:** Use a script to loop through all submodules.
b. **Update hook:** For each submodule, copy the latest template and make it executable.

**Example Update Script (`update-submodule-hooks.sh`):**
```bash
#!/usr/bin/env bash

set -e

MAIN_REPO_HOOKS_DIR=".githooks"
PRE_COMMIT_TEMPLATE="${MAIN_REPO_HOOKS_DIR}/pre-commit-template"

if [ ! -f "$PRE_COMMIT_TEMPLATE" ]; then
  echo "Error: Pre-commit template not found at $PRE_COMMIT_TEMPLATE"
  exit 1
fi

git submodule foreach --recursive \
  'echo "Updating pre-commit hook in $name..."\
  mkdir -p "$toplevel/$name/.git/hooks"\
  cp "$toplevel/$PRE_COMMIT_TEMPLATE" "$toplevel/$name/.git/hooks/pre-commit"\
  chmod +x "$toplevel/$name/.git/hooks/pre-commit"\
  echo "Done."\
'

echo "All submodule pre-commit hooks updated."
```

### 4. Ensuring `shellcheck` Availability
Ensure that `shellcheck` is available in the development environment for all contributors. This can be achieved by:
*   Including `shellcheck` in the project's `flake.nix` or `shell.nix` for Nix-based environments.
*   Documenting `shellcheck` as a required dependency in the project's `README.md`.

## Benefits
*   **Consistency:** All submodules will adhere to the same code quality standards for shell scripts.
*   **Automation:** Reduces manual effort in setting up and updating hooks.
*   **Early Bug Detection:** Catches common shell scripting errors before they are committed.
*   **Improved Code Quality:** Encourages writing cleaner and more robust shell scripts.
