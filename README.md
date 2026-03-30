# 🐧 Linuxy

**The Ultimate AppImage Manager for Linux**

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/release/swadhinbiswas/linuxy.svg)](https://github.com/swadhinbiswas/linuxy/releases)
[![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri-24C8DB)](https://tauri.app)
[![Rust](https://img.shields.io/badge/rust-v1.70+-orange)](https://www.rust-lang.org)

Linuxy is a modern, feature-rich AppImage manager that simplifies how you
install, organize, and run AppImage applications on Linux. With one-click
installation, Firejail sandboxing, and optional delta updates, Linuxy brings
simplicity and security to managing portable Linux applications.

**📚 Complete Documentation**: [DOCS_INDEX.md](DOCS_INDEX.md)

![Linuxy Screenshot](.github/screenshot.png)

## ✨ Features

- **📦 One-Click Installation** - Drag and drop AppImage files to install them
  instantly
- **🛡️ Firejail Sandboxing** - Toggle secure sandboxing per application with a
  single click
- **🔄 Optional Delta Updates** - Update checks for supported AppImages when
  `appimageupdatetool` is installed
- **🎨 Beautiful Interface** - Modern UI with dark and light theme support
- **📊 Storage Management** - Monitor disk usage across all installed AppImages
- **🔍 App Discovery** - Browse and discover new AppImages directly from the app
- **🖥️ Desktop Integration** - Automatic .desktop file creation for menu
  integration
- **🎯 Icon Management** - Automatic icon extraction and caching

## 📸 Screenshots

### Library View

![Library View](.github/library.png)

### Settings View

![Settings View](.github/settings.png)

### Dark/Light Themes

![Themes](.github/themes.png)

## 🚀 Quick Start

### Prerequisites

- **Linux** (x86_64 architecture)
- **Node.js** 18+ (for building from source)
- **Rust** 1.70+ (for building from source)
- **Firejail** (optional, for sandboxing features)

### Installation Methods

#### Option 1: DEB Package (Debian/Ubuntu)

```bash
# Download the latest .deb package from releases
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_1.0.0_amd64.deb

# Install the package
sudo apt install ./linuxy_1.0.0_amd64.deb
```

#### Option 2: AUR Package (Arch Linux)

```bash
# Using yay
yay -S linuxy

# Using paru
paru -S linuxy
```

#### Option 3: Build from Source

```bash
# Clone the repository
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy

# Install dependencies
npm install

# Build the application
npm run tauri build

# The built application will be in src-tauri/target/release/
```

#### Option 4: Run in Development Mode

```bash
# Clone and install dependencies
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy
npm install

# Run in development mode
npm run dev
```

## 📖 Usage Guide

### Installing AppImages

1. **Drag and Drop**: Simply drag an `.AppImage` file onto the Linuxy window
2. **File Browser**: Click the "Browse" button to select an AppImage file
3. **Discovery**: Use the built-in app discovery to find and install popular
   AppImages

### Managing Installed Apps

- **Launch**: Click the "Launch" button to run an installed AppImage
- **Sandbox**: Click the shield icon to toggle Firejail sandboxing
- **Update**: Click the refresh icon to check for updates when
  `appimageupdatetool` is available
- **Remove**: Click the trash icon to uninstall an AppImage

### Firejail Sandboxing

Firejail provides an additional security layer by restricting what AppImages can
access:

```bash
# Install Firejail (if not already installed)
sudo apt install firejail        # Debian/Ubuntu
sudo dnf install firejail        # Fedora
sudo pacman -S firejail          # Arch Linux
```

When sandboxing is enabled for an app, it runs with restricted access to:

- Home directory (configurable)
- System files
- Network (optional)
- Other processes

### Keyboard Shortcuts

| Shortcut | Action              |
| -------- | ------------------- |
| `Ctrl+O` | Open AppImage file  |
| `Ctrl+R` | Refresh app list    |
| `Ctrl+,` | Open settings       |
| `Delete` | Remove selected app |

## 🏗️ Project Structure

```
linuxy/
├── src/                    # React/TypeScript frontend
│   ├── components/         # UI components
│   │   ├── AppGrid.tsx     # App display grid
│   │   ├── Sidebar.tsx     # Navigation sidebar
│   │   ├── DropZone.tsx    # Drag-drop zone
│   │   └── Storefront.tsx  # App discovery
│   ├── styles/             # CSS stylesheets
│   ├── App.tsx             # Main application component
│   └── main.tsx            # Entry point
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Application entry
│   │   ├── manager.rs      # App management logic
│   │   ├── installer.rs    # Installation logic
│   │   ├── updater.rs      # Update checking
│   │   ├── watcher.rs      # File system watcher
│   │   └── downloader.rs   # Download utilities
│   ├── icons/              # Application icons
│   └── Cargo.toml          # Rust dependencies
├── packaging/              # Package configurations
│   ├── deb/                # Debian package config
│   ├── aur/                # AUR package config
│   └── appimage/           # AppImage build config
└── index.html              # Landing page
```

## ⚙️ Configuration

### Storage Locations

| Type          | Location                       |
| ------------- | ------------------------------ |
| AppImages     | `~/.local/appimages/`          |
| Desktop Files | `~/.local/share/applications/` |
| Icons         | `~/.local/share/icons/`        |

### Environment Variables

| Variable       | Description         | Default |
| -------------- | ------------------- | ------- |
| `TAURI_DEBUG`  | Enable debug mode   | `false` |
| `VITE_API_URL` | Custom API endpoint | -       |

## 🔧 Development

### Prerequisites Setup

```bash
# Install Node.js (using nvm recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Debian/Ubuntu)
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### Available Scripts

```bash
# Development
npm run dev          # Start Vite dev server
npm run tauri dev    # Start Tauri dev app

# Building
npm run build        # Build frontend
npm run tauri build  # Build full application

# Other
npm run preview      # Preview production build
```

### Code Style

```bash
# Frontend linting
npm run lint

# Rust formatting
cd src-tauri && cargo fmt

# Rust linting
cd src-tauri && cargo clippy
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md)
for details.

### Quick Start for Contributors

```bash
# Fork the repository
git clone https://github.com/YOUR_USERNAME/linuxy.git
cd linuxy

# Create a branch
git checkout -b feature/your-feature

# Make your changes and commit
git commit -m "feat: add your feature"

# Push and create PR
git push origin feature/your-feature
```

### Development Guidelines

1. **Frontend**: TypeScript + React with functional components
2. **Backend**: Rust with async/await patterns
3. **Styling**: CSS custom properties for theming
4. **Commits**: Follow
   [Conventional Commits](https://www.conventionalcommits.org/)

## 🐛 Troubleshooting

### Common Issues

#### "Firejail not found" error

```bash
# Install Firejail
sudo apt install firejail
```

#### AppImage won't launch

```bash
# Make sure the AppImage is executable
chmod +x ~/.local/appimages/your-app.AppImage
```

#### Missing icons

```bash
# Reinstall the app to regenerate icons
# Or manually extract icons to ~/.local/share/icons/
```

#### Build fails with missing dependencies

```bash
# Ubuntu/Debian (24.04+)
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev

# Ubuntu/Debian (20.04-22.04)
sudo apt install libwebkit2gtk-4.0-dev libssl-dev libayatana-appindicator3-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel libappindicator-gtk3-devel

# Arch Linux
sudo pacman -S webkit2gtk-4.1 openssl libappindicator-gtk3
```

## 📄 License

Linuxy is released under the [MIT License](LICENSE).

```
Copyright (c) 2024 Linuxy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```

## 🙏 Acknowledgments

- [Tauri](https://tauri.app) - The framework powering Linuxy
- [Firejail](https://firejail.wordpress.com/) - Sandbox security feature
- [AppImage](https://appimage.org/) - Universal Linux package format
- [React](https://react.dev/) - UI library
- [Lucide Icons](https://lucide.dev/) - Beautiful icons

## 📬 Contact

- **GitHub Issues**:
  [Report a bug or request a feature](https://github.com/swadhinbiswas/linuxy/issues)
- **Discussions**:
  [Community discussions](https://github.com/swadhinbiswas/linuxy/discussions)
- **Twitter**: [@linuxyapp](https://twitter.com/linuxyapp)

---

<div align="center">
  <p>Made with ❤️ and 🦀 for the Linux community</p>
  <p>
    <a href="#-linuxy">↑ Back to top ↑</a>
  </p>
</div>
