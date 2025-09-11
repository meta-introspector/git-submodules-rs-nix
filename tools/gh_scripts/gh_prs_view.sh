#!/bin/bash
# Script to view details of a GitHub Pull Request
if [ -z "$1" ]; then
  echo "Usage: $0 <PR_NUMBER>"
  exit 1
fi
gh pr view "$@"