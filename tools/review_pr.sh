#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "Usage: $0 <pr_number>"
  exit 1
fi

pr_number=$1

echo "### Viewing PR #$pr_number details"
gh pr view "$pr_number"

echo "### Viewing PR #$pr_number diff"
gh pr diff "$pr_number"

echo "### Viewing PR #$pr_number comments"
gh pr view "$pr_number" --comments
