#!/bin/bash
# Publish the AUR package to aur.archlinux.org with retry-on-failure logic.
# Handles transient AUR outages/maintenance by retrying the git push with backoff.
#
# Required environment variables:
#   AUR_SSH_PRIVATE_KEY  - SSH private key with access to the AUR package
#   AUR_VERSION          - version to publish (also used in the commit message)
# Optional:
#   AUR_COMMIT_USERNAME  - git committer name  (default: "Linuxy Bot")
#   AUR_COMMIT_EMAIL     - git committer email (default: "bot@localhost")
#   AUR_MAX_ATTEMPTS     - max push attempts   (default: 6)

set -euo pipefail

AUR_PKG="${AUR_PKG:-linuxy}"
AUR_URL="ssh://aur@aur.archlinux.org/${AUR_PKG}.git"
SSH_PRIVATE_KEY="${AUR_SSH_PRIVATE_KEY:?AUR_SSH_PRIVATE_KEY is required}"
AUR_VERSION="${AUR_VERSION:?AUR_VERSION is required}"
COMMIT_USERNAME="${AUR_COMMIT_USERNAME:-Linuxy Bot}"
COMMIT_EMAIL="${AUR_COMMIT_EMAIL:-bot@localhost}"
MAX_ATTEMPTS="${AUR_MAX_ATTEMPTS:-6}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AUR_DIR="$(cd "$SCRIPT_DIR/../packaging/aur" && pwd)"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

echo "Initializing SSH directory"
mkdir -p ~/.ssh
chmod 700 ~/.ssh

echo "Adding aur.archlinux.org to known hosts"
ssh-keyscan -t rsa,ed25519 aur.archlinux.org >>~/.ssh/known_hosts 2>/dev/null || true

echo "Importing private key"
(umask 077; printf '%s\n' "$SSH_PRIVATE_KEY" >~/.ssh/aur_key)
chmod 600 ~/.ssh/aur_key

export GIT_SSH_COMMAND="ssh -i $HOME/.ssh/aur_key -o IdentitiesOnly=yes"

echo "Cloning AUR package into $WORK_DIR"
git clone --quiet "$AUR_URL" "$WORK_DIR/repo"
cd "$WORK_DIR/repo"
git config user.name "$COMMIT_USERNAME"
git config user.email "$COMMIT_EMAIL"

echo "Copying files into the repository"
cp "$AUR_DIR/PKGBUILD" "$AUR_DIR/.SRCINFO" "$AUR_DIR/linuxy.install" ./

git add -A
if git diff --cached --quiet; then
    echo "No changes to push; AUR package is already up to date."
    exit 0
fi

echo "Committing files to the repository"
git commit -q -m "chore: update to version ${AUR_VERSION}"

echo "Publishing the repository"
attempt=0
while ! git push --quiet origin master; do
    attempt=$((attempt + 1))
    if [ "$attempt" -ge "$MAX_ATTEMPTS" ]; then
        echo "Failed to push to AUR after ${attempt} attempts (AUR may be down)." >&2
        exit 1
    fi
    delay=$((attempt * 30))
    echo "Push failed (attempt ${attempt}/${MAX_ATTEMPTS}); retrying in ${delay}s..." >&2
    sleep "$delay"
    git fetch --quiet origin master && git rebase --quiet origin/master || true
done

echo "Successfully published ${AUR_PKG} ${AUR_VERSION} to the AUR."
