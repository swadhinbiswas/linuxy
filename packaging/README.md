# Linuxy Packaging Guide

This directory contains packaging files for distributing Linuxy across
**all major Linux distributions**, plus Windows and macOS.

## Package Formats

| Format | Distro Family | File | Build Tool |
|--------|--------------|------|------------|
| **DEB** | Debian, Ubuntu, Mint, Pop!_OS, Kali, Deepin, Parrot | `debian/` (root) | `dpkg-buildpackage` |
| **RPM** | Fedora, RHEL, CentOS, OpenSUSE | `packaging/rpm/linuxy.spec` | `rpmbuild` or fpm |
| **AppImage** | All Linux (universal) | via Tauri | `npx tauri build` |
| **Flatpak** | All Linux (sandboxed) | `flathub/com.linuxy.App/` | `flatpak-builder` |
| **AUR** | Arch Linux, Manjaro, EndeavourOS | `packaging/aur/` | `makepkg` |
| **APK** | Alpine Linux | `packaging/alpine/APKBUILD` | `abuild` |
| **XBPS** | Void Linux | `packaging/void/template` | `xbps-src` |
| **eopkg** | Solus | `packaging/solus/` | `eopkg` |
| **ebuild** | Gentoo, Funtoo | `packaging/gentoo/` | `emerge` |
| **Nix** | NixOS, Nix (any Linux) | `flake.nix` (root) | `nix build` |
| **DMG** | macOS | via Tauri | `npx tauri build` |
| **NSIS/MSI** | Windows | via Tauri | `npx tauri build` |

## Quick Install (End User)

```bash
# Any Linux distro - generic AppImage
curl -sSL https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy.AppImage
chmod +x linuxy.AppImage
./linuxy.AppImage
```

### Debian / Ubuntu / Mint
```bash
curl -fsSL https://swadhinbiswas.github.io/linuxy/apt/linuxy.gpg.key \
  | sudo gpg --dearmor -o /etc/apt/keyrings/linuxy.gpg
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/linuxy.gpg] https://swadhinbiswas.github.io/linuxy/apt /" \
  | sudo tee /etc/apt/sources.list.d/linuxy.list
sudo apt update && sudo apt install linuxy
```

### Fedora / RHEL
```bash
sudo dnf install https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy.rpm
```

### Arch Linux (AUR)
```bash
yay -S linuxy
# or
paru -S linuxy
```

### NixOS
```nix
# flake.nix
{
  inputs.linuxy.url = "github:swadhinbiswas/linuxy";
  outputs = { self, nixpkgs, linuxy }: {
    nixosConfigurations.myMachine = nixpkgs.lib.nixosSystem {
      modules = [ { environment.systemPackages = [ linuxy.packages.x86_64-linux.default ]; } ];
    };
  };
}
```

### Alpine Linux
```bash
# Manual build
abuild -r
apk add linuxy --allow-untrusted
```

### Void Linux
```bash
./xbps-src pkg linuxy
sudo xbps-install -R hostdir/binpkgs linuxy
```

### Solus
```bash
sudo eopkg install linuxy
```

### Gentoo
```bash
sudo emerge linuxy
```

### Flatpak (any Linux)
```bash
flatpak install flathub com.linuxy.App
# or build locally:
flatpak-builder --user --install build-dir flatpak/com.linuxy.App.yaml
```

### macOS
```bash
# Download .dmg from releases, or
brew install --cask linuxy
```

### Windows
```bash
# Download .msi or .exe from releases, or
winget install linuxy
```

## Building Packages Locally

### Prerequisites

#### Debian/Ubuntu
```bash
sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  libxdo-dev ruby ruby-dev
```

#### Fedora
```bash
sudo dnf install -y \
  webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg2-devel openssl-devel libxdo-devel ruby ruby-devel
```

#### Arch
```bash
sudo pacman -S --needed \
  webkit2gtk gtk3 libappindicator-gtk3 librsvg libxdo \
  base-devel nodejs npm rust
```

#### Alpine
```bash
sudo apk add \
  webkit2gtk-4.1-dev gtk+3.0-dev libappindicator-gtk3-dev \
  librsvg-dev openssl-dev libxdo-dev \
  cargo rust nodejs npm alpine-sdk
```

#### NixOS / Nix
```bash
nix develop
# or
nix-shell
```

#### Void Linux
```bash
sudo xbps-install -S \
  webkit2gtk4.1-devel gtk+3-devel libappindicator-gtk3-devel \
  librsvg-devel openssl-devel libxdo-devel \
  cargo nodejs npm base-devel
```

#### Solus
```bash
sudo eopkg install \
  webkit2gtk-4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg-devel openssl-devel libxdo-devel \
  cargo nodejs npm
```

#### Gentoo
```bash
sudo emerge \
  net-libs/webkit-gtk:4.1 x11-libs/gtk+:3 dev-libs/libappindicator:3 \
  gnome-base/librsvg dev-libs/openssl x11-libs/libxdo \
  dev-lang/rust dev-lang/nodejs
```

### Build Commands

```bash
# 1. Build DEB (Tauri default)
npm install
npm run build
npx tauri build

# 2. Build RPM (from DEB using fpm)
./scripts/build-rpm.sh

# 3. Convert DEB to RPM manually
gem install fpm
fpm -s deb -t rpm \
  --depends firejail --depends gtk3 --depends webkit2gtk4.1 \
  src-tauri/target/release/bundle/deb/linuxy_*.deb

# 4. Build Flatpak
./scripts/build-flatpak.sh

# 5. Build AppImage directly
npx tauri build --bundles appimage

# 6. Build for Windows/macOS (cross-compile)
npx tauri build --target x86_64-pc-windows-msvc   # Windows
npx tauri build --target x86_64-apple-darwin       # macOS
```

## Packaging Maintenance

### AUR (Arch Linux)

```bash
# Update version in PKGBUILD + .SRCINFO
./scripts/update-aur.sh 2.0.0

# Regenerate .SRCINFO after editing PKGBUILD
cd packaging/aur
makepkg --printsrcinfo > .SRCINFO

# Push to AUR
git clone ssh://aur@aur.archlinux.org/linuxy.git
cp PKGBUILD .SRCINFO linuxy.install linuxy/
cd linuxy
git add .
git commit -m "Update to v2.0.0"
git push
```

### Gentoo

```bash
# Copy ebuild to Gentoo overlay
cp packaging/gentoo/linuxy-2.0.0.ebuild /var/db/repos/local/app-admin/linuxy/
cd /var/db/repos/local/app-admin/linuxy
ebuild linuxy-2.0.0.ebuild manifest
sudo emerge linuxy
```

### Nix

```bash
# Build from flake
nix build github:swadhinbiswas/linuxy
# Run directly
nix run github:swadhinbiswas/linuxy
# Enter dev shell
nix develop github:swadhinbiswas/linuxy
```

### Alpine

```bash
# Build package
abuild-keygen -a -n
abuild -r
# Install
apk add linuxy --allow-untrusted
```

### Void Linux

```bash
# Using xbps-src
cd void-packages
./xbps-src pkg linuxy
sudo xbps-install -R hostdir/binpkgs linuxy
```

### Debian APT Repository

```bash
# Setup package signing
gpg --full-generate-key
# Export GPG key
gpg --armor --export key-id > linuxy.gpg.key
# Create repo structure
mkdir -p repo/pool/main/
cp linuxy_*.deb repo/pool/main/
cd repo
dpkg-scanpackages pool/ > dists/stable/main/binary-amd64/Packages
gzip -k dists/stable/main/binary-amd64/Packages

# Create Release file
apt-ftparchive release dists/stable/ > dists/stable/Release
gpg --armor --detach-sign --output dists/stable/Release.gpg dists/stable/Release
```

## Release Checklist

- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update version in `debian/changelog`
- [ ] Update version in `packaging/aur/PKGBUILD`
- [ ] Update version in `packaging/rpm/linuxy.spec`
- [ ] Update version in `packaging/alpine/APKBUILD`
- [ ] Update version in `packaging/gentoo/linuxy-*.ebuild`
- [ ] Update version in `flake.nix`
- [ ] Update `CHANGELOG.md`
- [ ] Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push tag: `git push origin vX.Y.Z`
- [ ] GitHub Actions will:
  - Build DEB, RPM, AppImage for Linux
  - Build NSIS + MSI for Windows
  - Build DMG for macOS
  - Publish all to GitHub Releases
- [ ] After CI: update AUR, submit to Flathub, etc.
