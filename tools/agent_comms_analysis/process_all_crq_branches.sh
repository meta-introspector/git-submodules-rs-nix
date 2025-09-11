#!/usr/bin/env bash

set -e

echo "Starting to process all CRQ branches..."

CURRENT_BRANCH=$(git branch --show-current)

# Get list of open PRs with their associated headRefName (branch name)
# and number.
PR_LIST=$(gh pr list --json number,headRefName,title) # Also get title to extract CRQ_ID early

if [ -z "$PR_LIST" ]; then
  echo "No open pull requests found."
  exit 0
fi

# Create a central directory for analysis data
ANALYSIS_DATA_DIR="analysis_data/comms/git"
mkdir -p "$ANALYSIS_DATA_DIR"

# Parse PR list using jq
echo "$PR_LIST" | jq -c '.[]' | while read -r pr_entry; do
  PR_NUMBER=$(echo "$pr_entry" | jq -r '.number')
  BRANCH_NAME=$(echo "$pr_entry" | jq -r '.headRefName')
  PR_TITLE=$(echo "$pr_entry" | jq -r '.title') # Extract title here

  # Extract CRQ ID from PR title (assuming format "CRQ-XXX: ...")
  CRQ_ID=$(echo "$PR_TITLE" | grep -oP 'CRQ-\d+' || echo "PR-$PR_NUMBER") # Extract CRQ-XXX, fallback if not found

  echo "--------------------------------------------------"
  echo "Processing PR #$PR_NUMBER (CRQ: $CRQ_ID) on branch '$BRANCH_NAME'"
  echo "--------------------------------------------------"

  # Stash any local changes before checking out a new branch
  # This handles cases where there are uncommitted changes that prevent checkout
  git stash push --include-untracked --message "Stashing changes before processing CRQ branch $BRANCH_NAME" || true

  # Checkout the branch
  if git checkout "$BRANCH_NAME"; then
    echo "Successfully checked out branch '$BRANCH_NAME'."
  else
    echo "Failed to checkout branch '$BRANCH_NAME'. Skipping this PR."
    git stash pop || true # Try to pop stash if checkout failed
    continue
  fi

  # Run the mirroring script
  if /data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/tools/agent_comms_analysis/mirror_pr_to_fs.sh "$PR_NUMBER"; then
    echo "Successfully mirrored PR #$PR_NUMBER to file system."
  else
    echo "Failed to mirror PR #$PR_NUMBER. Please check mirror_pr_to_fs.sh errors."
    # Decide whether to continue or exit on failure
    # For now, continue to next PR
  fi

  # Add and commit only the comms/ directory
  if git add comms/ && git commit -m "feat: Mirror PR #$PR_NUMBER and comments to file system"; then
    echo "Successfully committed changes for PR #$PR_NUMBER."
  else
    echo "Failed to commit changes for PR #$PR_NUMBER. Check for uncommitted changes or conflicts."
  fi

  # Copy the generated CRQ directory to the central analysis data directory
  if [ -d "comms/git/$CRQ_ID" ]; then
    echo "Copying comms/git/$CRQ_ID to $ANALYSIS_DATA_DIR/$CRQ_ID"
    cp -r "comms/git/$CRQ_ID" "$ANALYSIS_DATA_DIR/"
  else
    echo "Warning: comms/git/$CRQ_ID not found after mirroring. Skipping copy to analysis_data."
  fi

  # Clean up comms/ directory in the current branch before switching back
  # This prevents untracked files from interfering with subsequent checkouts
  git clean -fdx comms/ || true

  # Try to pop stash if there was one
  git stash pop || true

done

# Switch back to the original branch
if git checkout "$CURRENT_BRANCH"; then
  echo "Successfully switched back to original branch '$CURRENT_BRANCH'."
else
  echo "Failed to switch back to original branch '$CURRENT_BRANCH'. Please checkout manually."
fi

# Final stash pop in case something went wrong and stash was not popped
git stash pop || true

echo "--------------------------------------------------"
echo "All CRQ branches processed."
echo "Script finished."
