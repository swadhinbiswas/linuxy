#!/bin/bash

# Branch Protection Setup Script for Linuxy
# Usage: ./scripts/setup-branch-protection.sh [GITHUB_TOKEN]

set -e

REPO="swadhinbiswas/linuxy"
BRANCH="main"
GITHUB_TOKEN="${1:-$GITHUB_TOKEN}"

if [ -z "$GITHUB_TOKEN" ]; then
    echo "Error: GitHub token required"
    echo "Usage: $0 <GITHUB_TOKEN>"
    echo "Or set GITHUB_TOKEN environment variable"
    exit 1
fi

echo "Setting up branch protection for $REPO/$BRANCH..."

# API Base URL
API="https://api.github.com"

# 1. Get branch protection endpoint
PROTECTION_URL="$API/repos/$REPO/branches/$BRANCH/protection"

# 2. Require pull request + required reviews
echo "✓ Configuring PR requirements..."
curl -s -X PUT "$PROTECTION_URL/required_pull_request_reviews" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d '{
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true,
    "required_approving_review_count": 1
  }'

# 3. Require status checks
echo "✓ Configuring status checks..."
curl -s -X PUT "$PROTECTION_URL/required_status_checks" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d '{
    "strict": true,
    "contexts": [
      "Frontend Lint & Build",
      "Rust Check & Test",
      "Security Audit",
      "Verify CSP Configuration",
      "Integration Tests"
    ]
  }'

# 4. Require conversation resolution
echo "✓ Configuring conversation resolution..."
curl -s -X PUT "$PROTECTION_URL" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d '{
    "required_status_checks": null,
    "enforce_admins": true,
    "required_pull_request_reviews": null,
    "restrictions": null,
    "allow_force_pushes": false,
    "allow_deletions": false,
    "require_conversation_resolution": true
  }'

# 5. Include administrators
echo "✓ Setting admin enforcement..."
curl -s -X PUT "$PROTECTION_URL/enforce_admins" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": true
  }'

# 6. Configure branch restrictions (optional - requires team/org)
echo "✓ Configuring push restrictions..."
curl -s -X DELETE "$PROTECTION_URL/restrictions" \
  -H "Authorization: token $GITHUB_TOKEN" \
  -H "Accept: application/vnd.github+json" || true

echo ""
echo "✅ Branch protection configured for main!"
echo ""
echo "Required Status Checks (to be auto-detected after first run):"
echo "  - Frontend Lint & Build"
echo "  - Rust Check & Test"
echo "  - Security Audit"
echo "  - Verify CSP Configuration"
echo "  - Integration Tests"
echo ""
echo "Note: Status checks will be auto-detected after CI runs once."
