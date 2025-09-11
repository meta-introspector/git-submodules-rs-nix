#!/bin/bash
# Script to view details of a GitHub Issue
if [ -z "$1" ]; then
  echo "Usage: $0 <ISSUE_NUMBER>"
  exit 1
fi
gh issue view "$@"