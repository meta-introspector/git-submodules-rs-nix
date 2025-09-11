#!/bin/bash
# Script to view details of a GitHub Actions workflow run
if [ -z "$1" ]; then
  echo "Usage: $0 <RUN_ID> [flags]"
  exit 1
fi
gh run view "$@"