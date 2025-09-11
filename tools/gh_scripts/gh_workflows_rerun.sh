#!/bin/bash
# Script to re-run a GitHub Actions workflow run
if [ -z "$1" ]; then
  echo "Usage: $0 <RUN_ID>"
  exit 1
fi
gh run rerun "$@"