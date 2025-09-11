#!/bin/bash
# Script to extract unique actors (users) from GitHub issues and comments

# Ensure gh CLI is installed and authenticated
if ! command -v gh &> /dev/null
then
    echo "GitHub CLI (gh) could not be found. Please install it and authenticate."
    exit 1
fi

# Fetch all issues (open and closed) with their comments in JSON format
# We request 'number' and 'comments' fields. 'comments' will contain 'author.login'
ISSUES_JSON=$(gh issue list --state all --json number,comments,author,assignees,timelineItems)

# Initialize an empty array for actors
ACTORS=()

# Use jq to parse the JSON and extract unique author logins from issues and comments
# .[] iterates over each issue
# .author.login gets the issue author
# .comments[].author.login gets each comment author
# .assignees[].login gets each assignee
# .timelineItems[].actor.login gets actors from timeline events (e.g., reviewers, closers)
# select(. != null) filters out null values (e.g., for deleted users or system events without a direct actor)
# unique sorts and removes duplicates
ACTORS_ARRAY=$(echo "$ISSUES_JSON" | jq -r '[ .[] | .author.login, .[] | .comments[].author.login, .[] | .assignees[].login, .[] | .timelineItems[].actor.login ] | flatten | unique | .[] | select(. != null)))

# Populate the bash array
while IFS= read -r actor; do
    ACTORS+=("$actor")
done <<< "$ACTORS_ARRAY"

# Print the unique actors
echo "Unique Actors in Issues and Comments:"
for actor in "${ACTORS[@]}"; do
    echo "- $actor"
done

# Optionally, save to a file
# printf "%s\n" "${ACTORS[@]}" > actors_list.txt
