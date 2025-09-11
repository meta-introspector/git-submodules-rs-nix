#!/bin/bash
# Script to checkout a GitHub Pull Request locally
if [ -z "$1" ]; then
  echo "Usage: $0 <PR_NUMBER>"
  exit 1
fi
gh pr checkout "$@"