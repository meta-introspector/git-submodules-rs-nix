#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status.
set -e

PROJECT_ROOT="$(pwd)"
EMACS_CONFIG_PATH="${PROJECT_ROOT}/.emacs.d/terlar-emacs-config"
INIT_ORG_PATH="${EMACS_CONFIG_PATH}/init.org"
INIT_EL_PATH="${EMACS_CONFIG_PATH}/init.el"
LOG_FILE="${PROJECT_ROOT}/emacs_load_log.txt"

{
echo "--- Starting Emacs Configuration Test ---"
echo "Project Root: $PROJECT_ROOT"
echo "Emacs Config Path: $EMACS_CONFIG_PATH"
echo "Init Org Path: $INIT_ORG_PATH"
echo "Init El Path: $INIT_EL_PATH"
echo "Log File: $LOG_FILE"
echo ""
} > "$LOG_FILE"

# Step 1: Ensure submodules are updated
(
echo "Updating Git submodules..."
git submodule update --init --recursive
echo "Submodules updated."
) 2>&1 | tee -a "$LOG_FILE"

# Step 2: Tangle init.org to init.el
(
echo "Tangling init.org to init.el..."
nix develop --command bash -c "emacs --batch -l org --eval '(org-babel-tangle-file \"$INIT_ORG_PATH\")'"
echo "init.org tangled to init.el."
) 2>&1 | tee -a "$LOG_FILE"

# Step 3: Launch Emacs and log loaded features
(
echo "Launching Emacs in batch mode to log loaded features..."
nix develop --command bash -c "emacs --batch -l \"$INIT_EL_PATH\" --eval '(progn (message \"Emacs loaded init.el from submodule.\")(message \"Loaded features: %S\" feature-list))'"
echo "Emacs launch complete. Check '$LOG_FILE' for output."
) 2>&1 | tee -a "$LOG_FILE"

echo "--- Emacs Configuration Test Complete ---" | tee -a "$LOG_FILE"