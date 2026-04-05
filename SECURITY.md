# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are currently
supported:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of Linuxy seriously. If you believe you have found a
security vulnerability, please report it to us as described below.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email at: **swadhinbiswas.cse@gmail.com** (or
use GitHub's private vulnerability reporting feature)

You should receive a response within 48 hours. If for some reason you do not,
please follow up via email to ensure we received your original message.

### What to Include

Please include the following information in your report:

- **Type of issue**: e.g., buffer overflow, SQL injection, cross-site scripting,
  etc.
- **Full paths of source file(s) related to the issue**
- **Location of the affected source code (tag/branch/commit or direct URL)**
- **Step-by-step instructions to reproduce the issue**
- **Proof-of-concept or exploit code (if possible)**
- **Impact of the issue, including how an attacker might exploit it**

### Preferred Languages

We prefer all communications to be in English.

## Security Best Practices

### For Users

1. **Download from official sources only**
   - GitHub Releases: https://github.com/swadhinbiswas/linuxy/releases
   - Official package repositories (AUR, etc.)

2. **Verify checksums**
   - All releases include SHA256 checksums
   - Verify before installing

3. **Keep Linuxy updated**
   - Enable automatic updates when available
   - Check for updates regularly

4. **Use Firejail sandboxing**
   - Enable sandboxing for untrusted AppImages
   - Keep Firejail updated

### For Contributors

1. **Never commit sensitive data**
   - API keys
   - Passwords
   - Tokens
   - Personal information

2. **Follow secure coding practices**
   - Validate all inputs
   - Use parameterized queries
   - Implement proper error handling
   - Avoid hardcoded credentials

3. **Keep dependencies updated**
   - Regularly run `npm audit`
   - Regularly run `cargo audit`
   - Update vulnerable packages promptly

## Security Measures

### Automated Scanning

- **Dependency scanning**: Automated via GitHub Actions
- **Code scanning**: CodeQL analysis on all PRs
- **Secret scanning**: GitHub secret scanning enabled

### Build Security

- **Reproducible builds**: Working towards reproducible builds
- **Signed releases**: Release artifacts are signed
- **Minimal permissions**: CI/CD runs with minimal required permissions

### Code Review

- **Required reviews**: All PRs require at least one review
- **Security-focused review**: Security-sensitive changes get extra scrutiny
- **AI-assisted review**: CodeRabbit AI reviews all PRs

## Security Updates

Security updates will be released as patch versions (e.g., 0.1.1, 0.1.2).

For critical vulnerabilities, we may release an out-of-cycle security update.

## Disclosure Policy

1. Reporter submits vulnerability report
2. We acknowledge receipt within 48 hours
3. We investigate and confirm the vulnerability
4. We develop and test a fix
5. Fix is deployed to supported versions
6. Public disclosure after 30 days (or earlier by mutual agreement)

## Recognition

We believe in recognizing security researchers who help improve our security.
Researchers who report valid security vulnerabilities will be:

- Acknowledged in our security advisories (unless they wish to remain anonymous)
- Added to our security hall of fame
- Sent swag stickers (when available)

## Contact

- **Email**: swadhinbiswas.cse@gmail.com
- **GitHub**: Use private vulnerability reporting
- **PGP Key**: Available upon request

---

## Security Checklist for Releases

Before each release, ensure:

- [ ] All dependencies are up to date
- [ ] No known vulnerabilities in dependencies
- [ ] Code has been reviewed for security issues
- [ ] No sensitive data in code or config files
- [ ] Build process is secure
- [ ] Release notes mention any security fixes

---

**Last Updated**: March 2024
