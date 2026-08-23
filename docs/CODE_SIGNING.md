# Code Signing Guide for Linuxy

This guide explains how to sign Linuxy packages for secure distribution.

## Project Info

- **Name**: Linuxy
- **License**: MIT (Open Source)
- **Maintainer**: Swadhin Biswas <swadhinbiswas.cse@gmail.com>
- **Developer**: @swadhinbiswas
- **Repository**: https://github.com/swadhinbiswas/linuxy

## Why Sign Packages?

Package signing provides:

- **Authentication**: Verifies the package comes from you
- **Integrity**: Ensures the package hasn't been tampered with
- **Trust**: Users can verify the publisher before installing

## Quick Start

### 1. Build the Package

```bash
bun run tauri build
```

### 2. Sign the Package

```bash
./scripts/sign-package.sh
```

This script will:

- Check for GPG
- Create a signing key if needed
- Sign the DEB package

### 3. Verify the Signature

```bash
dpkg-sig --verify src-tauri/target/release/bundle/deb/linuxy_0.1.0_amd64.deb
```

Expected output:

```
GOODSIG
```

## Manual Signing

### Generate GPG Key

```bash
gpg --gen-key
```

Provide:

- **Name**: Swadhin Biswas
- **Email**: swadhinbiswas.cse@gmail.com
- **Passphrase**: (choose a strong one)

### Export Public Key

```bash
gpg --armor --export swadhinbiswas.cse@gmail.com > linuxy-public.key
```

Users can import this to verify your packages.

### Sign the Package

```bash
# Find your key ID
gpg --list-secret-keys

# Sign
dpkg-sig --sign builder -k YOUR_KEY_ID linuxy_*.deb
```

## For Users to Verify

### Import Developer's Public Key

```bash
gpg --import linuxy-public.key
```

### Verify Package

```bash
dpkg-sig --verify linuxy_*.deb
```

## DEB Package Metadata

The DEB package includes:

| Field          | Value                                        |
| -------------- | -------------------------------------------- |
| **Maintainer** | Swadhin Biswas <swadhinbiswas.cse@gmail.com> |
| **Developer**  | @swadhinbiswas                               |
| **Identifier** | com.linuxy.dev                               |
| **License**    | MIT (Open Source)                            |

## Distribution

### For AUR (Arch Linux)

Create PKGBUILD with proper maintainer info:

```bash
maintainer='Swadhin Biswas <swadhinbiswas.cse@gmail.com>'
```

### For PPA (Ubuntu)

1. Sign with your GPG key
2. Upload to Launchpad
3. Launchpad will re-sign with Ubuntu keys

## Best Practices

1. **Keep private key secure** - Never commit to git
2. **Use strong passphrase** - Protect your signing key
3. **Backup your key** - Export and store safely
4. **Rotate keys periodically** - Security best practice
5. **Publish public key** - Make it easy for users to verify

## Troubleshooting

### "dpkg-sig: command not found"

```bash
sudo apt install dpkg-sig
```

### "No secret key"

Generate a new key:

```bash
gpg --gen-key
```

### "Bad signature"

The package was modified after signing. Rebuild and re-sign.

## Contact

For questions about package signing:

- **Email**: swadhinbiswas.cse@gmail.com
- **GitHub**: https://github.com/swadhinbiswas/linuxy

---

**Last Updated**: March 2024 **Project**: Linuxy - Open Source (MIT License)
**Repository**: https://github.com/swadhinbiswas/linuxy
