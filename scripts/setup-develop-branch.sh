#!/bin/bash

# Branch Protection Setup Script for Linuxy - Develop Branch
# Usage: ./scripts/setup-develop-branch.sh [GITHUB_TOKEN]

set -e

REPO="swadhinbiswas/linuxy"
BRANCH="develop"
GITHUB_TOKEN="${1:-$GITHUB_TOKEN}"

if [ -z "$GITHUB_TOKEN" ]; then
    echo "Error: GitHub token required"
    echo "Usage: $0 <GITHUB_TOKEN>"
    echo "Or set GITHUB_TOKEN environment variable"
    exit 1
fi

echo "Setting up branch protection for $REPO/$BRANCH..."

# Check if branch exists
BRANCH_EXISTS=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
  "$API/repos/$REPO/branches/$BRANCH" | jq -r '.name // empty')

if [ -z "$BRANCH_EXISTS" ]; then
    echo "Branch 'develop' doesn't exist. Creating it from main..."
    # Create develop branch from main
    MAIN_SHA=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
      "$API/repos/$REPO/branches/main" | jq -r '.commit.sha')
    
    curl -s -X POST "$API/repos/$REPO/git/refs/refs/heads/$BRANCH" \
      -H "Authorization: token $GITHUB_TOKEN" \
      -H "Accept: application/vnd.github+json" \
      -d "{\"sha\": \"$MAIN_SHA\", \"ref\": \"refs/heads/$BRANCH\"}"
    
    echo "✓ Created develop branch"
fi

# API Base URL
API="https://api.github.com"
PROTECTION_URL="$API/repos/$REPO/branches/$BRANCH/protection"

echo "✓ Configuring protection for develop..."

# Basic protection for develop
curl -s -X PUT "$PROTECTION_URL" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d '{
    "required_status_checks": {
      "strict": false,
      "contexts": [
        "Frontend Lint & Build",
        "Rust Check & Test"
      ]
    },
    "enforce_admins": true,
    "required_pull_request_reviews": {
      "dismiss_stale_reviews": false,
      "required_approving_review_count": 1
    },
    "restrictions": null,
    "allow_force_pushes": false,
    "allow_deletions": false,
    "require_conversation_resolution": false
  }'

echo ""
echo "✅ Branch protection configured for develop!"
