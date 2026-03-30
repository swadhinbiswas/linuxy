# 📚 Linuxy Documentation Index

Complete documentation for the Linuxy AppImage Manager project.

**Repository**: https://github.com/swadhinbiswas/linuxy

---

## 🚀 Quick Start

### For Users
1. **[Installation Guide](docs/INSTALL.md)** - How to install Linuxy
2. **[Usage Guide](docs/USAGE.md)** - How to use Linuxy
3. **[Application Info](APPLICATION_INFO.md)** - Window controls, icons, signatures

### For Contributors
1. **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
2. **[Code of Conduct](CODE_OF_CONDUCT.md)** - Community guidelines
3. **[Maintenance Guide](docs/MAINTENANCE.md)** - Repository maintenance

---

## 📖 Documentation Files

### Core Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [README.md](README.md) | Project overview and quick start | Everyone |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines | Contributors |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Community code of conduct | Everyone |
| [LICENSE](LICENSE) | MIT License | Everyone |
| [CHANGELOG.md](CHANGELOG.md) | Version history | Everyone |

### User Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [docs/INSTALL.md](docs/INSTALL.md) | Installation instructions | Users |
| [docs/USAGE.md](docs/USAGE.md) | Usage guide and tips | Users |
| [APPLICATION_INFO.md](APPLICATION_INFO.md) | App info (icons, signatures, etc.) | Users |
| [website.html](website.html) | Landing page | Visitors |

### Developer Documentation

| File | Purpose | Audience |
|------|---------|----------|
| [docs/MAINTENANCE.md](docs/MAINTENANCE.md) | Repository maintenance | Maintainers |
| [docs/CODE_SIGNING.md](docs/CODE_SIGNING.md) | Code signing guide | Maintainers |
| [SECURITY.md](SECURITY.md) | Security policy | Security researchers |
| [.github/BRANCH_PROTECTION.md](.github/BRANCH_PROTECTION.md) | Branch protection rules | Maintainers |

---

## 🔧 Configuration Files

### GitHub Configuration

| File | Purpose |
|------|---------|
| [.github/workflows/ci.yml](.github/workflows/ci.yml) | CI/CD pipeline |
| [.github/workflows/release.yml](.github/workflows/release.yml) | Release automation |
| [.github/workflows/codeql.yml](.github/workflows/codeql.yml) | Security analysis |
| [.github/workflows/stale.yml](.github/workflows/stale.yml) | Close stale issues |
| [.github/workflows/welcome.yml](.github/workflows/welcome.yml) | Welcome new contributors |
| [.github/workflows/labeler.yml](.github/workflows/labeler.yml) | Auto-label PRs |
| [.github/dependabot.yml](.github/dependabot.yml) | Dependency updates |
| [.github/CODEOWNERS](.github/CODEOWNERS) | Code ownership |
| [.github/labeler.yml](.github/labeler.yml) | Label configuration |
| [.github/release-drafter.yml](.github/release-drafter.yml) | Release notes draft |

### Code Quality

| File | Purpose |
|------|---------|
| [.eslintrc.yml](.eslintrc.yml) | ESLint configuration |
| [.prettierrc.yml](.prettierrc.yml) | Prettier formatting |
| [.editorconfig](.editorconfig) | Editor settings |
| [src-tauri/rustfmt.toml](src-tauri/rustfmt.toml) | Rust formatting |
| [src-tauri/clippy.toml](src-tauri/clippy.toml) | Rust linting |
| [.coderabbit.yml](.coderabbit.yml) | CodeRabbit AI review |

### IDE/Editor

| File | Purpose |
|------|---------|
| [.vscode/settings.json](.vscode/settings.json) | VS Code workspace settings |
| [.vscode/extensions.json](.vscode/extensions.json) | Recommended extensions |

---

## 📦 Package Information

### Project Details

```
Name: Linuxy
Version: 0.1.0
License: MIT (Open Source)
Maintainer: Linuxy Contributors
Developer: @swadhinbiswas
Identifier: com.linuxy.dev
Repository: https://github.com/swadhinbiswas/linuxy
```

### Build Commands

```bash
# Development
npm run dev              # Start Vite dev server
npm run tauri:dev        # Start Tauri dev app

# Code Quality
npm run lint             # Run ESLint
npm run format           # Format with Prettier
npm run typecheck        # TypeScript check
npm run check            # Run all checks

# Building
npm run build            # Build frontend
npm run tauri:build      # Build full application

# Rust
cd src-tauri && cargo clippy   # Rust linting
cd src-tauri && cargo fmt      # Rust formatting
cd src-tauri && cargo audit    # Security audit
```

---

## 🌐 Online Resources

### GitHub
- **Repository**: https://github.com/swadhinbiswas/linuxy
- **Issues**: https://github.com/swadhinbiswas/linuxy/issues
- **Pull Requests**: https://github.com/swadhinbiswas/linuxy/pulls
- **Releases**: https://github.com/swadhinbiswas/linuxy/releases
- **Discussions**: https://github.com/swadhinbiswas/linuxy/discussions
- **Actions (CI/CD)**: https://github.com/swadhinbiswas/linuxy/actions

### External
- **Tauri Framework**: https://tauri.app
- **AppImage**: https://appimage.org
- **Firejail**: https://firejail.wordpress.com

---

## 📋 Documentation Status

### Completed ✅
- [x] README.md - Project overview
- [x] CONTRIBUTING.md - Contribution guide
- [x] CODE_OF_CONDUCT.md - Community guidelines
- [x] SECURITY.md - Security policy
- [x] docs/INSTALL.md - Installation guide
- [x] docs/USAGE.md - Usage guide
- [x] docs/MAINTENANCE.md - Maintenance guide
- [x] docs/CODE_SIGNING.md - Code signing guide
- [x] APPLICATION_INFO.md - Application information
- [x] CHANGELOG.md - Version history
- [x] website.html - Landing page

### GitHub Configuration ✅
- [x] CI/CD workflows
- [x] CodeRabbit AI review
- [x] Dependabot auto-updates
- [x] Issue and PR templates
- [x] Branch protection rules
- [x] Code owners
- [x] Release drafter
- [x] Stale issue handler
- [x] Welcome bot

### Code Quality ✅
- [x] ESLint configuration
- [x] Prettier formatting
- [x] Rust Clippy linting
- [x] Rust formatting
- [x] EditorConfig
- [x] VS Code settings

---

## 🤝 Contributing to Documentation

If you find errors or want to improve documentation:

1. **Create an issue**: https://github.com/swadhinbiswas/linuxy/issues/new?template=docs_issue.yml
2. **Submit a PR**: https://github.com/swadhinbiswas/linuxy/pulls
3. **Start a discussion**: https://github.com/swadhinbiswas/linuxy/discussions

### Documentation Guidelines

- Use clear, concise language
- Include examples where helpful
- Keep formatting consistent
- Update related docs when making changes
- Test command examples before committing

---

## 📞 Support

- **Report a bug**: https://github.com/swadhinbiswas/linuxy/issues/new?template=bug_report.yml
- **Request a feature**: https://github.com/swadhinbiswas/linuxy/issues/new?template=feature_request.yml
- **Ask a question**: https://github.com/swadhinbiswas/linuxy/discussions

---

**Last Updated**: March 2024  
**License**: MIT (Open Source)  
**Repository**: https://github.com/swadhinbiswas/linuxy
