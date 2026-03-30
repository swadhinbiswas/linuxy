# Branch Protection Configuration

This document describes the recommended branch protection rules for the Linuxy
repository.

## Protected Branches

### `main` (Primary Branch)

The `main` branch should have the following protections:

#### Required Settings

- [x] **Require a pull request before merging**
  - [x] Require approvals: **1**
  - [x] Dismiss stale pull request approvals when new commits are pushed
  - [x] Require review from Code Owners: **Yes**

- [x] **Require status checks to pass before merging**
  - [x] Require branches to be up to date before merging
  - Required status checks:
    - [x] `Frontend Lint & Build`
    - [x] `Rust Check & Test`
    - [x] `Security Audit`
    - [x] `Verify CSP Configuration`
    - [x] `Integration Tests`

- [x] **Require conversation resolution before merging**
  - All comments must be resolved before merging

- [x] **Include administrators**
  - Apply all rules to repository administrators as well

#### Recommended Settings

- [x] **Do not allow bypassing the above settings**
- [x] **Restrict who can push to matching branches**
  - Allow only users with write permission
- [x] **Allow force pushes**: No
- [x] **Allow deletions**: No
- [x] **Lock branch**: No (allow PRs)

### `develop` (Development Branch)

The `develop` branch should have similar protections:

#### Required Settings

- [x] **Require a pull request before merging**
  - [x] Require approvals: **1**

- [x] **Require status checks to pass before merging**
  - Required status checks:
    - [x] `Frontend Lint & Build`
    - [x] `Rust Check & Test`

#### Recommended Settings

- [x] **Include administrators**: Yes
- [x] **Allow force pushes**: No (or limited to maintainers)
- [x] **Allow deletions**: No

## Branch Naming Conventions

### Feature Branches

```
feature/<short-description>
feat/<short-description>
```

Examples:

- `feature/firejail-sandbox-improvements`
- `feat/dark-mode-toggle`

### Bug Fix Branches

```
fix/<issue-number>-<short-description>
bugfix/<issue-number>-<short-description>
```

Examples:

- `fix/123-appimage-launch-crash`
- `bugfix/456-icon-not-loading`

### Documentation Branches

```
docs/<short-description>
```

Examples:

- `docs/update-installation-guide`
- `docs/add-api-reference`

### Release Branches

```
release/<version>
```

Examples:

- `release/v0.1.0`
- `release/v0.2.0-beta`

### Hotfix Branches

```
hotfix/<issue-number>-<short-description>
```

Examples:

- `hotfix/789-critical-security-fix`

## Code Owner Reviews

### Required Reviewers

At least one approval is required from:

1. **Code Owners** (for their respective areas)
2. **Repository Maintainers**

### Code Owners File

See `.github/CODEOWNERS` for the list of code owners.

## Status Checks

### Required Checks

All PRs must pass:

1. **CI/CD Pipeline**
   - Frontend lint and build
   - Rust check and test
   - Security audit
   - CSP configuration verification
   - Integration tests

2. **Code Quality**
   - No Clippy warnings
   - TypeScript type checking passes
   - Code formatting is correct

### Optional Checks

- Code coverage (when implemented)
- Performance benchmarks

## Merge Strategies

### Allowed Merge Methods

- [x] **Create a merge commit**
- [x] **Squash and merge**
- [ ] **Rebase and merge** (use with caution)

### Recommended Strategy

**Squash and merge** is recommended for feature branches to keep history clean.

**Merge commit** is acceptable for release branches.

## Manual Intervention

### Emergency Bypass

In case of emergency (e.g., critical security fix), repository administrators
can:

1. Create a hotfix branch
2. Submit PR with `[EMERGENCY]` prefix
3. Request expedited review
4. Merge after at least one approval

All emergency merges must be documented in the next team meeting.

## Enforcement

### Automated Enforcement

- GitHub Actions enforce status checks
- Branch protection rules prevent direct pushes
- CODEOWNERS ensures proper review

### Manual Enforcement

Repository maintainers should:

- Regularly review branch protection settings
- Ensure all contributors understand the process
- Address any attempts to bypass protections

---

## Setup Instructions

### For Repository Administrators

1. Go to **Settings** → **Branches**
2. Click **Add branch protection rule**
3. Enter branch name pattern: `main`
4. Select the required settings above
5. Click **Create**

### For Contributors

No setup required. GitHub will enforce these rules automatically when you:

- Try to push directly to protected branches
- Create a pull request
- Merge a pull request

---

**Last Updated**: March 2024 **Maintained by**: Repository Administrators
