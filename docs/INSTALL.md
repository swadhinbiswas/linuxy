# Installation Guide

This guide covers all available methods to install Linuxy on your Linux system.

## Table of Contents

- [System Requirements](#system-requirements)
- [Installation Methods](#installation-methods)
  - [DEB Package (Debian/Ubuntu)](#deb-package-debianubuntu)
  - [AUR Package (Arch Linux)](#aur-package-arch-linux)
  - [DEB Package (Arch Linux)](#deb-package-arch-linux)
  - [RPM Package (Fedora/openSUSE)](#rpm-package-fedoraopensuse)
  - [AppImage (Universal)](#appimage-universal)
  - [Build from Source](#build-from-source)
- [Post-Installation](#post-installation)
- [Uninstallation](#uninstallation)

---

## System Requirements

### Minimum Requirements

- **OS**: Linux (x86_64)
- **RAM**: 512 MB
- **Disk Space**: 100 MB
- **Display**: 1280x720 resolution

### Recommended Requirements

- **OS**: Modern Linux distribution (Ubuntu 20.04+, Fedora 36+, Arch Linux)
- **RAM**: 1 GB
- **Disk Space**: 500 MB (for installed AppImages)
- **Display**: 1920x1080 resolution

### Dependencies

Linuxy requires the following system libraries:

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libxdo-dev

# Fedora / Nobara
sudo dnf install gtk3 \
    gtk3-devel \
    webkit2gtk4.1 \
    webkit2gtk4.1-devel \
    openssl-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libxdo-devel

# Arch Linux
sudo pacman -S webkit2gtk \
    openssl \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libxdo
```

---

## Installation Methods

### DEB Package (Debian/Ubuntu)

#### Step 1: Download

Download the latest `.deb` package from the
[releases page](https://github.com/swadhinbiswas/linuxy/releases):

```bash
# Using wget
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_0.1.0_amd64.deb

# Or using curl
curl -LO https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_0.1.0_amd64.deb
```

#### Step 2: Install

```bash
# Using apt (recommended)
sudo apt install ./linuxy_0.1.0_amd64.deb

# Or using dpkg
sudo dpkg -i linuxy_0.1.0_amd64.deb
sudo apt install -f  # Fix any missing dependencies
```

#### Step 3: Verify

```bash
# Launch from terminal
linuxy

# Or find it in your application menu
```

---

### AUR Package (Arch Linux)

#### Using yay (recommended)

```bash
yay -S linuxy
```

#### Using paru

```bash
paru -S linuxy
```

#### Manual AUR Installation

```bash
# Clone the AUR package
git clone https://aur.archlinux.org/linuxy.git
cd linuxy

# Build and install
makepkg -si
```

### DEB Package (Arch Linux)

> **Note**: The AUR package is the recommended installation method for Arch
> Linux. Use this method only if you prefer installing the pre-built `.deb`
> package directly.

#### Step 1: Install debtap

```bash
# Using yay
yay -S debtap

# Or using paru
paru -S debtap
```

#### Step 2: Update debtap Database

```bash
sudo debtap -u
```

#### Step 3: Download the DEB Package

```bash
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_1.2.0_amd64.deb
```

#### Step 4: Convert and Install

```bash
# Convert DEB to Arch package
debtap linuxy_1.2.0_amd64.deb

# Install the converted package
sudo pacman -U linuxy-1.2.0-1-x86_64.pkg.tar.zst
```

---

### RPM Package (Fedora/openSUSE)

#### Step 1: Download

```bash
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy-0.1.0.x86_64.rpm
```

#### Step 2: Install

```bash
# Fedora/RHEL
sudo dnf install ./linuxy-0.1.0.x86_64.rpm

# openSUSE
sudo zypper install linuxy-0.1.0.x86_64.rpm
```

---

### AppImage (Universal)

#### Step 1: Download

```bash
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy-0.1.0-x86_64.AppImage
```

#### Step 2: Make Executable

```bash
chmod +x linuxy-0.1.0-x86_64.AppImage
```

#### Step 3: Run

```bash
# Run directly
./linuxy-0.1.0-x86_64.AppImage

# Or integrate with system (optional)
./linuxy-0.1.0-x86_64.AppImage --appimage-install
```

---

### Build from Source

#### Prerequisites

```bash
# Install Bun (v1.1 or later)
curl -fsSL https://bun.sh/install | bash

# Install Rust (v1.70 or later)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build dependencies (Debian/Ubuntu)
sudo apt install build-essential \
    libwebkit2gtk-4.1-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libxdo-dev
```

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy

# Install Bun dependencies
bun install

# Build the application
bun run tauri build
```

#### Locate Built Application

After building, find your application in:

```
src-tauri/target/release/linuxy
```

#### Optional: Install System-Wide

```bash
# Copy binary
sudo cp src-tauri/target/release/linuxy /usr/local/bin/

# Copy desktop file
sudo cp src-tauri/target/release/bundle/deb/linuxy.desktop /usr/share/applications/

# Copy icon
sudo cp src-tauri/icons/128x128.png /usr/share/icons/hicolor/128x128/apps/linuxy.png
```

---

## Post-Installation

### Optional: Install Firejail for Sandboxing

```bash
# Debian/Ubuntu
sudo apt install firejail

# Fedora
sudo dnf install firejail

# Arch Linux
sudo pacman -S firejail
```

### First Launch

1. Open your application menu
2. Search for "Linuxy"
3. Click to launch

Or from terminal:

```bash
linuxy
```

### Configure Desktop Integration

Linuxy automatically creates desktop entries for installed AppImages. If you
want to manually refresh the desktop database:

```bash
update-desktop-database ~/.local/share/applications/
```

---

## Uninstallation

### DEB Package

```bash
sudo apt remove linuxy
# Or completely remove with config
sudo apt purge linuxy
```

### AUR Package

```bash
yay -R linuxy
# Or with paru
paru -R linuxy
```

### DEB Package (converted via debtap)

```bash
sudo pacman -R linuxy
```

### RPM Package

```bash
sudo dnf remove linuxy
# Or
sudo rpm -e linuxy
```

### AppImage

```bash
# Simply delete the AppImage file
rm linuxy-0.1.0-x86_64.AppImage
```

### Source Installation

```bash
# Remove binary
sudo rm /usr/local/bin/linuxy

# Remove desktop files
sudo rm /usr/share/applications/linuxy.desktop
sudo rm ~/.local/share/applications/linuxy.desktop

# Remove icons
sudo rm /usr/share/icons/hicolor/*/apps/linuxy.png
```

### Remove User Data (Optional)

```bash
# Remove installed AppImages
rm -rf ~/.local/appimages/

# Remove desktop entries
rm ~/.local/share/applications/*.desktop

# Remove icons
rm ~/.local/share/icons/*_icon.*

# Remove configuration
rm -rf ~/.config/linuxy/
```

---

## Troubleshooting

### Installation Fails with Dependency Errors

```bash
# Debian/Ubuntu - fix broken dependencies
sudo apt --fix-broken install

# Check for missing libraries
ldd /usr/bin/linuxy | grep "not found"
```

### Application Won't Start

```bash
# Check logs
journalctl -xe | grep linuxy

# Run from terminal to see errors
linuxy --verbose
```

### Missing Icons in Application Menu

```bash
# Refresh icon cache
gtk-update-icon-cache -f /usr/share/icons/hicolor/

# Rebuild desktop database
update-desktop-database
```

---

## Getting Help

- **Documentation**: [GitHub Wiki](https://github.com/swadhinbiswas/linuxy/wiki)
- **Issues**: [Report a problem](https://github.com/swadhinbiswas/linuxy/issues)
- **Discussions**:
  [Community help](https://github.com/swadhinbiswas/linuxy/discussions)

---

<div align="center">
  <p><strong>Having trouble? Open an issue on GitHub!</strong></p>
</div>
