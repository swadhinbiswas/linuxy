# Repository Maintenance Guide

This guide provides instructions for maintaining the Linuxy repository and ensuring code quality.

## Table of Contents

- [Daily Tasks](#daily-tasks)
- [Weekly Tasks](#weekly-tasks)
- [Monthly Tasks](#monthly-tasks)
- [Release Checklist](#release-checklist)
- [Code Quality Tools](#code-quality-tools)
- [Troubleshooting](#troubleshooting)

---

## Daily Tasks

### Review Pull Requests

1. Check for new PRs: https://github.com/swadhinbiswas/linuxy/pulls
2. Review CodeRabbit AI comments
3. Ensure all status checks pass
4. Request changes if needed
5. Approve and merge when ready

### Monitor Issues

1. Check for new issues: https://github.com/swadhinbiswas/linuxy/issues
2. Label appropriately (bug, enhancement, etc.)
3. Assign to milestone if applicable
4. Respond to reporters

### Check CI/CD Status

1. Review Actions: https://github.com/swadhinbiswas/linuxy/actions
2. Fix any failing workflows
3. Restart failed jobs if transient

---

## Weekly Tasks

### Dependency Updates

```bash
# Check for npm updates
npm outdated

# Update dependencies
npm update

# Check for Rust updates
cd src-tauri && cargo outdated

# Update Rust dependencies
cd src-tauri && cargo update
```

### Security Audits

```bash
# NPM audit
npm audit
npm audit fix

# Cargo audit
cd src-tauri && cargo audit
```

### Code Quality Review

```bash
# Run all checks
npm run check

# Run Clippy
cd src-tauri && cargo clippy -- -D warnings

# Check formatting
npm run format:check
cd src-tauri && cargo fmt --all -- --check
```

---

## Monthly Tasks

### Review Branch Protection

1. Go to Settings → Branches
2. Verify protection rules are active
3. Update required status checks if needed

### Clean Up

```bash
# Remove stale branches
git fetch --prune

# Clean old tags (if needed)
git tag -d <old-tag>
```

### Documentation Review

- [ ] README.md is up to date
- [ ] Installation guide works
- [ ] Usage guide reflects current features
- [ ] CONTRIBUTING.md is accurate
- [ ] API documentation (if applicable)

### Performance Review

1. Check app startup time
2. Monitor memory usage
3. Review bundle size
4. Optimize if needed

---

## Release Checklist

### Pre-Release

- [ ] All tests passing
- [ ] No critical open issues
- [ ] CHANGELOG.md updated
- [ ] Version bumped in:
  - [ ] package.json
  - [ ] src-tauri/tauri.conf.json
  - [ ] Cargo.toml
- [ ] Documentation reviewed
- [ ] Screenshots updated (if UI changed)

### Release Process

```bash
# 1. Create release branch
git checkout -b release/v0.1.0

# 2. Update version files
# Edit package.json, tauri.conf.json, Cargo.toml

# 3. Commit changes
git commit -m "chore: bump version to 0.1.0"

# 4. Push and create PR
git push origin release/v0.1.0

# 5. After PR merge, create tag
git checkout main
git pull
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# 6. GitHub Actions will build and create release
```

### Post-Release

- [ ] Verify release assets uploaded
- [ ] Test downloaded binaries
- [ ] Update website/download page
- [ ] Announce on social media
- [ ] Update AUR package (if applicable)

---

## Code Quality Tools

### Frontend

| Tool | Command | Purpose |
|------|---------|---------|
| TypeScript | `npm run typecheck` | Type checking |
| ESLint | `npm run lint` | Code quality |
| Prettier | `npm run format` | Formatting |

### Backend

| Tool | Command | Purpose |
|------|---------|---------|
| Cargo Check | `cargo check` | Compilation check |
| Clippy | `cargo clippy` | Linting |
| Rustfmt | `cargo fmt` | Formatting |
| Cargo Audit | `cargo audit` | Security |

### Automated

| Tool | Configuration | Purpose |
|------|---------------|---------|
| GitHub Actions | `.github/workflows/` | CI/CD |
| CodeRabbit | `.coderabbit.yml` | AI code review |
| Dependabot | `.github/dependabot.yml` | Dependency updates |
| Labeler | `.github/labeler.yml` | Auto-labeling |

---

## Troubleshooting

### CI/CD Failing

**Problem**: Status checks failing on PR

**Solutions**:
1. Check workflow logs in Actions tab
2. Run locally: `npm run check`
3. Fix reported issues
4. Push new commit to re-run

### Dependency Conflicts

**Problem**: npm install fails

**Solutions**:
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install
```

**Problem**: Cargo build fails

**Solutions**:
```bash
# Clean and rebuild
cd src-tauri
cargo clean
cargo build
```

### Formatting Issues

**Problem**: Format check fails in CI

**Solutions**:
```bash
# Auto-fix formatting
npm run format
cd src-tauri && cargo fmt --all
```

### Clippy Warnings

**Problem**: Clippy warnings in CI

**Solutions**:
```bash
# See all warnings
cd src-tauri && cargo clippy -- -W clippy::all

# Fix auto-fixable issues
cd src-tauri && cargo clippy --fix
```

---

## Best Practices

### Commit Messages

- Use conventional commits format
- Keep subject line under 72 characters
- Include issue number when applicable

Example:
```
feat: add automatic update checking

- Implement update detection for zsync2 AppImages
- Add update button to app cards
- Show changelog when update available

Fixes #123
```

### Code Review

- Be constructive and specific
- Explain why, not just what
- Suggest improvements, not just fixes
- Acknowledge good code

### Issue Management

- Respond within 48 hours
- Label appropriately
- Close when resolved
- Link to PRs when fixed

---

## Automation Setup

### Dependabot Configuration

Create `.github/dependabot.yml`:

```yaml
version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    labels:
      - "dependencies"
      - "npm"
    open-pull-requests-limit: 10

  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
    labels:
      - "dependencies"
      - "rust"
    open-pull-requests-limit: 10
```

### Stale Issue Management

Create `.github/workflows/stale.yml`:

```yaml
name: Close Stale Issues

on:
  schedule:
    - cron: '0 0 * * *'

jobs:
  stale:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/stale@v9
        with:
          stale-issue-message: 'This issue has been automatically marked as stale because it has not had recent activity.'
          stale-pr-message: 'This PR has been automatically marked as stale because it has not had recent activity.'
          close-issue-message: 'This issue has been automatically closed due to inactivity.'
          days-before-stale: 30
          days-before-close: 7
          exempt-issue-labels: 'bug,enhancement,help-wanted'
```

---

## Contact

For questions about repository maintenance:

- **Email**: maintainers@linuxy.app
- **Discussions**: https://github.com/swadhinbiswas/linuxy/discussions

---

**Last Updated**: March 2024
**Maintained by**: Repository Maintainers
