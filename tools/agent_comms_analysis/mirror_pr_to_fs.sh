#!/usr/bin/env bash

set -e

PR_NUMBER=$1

if [ -z "$PR_NUMBER" ]; then
  echo "Usage: $0 <PR_NUMBER>"
  exit 1
fi

echo "Processing PR #$PR_NUMBER..."

# Fetch PR details and comments
PR_DATA=$(gh pr view "$PR_NUMBER" --json title,body,author,url,createdAt,baseRefName,comments)

# Extract CRQ ID from PR title (assuming format "CRQ-XXX: ...")
PR_TITLE=$(echo "$PR_DATA" | jq -r '.title')
CRQ_ID=$(echo "$PR_TITLE" | grep -oP 'CRQ-\d+') # Extract CRQ-XXX

if [ -z "$CRQ_ID" ]; then
  echo "Warning: Could not extract CRQ ID from PR title: '$PR_TITLE'. Using PR number as fallback."
  CRQ_ID="PR-$PR_NUMBER"
fi

BASE_DIR="comms/git/$CRQ_ID"
RESPONSES_DIR="$BASE_DIR/coderabbitai/responses"

# Create directories
mkdir -p "$RESPONSES_DIR"

# --- Write PR description/metadata ---
PR_BODY=$(echo "$PR_DATA" | jq -r '.body')
PR_AUTHOR=$(echo "$PR_DATA" | jq -r '.author.login')
PR_URL=$(echo "$PR_DATA" | jq -r '.url')
PR_CREATED_AT=$(echo "$PR_DATA" | jq -r '.createdAt')
PR_BASE_BRANCH=$(echo "$PR_DATA" | jq -r '.baseRefName')

PR_DESCRIPTION_FILE="$BASE_DIR/pr_description.md"
{
  echo "---"
  echo "crq: \"$CRQ_ID\""
  echo "messageId: \"$PR_NUMBER\"" # Using PR number as message ID for the PR description
  echo "timestamp: \"$PR_CREATED_AT\"" # Using createdAt as timestamp
  echo "title: \"$PR_TITLE\""
  echo "author: \"$PR_AUTHOR\""
  echo "url: \"$PR_URL\""
  echo "baseBranch: \"$PR_BASE_BRANCH\""
  echo "---"
  echo ""
  echo "$PR_BODY"
} > "$PR_DESCRIPTION_FILE"
echo "Created $PR_DESCRIPTION_FILE"

# --- Write comments ---
COMMENTS=$(echo "$PR_DATA" | jq -c '.comments[]')
COMMENT_COUNT=0

echo "$COMMENTS" | while read -r comment; do
  COMMENT_COUNT=$((COMMENT_COUNT + 1))
  COMMENT_AUTHOR=$(echo "$comment" | jq -r '.author.login')
  COMMENT_BODY=$(echo "$comment" | jq -r '.body')
  COMMENT_CREATED_AT=$(echo "$comment" | jq -r '.createdAt')

  COMMENT_FILE="$RESPONSES_DIR/$(printf "%03d" "$COMMENT_COUNT").md"
  {
    echo "---"
    echo "crq: \"$CRQ_ID\""
    echo "messageId: \"$(printf "%03d" "$COMMENT_COUNT")\"" # Sequential number as message ID for comments
    echo "timestamp: \"$COMMENT_CREATED_AT\"" # Using createdAt as timestamp
    echo "author: \"$COMMENT_AUTHOR\""
    echo "---"
    echo ""
    echo "$COMMENT_BODY"
  } > "$COMMENT_FILE"
  echo "Created $COMMENT_FILE"
done

echo "Finished processing PR #$PR_NUMBER."