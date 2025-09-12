#!/usr/bin/env bash

SESSION_NAME="gemini-dev-session" # Ensure this matches the session name used by run_boot.sh


# Check if the tmux session exists
if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
    # Create a new window and run nix-shell with emacs
    tmux split-window -t "$SESSION_NAME"
    tmux send-keys -t "$SESSION_NAME" "cd .emacs.d/terlar-emacs-config && nix develop --command 'emacs'" C-m
else
    echo "tmux session '$SESSION_NAME' not found. Please ensure run_boot.sh has started it."
    exit 1
fi
