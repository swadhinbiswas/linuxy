<div align="center">
  <img src="assets/linuxy-icon.png" alt="Linuxy Mascot" width="140" />

  <h1>Linuxy</h1>

  <p><strong>The Modern AppImage Manager for Linux</strong></p>

  <p>
    <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-22c55e?style=for-the-badge" alt="MIT License" /></a>
    <a href="https://github.com/swadhinbiswas/linuxy/releases"><img src="https://img.shields.io/github/v/release/swadhinbiswas/linuxy?style=for-the-badge&color=6366f1" alt="Release" /></a>
    <a href="https://tauri.app"><img src="https://img.shields.io/badge/Built_with-Tauri_v1-24C8DB?style=for-the-badge" alt="Tauri" /></a>
    <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/Rust-1.70+-e97b30?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" /></a>
    <a href="https://github.com/swadhinbiswas/linuxy/actions"><img src="https://img.shields.io/github/actions/workflow/status/swadhinbiswas/linuxy/ci.yml?style=for-the-badge&label=CI" alt="CI" /></a>
  </p>

  <p>
    Install, organize, sandbox, and update AppImages &mdash; all from one place.<br/>
    Drag-and-drop simplicity backed by Rust-powered performance.
  </p>
</div>

<br/>

<div align="center">
  <img src="docs/images/installed.png" alt="Linuxy Library View" width="80%" />
</div>

---

## 🎬 Demo Video

<div align="center">
  <a href="https://www.youtube.com/watch?v=1VJj5NP0NtE" target="_blank">
    <img src="https://img.youtube.com/vi/1VJj5NP0NtE/maxresdefault.jpg" alt="Linuxy Demo Video" width="80%" />
  </a>
  <p><strong>Watch the full demo on YouTube</strong></p>
</div>

---

## Features

|       | Feature                 | Description                                                                                                                                                                                                                           |
| ----- | ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1** | **One-Click Install**   | Drag & drop any `.AppImage` onto the window, or browse for it with the file picker. Linuxy validates the ELF header, extracts metadata and icons from the SquashFS, creates a `.desktop` entry, and copies everything to `~/.local/`. |
| **2** | **App Discovery**       | Browse the [AppImageHub](https://appimage.github.io) catalog or search GitHub's `appimage` topic -- then install directly in one click.                                                                                               |
| **3** | **Firejail Sandboxing** | Toggle per-app sandboxing with a single click. When enabled, the app runs under `firejail --appimage` with restricted filesystem and network access.                                                                                  |
| **4** | **Delta Updates**       | Checks for updates via the bundled `appimageupdatetool` sidecar. Only downloads binary diffs -- not the entire AppImage.                                                                                                              |
| **5** | **Downloads Watcher**   | Automatically detects new `.AppImage` files appearing in `~/Downloads` and prompts you to install them. A "Later" option suppresses repeated notifications.                                                                           |
| **6** | **Theming**             | Switch between Dark, Light, and System-follow themes. Preferences persist across sessions via `localStorage`.                                                                                                                         |
| **7** | **Storage Dashboard**   | Settings panel shows total disk usage and installed app count at a glance.                                                                                                                                                            |
| **8** | **CLI Mode**            | Headless installation and management from the terminal: `linuxy install <path>`, `linuxy list`, `linuxy help`.                                                                                                                        |
| **9** | **Desktop Integration** | Automatic `.desktop` file creation, icon extraction, and `update-desktop-database` for native menu integration.                                                                                                                       |

---

## Screenshots

<div align="center">

| Library                                                                   | Discover                                                          |
| ------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| <img src="docs/images/installed.png" alt="Installed Apps" width="100%" /> | <img src="docs/images/hub.png" alt="AppImage Hub" width="100%" /> |

| Settings                                                           | Auto-Discovery                                                               |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------- |
| <img src="docs/images/settings.png" alt="Settings" width="100%" /> | <img src="docs/images/autodiscover.png" alt="Auto Discovery" width="100%" /> |

</div>

---

## 📺 Video Walkthrough

<div align="center">
  <a href="https://www.youtube.com/watch?v=1VJj5NP0NtE" target="_blank">
    <img src="https://img.youtube.com/vi/1VJj5NP0NtE/hqdefault.jpg" alt="Linuxy Walkthrough" width="60%" />
  </a>
  <p>See Linuxy in action -- from installing your first AppImage to managing sandboxed applications.</p>
</div>

---

## Quick Start

### Prerequisites

| Requirement  | Minimum      | Notes                                     |
| ------------ | ------------ | ----------------------------------------- |
| **OS**       | Linux x86_64 | Tested on Ubuntu 22.04+, Fedora 38+, Arch |
| **Node.js**  | 18+          | For building from source                  |
| **Rust**     | 1.70+        | For building from source                  |
| **Firejail** | Any          | _Optional_ -- enables sandboxing          |

### Install from .deb (Debian / Ubuntu)

#### Option 1: Official APT Repository (Recommended)

Add the official repository to automatically receive updates via your package
manager:

```bash
# Add the repository signing key
curl -fsSL https://swadhinbiswas.github.io/linuxy/apt/linuxy.gpg.key | sudo gpg --dearmor -o /etc/apt/keyrings/linuxy.gpg

# Add the repository to sources
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/linuxy.gpg] https://swadhinbiswas.github.io/linuxy/apt /" | sudo tee /etc/apt/sources.list.d/linuxy.list

# Install Linuxy
sudo apt update
sudo apt install linuxy
```

#### Option 2: Manual Download

```bash
# Download the latest release
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_1.2.0_amd64.deb

# Install
sudo apt install ./linuxy_1.2.0_amd64.deb
```

### Install from .rpm (RHEL / Fedora / openSUSE)

```bash
# Download the latest release
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy-1.2.0-1.x86_64.rpm

# Install
sudo dnf install ./linuxy-1.2.0-1.x86_64.rpm       # Fedora / RHEL
sudo zypper install ./linuxy-1.2.0-1.x86_64.rpm     # openSUSE
```

### Install from .deb (Arch Linux)

> **Note**: The AUR package is recommended for Arch Linux. Use this only if you
> prefer the pre-built `.deb`.

```bash
# Install debtap (AUR helper)
yay -S debtap

# Update debtap database
sudo debtap -u

# Download the latest release
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_1.2.0_amd64.deb

# Convert DEB to Arch package
debtap linuxy_1.2.0_amd64.deb

# Install the converted package
sudo pacman -U linuxy-1.2.0-1-x86_64.pkg.tar.zst
```

### Install from AUR (Arch Linux)

```bash
yay -S linuxy
# Or with paru
paru -S linuxy
```

### Install from Snap

```bash
# From Snap Store
sudo snap install linuxy

# Or from local build
sudo snap install ./linuxy_*.snap --dangerous
```

### Install from Flatpak

```bash
# Add Flathub remote (if not already added)
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Build and install from source
flatpak-builder --force-clean --user --install-deps-from=flathub --install build-dir flatpak/com.linuxy.App.yaml

# Or run without installing
flatpak-builder --force-clean build-dir flatpak/com.linuxy.App.yaml
flatpak run com.linuxy.App
```

#### Option 2: Manual Download

```bash
# Download the latest release
wget https://github.com/swadhinbiswas/linuxy/releases/latest/download/linuxy_1.0.0_amd64.deb

# Install
sudo apt install ./linuxy_1.0.0_amd64.deb
```

### Build from Source

```bash
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy
npm install
npm run tauri build

# Binary output -> src-tauri/target/release/linuxy
# DEB package  -> src-tauri/target/release/bundle/deb/
```

### Run in Development

```bash
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy
npm install
npm run tauri dev
```

> [!TIP] On Wayland, Linuxy forces the X11/XWayland backend to work around a
> WebKitGTK compositing crash. No manual intervention required.

---

## Usage

### GUI

1. **Library** -- Your home screen. Drag an `.AppImage` here or click the
   drop-zone to browse.
2. **Discover** -- Search and install from AppImageHub or trending GitHub
   projects.
3. **Settings** -- Toggle themes, view storage stats, check Firejail /
   update-tool status, and open the AppImage directory.

Each installed app shows action buttons:

- **Launch** -- Run the AppImage (through Firejail if sandbox is toggled on).
- **Sandbox** -- Enable / disable Firejail isolation.
- **Update** -- Check for and apply delta updates.
- **Remove** -- Delete the AppImage, its `.desktop` entry, and cached icons.

### CLI

```bash
linuxy install ~/Downloads/App.AppImage   # Install an AppImage
linuxy list                               # List installed AppImages
linuxy help                               # Show available commands
```

You can also pass an AppImage path directly:

```bash
linuxy ~/Downloads/App.AppImage
```

---

## Architecture

```
linuxy/
├── src/                        # React + TypeScript frontend
│   ├── App.tsx                 # Root component -- views, modals, state
│   ├── components/
│   │   ├── Sidebar.tsx         # Navigation (Library / Discover / Settings)
│   │   ├── AppGrid.tsx         # Installed app cards with actions
│   │   ├── DropZone.tsx        # Drag-and-drop install area
│   │   └── Storefront.tsx      # AppImageHub + GitHub discovery
│   ├── styles/
│   │   └── main.css            # CSS custom-property theme system
│   └── main.tsx                # React entry point
│
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── main.rs             # Tauri builder, handler registration
│   │   ├── installer.rs        # SquashFS extraction, .desktop creation
│   │   ├── manager.rs          # CRUD ops, launch, sandbox toggle
│   │   ├── updater.rs          # Sidecar-based delta update commands
│   │   ├── watcher.rs          # ~/Downloads polling for new AppImages
│   │   ├── downloader.rs       # Streaming HTTP download + install
│   │   └── cli.rs              # Headless CLI subcommands
│   ├── icons/                  # App icons (all sizes + .ico/.icns)
│   ├── bin/                    # Bundled sidecar (appimageupdatetool)
│   └── Cargo.toml              # Rust dependencies
│
├── packaging/                  # Distribution configs
│   ├── deb/                    # Debian package metadata
│   ├── rpm/                    # RPM spec (planned)
│   ├── aur/                    # AUR PKGBUILD (planned)
│   └── appimage/               # Meta-AppImage build (planned)
│
├── docs/                       # Extended documentation
│   ├── images/                 # Application screenshots
│   ├── INSTALL.md
│   ├── USAGE.md
│   ├── MAINTENANCE.md
│   ├── CODE_SIGNING.md
│   └── PRODUCTION_READINESS.md
│
└── assets/                     # Branding
    └── linuxy-icon.png         # Mascot icon
```

---

## Configuration

### Storage Locations

| Purpose           | Path                           |
| ----------------- | ------------------------------ |
| AppImage binaries | `~/.local/appimages/`          |
| Desktop entries   | `~/.local/share/applications/` |
| Extracted icons   | `~/.local/share/icons/`        |

### Firejail Sandboxing

Install Firejail with your distro's package manager:

```bash
sudo apt install firejail        # Debian / Ubuntu
sudo dnf install firejail        # Fedora
sudo pacman -S firejail          # Arch
```

When sandbox is toggled on for an app, Linuxy rewrites its `.desktop` file to
prepend `firejail --appimage` to the `Exec=` line. Toggle it off to restore
direct execution.

### Delta Updates

Linuxy bundles `appimageupdatetool` as a Tauri sidecar. If the tool is present
in `src-tauri/bin/`, update checks and apply are available per-app. The Settings
panel shows the current detection status.

---

## Development

### System Dependencies

```bash
# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora / Nobara
sudo dnf install gtk3 gtk3-devel webkit2gtk4.1 webkit2gtk4.1-devel openssl-devel libappindicator-gtk3-devel

# Arch
sudo pacman -S webkit2gtk-4.1 openssl libappindicator-gtk3
```

### Scripts

| Command                | Description                           |
| ---------------------- | ------------------------------------- |
| `npm run dev`          | Start Vite dev server (frontend only) |
| `npm run tauri dev`    | Start full Tauri dev app              |
| `npm run build`        | Build frontend for production         |
| `npm run tauri build`  | Build the complete application        |
| `npm run lint`         | Run ESLint                            |
| `npm run lint:fix`     | Auto-fix lint issues                  |
| `npm run format`       | Format with Prettier                  |
| `npm run format:check` | Check formatting                      |
| `npm run typecheck`    | TypeScript type check                 |
| `npm run check`        | Run all quality checks                |

### Rust Quality

```bash
cd src-tauri && cargo fmt         # Format
cd src-tauri && cargo clippy      # Lint
cd src-tauri && cargo test        # Run unit tests
```

---

## Contributing

Contributions are welcome! Please see the [Contributing Guide](CONTRIBUTING.md)
for full details.

```bash
# 1. Fork & clone
git clone https://github.com/YOUR_USERNAME/linuxy.git && cd linuxy

# 2. Branch
git checkout -b feat/your-feature

# 3. Develop & commit (Conventional Commits)
git commit -m "feat: add your feature"

# 4. Push & open a PR
git push origin feat/your-feature
```

### Tech Stack

| Layer     | Technology                              |
| --------- | --------------------------------------- |
| Frontend  | React 18, TypeScript, Lucide Icons      |
| Backend   | Rust, Tauri v1, Tokio, Reqwest          |
| Build     | Vite 5, ESLint, Prettier                |
| Packaging | DEB, RPM, AUR (all platforms supported) |

---

## Troubleshooting

<details>
<summary><strong>AppImage won't launch</strong></summary>

Ensure the file is executable:

```bash
chmod +x ~/.local/appimages/your-app.AppImage
```

</details>

<details>
<summary><strong>Firejail not found</strong></summary>

Install Firejail with your package manager:

```bash
sudo apt install firejail          # Debian / Ubuntu
sudo dnf install firejail          # Fedora
sudo pacman -S firejail            # Arch
```

</details>

<details>
<summary><strong>Build fails with missing WebKit headers</strong></summary>

Install the WebKitGTK development package:

```bash
# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.1-dev
```

</details>

<details>
<summary><strong>Icons not showing after install</strong></summary>

Re-install the AppImage through Linuxy to re-extract icons, or manually copy an
icon to `~/.local/share/icons/`.

</details>

---

## Documentation

Full documentation is available in the [`docs/`](docs/) directory:

- [Installation Guide](docs/INSTALL.md)
- [Usage Guide](docs/USAGE.md)
- [Maintenance Guide](docs/MAINTENANCE.md)
- [Code Signing](docs/CODE_SIGNING.md)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

---

## License

This project is licensed under the [MIT License](LICENSE).

Copyright (c) 2025 [Swadhin Biswas](https://github.com/swadhinbiswas)

---

## Acknowledgments

- [Tauri](https://tauri.app) -- Native app framework
- [AppImage](https://appimage.org) -- Universal Linux package format
- [Firejail](https://firejail.wordpress.com) -- Linux sandbox
- [AppImageHub](https://appimage.github.io) -- App catalog feed
- [React](https://react.dev) -- UI library
- [Lucide](https://lucide.dev) -- Icon set

---

<div align="center">
  <sub>Made with care for the Linux community</sub>
  <br/>
  <a href="#linuxy">Back to top</a>
</div>
