#!/usr/bin/env bash

# This script runs a headless batch test for the Emacs configuration.

# Get the directory of the current script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Path to the Emacs Lisp test file
EMACS_LISP_TEST_FILE="$SCRIPT_DIR/docs/qa/emacs_batch_test.el"

# Navigate to the terlar-emacs-config directory
# This is where the flake.nix for the Emacs config resides
cd "$SCRIPT_DIR/.emacs.d/terlar-emacs-config" || { echo "Error: Could not change to terlar-emacs-config directory."; exit 1; }

echo "Running headless Emacs configuration test..."

# Run Emacs in batch mode using nix develop
# The -l flag loads the specified Emacs Lisp file
nix develop --command "emacs --batch -l \"$EMACS_LISP_TEST_FILE\""

# Capture the exit code of the Emacs process
TEST_EXIT_CODE=$?

if [ $TEST_EXIT_CODE -eq 0 ]; then
  echo "Headless Emacs test PASSED."
else
  echo "Headless Emacs test FAILED with exit code $TEST_EXIT_CODE."
fi

exit $TEST_EXIT_CODE
