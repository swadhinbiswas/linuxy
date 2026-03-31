# Linuxy Packaging

This directory contains packaging files for distributing Linuxy across different Linux distributions.

## Package Formats

### DEB (Debian/Ubuntu)
Built automatically by Tauri. Supports:
- Debian 10+
- Ubuntu 20.04+
- Linux Mint
- Pop!_OS
- Other Debian-based distributions

**Installation via Custom APT Repository:**
The easiest way to install and keep Linuxy updated is to add the official APT repository (hosted on GitHub Pages):

```bash
# Import the GPG key
curl -fsSL https://swadhinbiswas.github.io/linuxy/apt/linuxy.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/linuxy.gpg

# Add the repository
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/linuxy.gpg] https://swadhinbiswas.github.io/linuxy/apt /" | sudo tee /etc/apt/sources.list.d/linuxy.list

# Update & Install
sudo apt update
sudo apt install linuxy
```

**Installation via Manual Download:**
```bash
sudo apt install ./linuxy_*.deb
```

### RPM (RHEL/Fedora/OpenSUSE)
Converted from DEB using `fpm`. Supports:
- Fedora 35+
- RHEL 8+
- CentOS 8+
- OpenSUSE Tumbleweed
- Other RPM-based distributions

**Installation:**
```bash
# Fedora/RHEL
sudo dnf install ./linuxy-*.rpm

# OpenSUSE
sudo zypper install ./linuxy-*.rpm
```

### AUR (Arch Linux)
Community-maintained package for Arch-based distributions.

**Installation:**
```bash
# Using yay
yay -S linuxy

# Using paru
paru -S linuxy

# Manual build
git clone https://aur.archlinux.org/linuxy.git
cd linuxy
makepkg -si
```

## Building Packages Locally

### Prerequisites

```bash
# Debian/Ubuntu
sudo apt install -y libwebkit2gtk-4.0-dev build-essential libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  ruby ruby-dev

# Arch Linux
sudo pacman -S webkit2gtk base-devel openssl gtk3 \
  libappindicator-gtk3 librsvg libxdo ruby

# Fedora
sudo dnf install -y webkit2gtk3-devel openssl-devel gtk3-devel \
  libappindicator-gtk3-devel librsvg2-devel ruby ruby-dev
```

### Build Commands

```bash
# Install Node.js dependencies
npm install

# Build DEB package (and frontend)
npm run tauri build

# Build RPM package (requires fpm)
./scripts/build-rpm.sh
```

## AUR Package Maintenance

The AUR package files are located in `packaging/aur/`:

- `PKGBUILD` - Build script for AUR
- `.SRCINFO` - Package metadata
- `linuxy.install` - Install/upgrade hooks

To update the AUR package version:

1. Update `pkgver` in `PKGBUILD`
2. Update source URL with new version
3. Regenerate `.SRCINFO`:
   ```bash
   cd packaging/aur
   makepkg --printsrcinfo > .SRCINFO
   ```

4. Push to AUR:
   ```bash
   git clone ssh://aur@aur.archlinux.org/linuxy.git
   cp PKGBUILD .SRCINFO linuxy.install linuxy/
   cd linuxy
   git add .
   git commit -m "Update to version X.Y.Z"
   git push
   ```

## Release Checklist

- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] GitHub Actions will build and publish:
  - DEB package (GitHub Releases)
  - RPM package (GitHub Releases)
  - AUR package (AUR repository)
