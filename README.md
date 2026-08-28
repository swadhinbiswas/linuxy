<p align="center">
  <img src="src-tauri/icons/icon.png" alt="Linuxy" width="100" />
</p>

<h1 align="center">Linuxy</h1>

<p align="center">
  <strong>AppImage manager that doesn't get in your way.</strong><br>
  Install, browse, sandbox, and update AppImages from one window.
</p>

<p align="center">
  <a href="https://github.com/swadhinbiswas/linuxy/releases"><img src="https://img.shields.io/github/v/release/swadhinbiswas/linuxy?style=for-the-badge&color=6366f1" alt="Latest Release"></a>
  <a href="https://github.com/swadhinbiswas/linuxy/actions"><img src="https://img.shields.io/github/actions/workflow/status/swadhinbiswas/linuxy/ci.yml?style=for-the-badge&label=CI" alt="CI Status"></a>
  <a href="https://github.com/swadhinbiswas/linuxy/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-22c55e?style=for-the-badge" alt="License"></a>
  <a href="https://aur.archlinux.org/packages/linuxy"><img src="https://img.shields.io/aur/version/linuxy?style=for-the-badge&color=1793d1&label=AUR" alt="AUR Version"></a>
</p>

---

## Screenshots

<div align="center">
  <table>
    <tr>
      <td align="center">
        <img src="docs/images/installed.png" alt="Library" width="380" /><br>
        <sub><b>Library</b> — installed apps at a glance</sub>
      </td>
      <td align="center">
        <img src="docs/images/hub.png" alt="Discover" width="380" /><br>
        <sub><b>Discover</b> — browse AppImageHub + GitHub</sub>
      </td>
    </tr>
    <tr>
      <td align="center">
        <img src="docs/images/settings.png" alt="Settings" width="380" /><br>
        <sub><b>Settings</b> — themes, storage, sandboxing</sub>
      </td>
      <td align="center">
        <img src="docs/images/autodiscover.png" alt="Auto-discover" width="380" /><br>
        <sub><b>Auto-detect</b> — watches your Downloads folder</sub>
      </td>
    </tr>
  </table>
</div>

## Features

| Feature | Description |
|---------|-------------|
| Drag-and-drop install | Drop an AppImage — Linuxy extracts metadata, icons, creates `.desktop` entry, and copies it to `~/.local/appimages/`. Done. |
| App discovery | Browse the [AppImageHub](https://appimage.github.io) catalog or search GitHub repos tagged `appimage`. Install in one click. |
| Firejail sandboxing | Toggle per-app. Enabled apps run under `firejail --appimage` with restricted filesystem and network access. |
| Delta updates | Bundles `appimageupdatetool`. Only fetches the binary diff, not the whole AppImage. |
| Downloads watcher | Notices new AppImages in `~/Downloads` and prompts to install. "Later" snoozes until the next one appears. |
| CLI mode | `linuxy install`, `linuxy list`, `linuxy launch`. Works headless. |
| Dark/Light/System themes | Persists across sessions. |
| Storage dashboard | Disk usage and app count at a glance. |

## Install

### One-liner (any distro)

```bash
curl -fsSL https://raw.githubusercontent.com/swadhinbiswas/linuxy/main/install.sh | bash
```

Automatically detects your package manager (APT, DNF, Zypper) and downloads the right package. Falls back to the universal AppImage.

### Manual

<details>
<summary><b>Debian / Ubuntu / Mint / Pop!_OS</b></summary>

```bash
sudo apt install ./linuxy_*_amd64.deb
```
</details>

<details>
<summary><b>Fedora / RHEL</b></summary>

```bash
sudo dnf install ./linuxy-*.rpm
```
</details>

<details>
<summary><b>openSUSE</b></summary>

```bash
sudo zypper install ./linuxy-*.rpm
```
</details>

<details>
<summary><b>Arch Linux (AUR)</b></summary>

```bash
yay -S linuxy
```
</details>

<details>
<summary><b>Any distro (AppImage)</b></summary>

```bash
chmod +x Linuxy_*.AppImage
./Linuxy_*.AppImage
```
</details>

<details>
<summary><b>Build from source</b></summary>

```bash
git clone https://github.com/swadhinbiswas/linuxy.git
cd linuxy
bun install
bun run tauri build
```
</details>

## Usage

### GUI

| Tab | What it does |
|-----|-------------|
| **Library** | Your installed apps. Launch, toggle sandbox, check updates, remove. Drag new AppImages here. |
| **Discover** | Browse AppImageHub or search GitHub. One-click install. |
| **Settings** | Theme toggle, storage stats, Firejail / updater status. |

### CLI

```bash
linuxy install ~/Downloads/App.AppImage   # install from path
linuxy install https://example.org/app    # install from URL
linuxy list                               # list installed apps
linuxy launch AppName                     # launch by name
linuxy remove AppName                     # remove by name
linuxy --help                             # full command list
```

## Configuration

| Path | Contents |
|------|----------|
| `~/.local/appimages/` | AppImage binaries |
| `~/.local/share/applications/` | `.desktop` entries |
| `~/.local/share/icons/hicolor/*/apps/` | Extracted icons |

**Firejail**: install with your distro's package manager, then toggle per-app from the GUI. When enabled, Linuxy rewrites the `.desktop` `Exec=` line to prepend `firejail --appimage`.

**Wayland**: Linuxy runs via XWayland by default (`WEBKIT_DISABLE_COMPOSITING_MODE=1`). No configuration needed.

## Development

### Prerequisites

```bash
# Ubuntu / Debian
sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libxdo-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel gtk3-devel \
  libappindicator-gtk3-devel librsvg2-devel libxdo-devel
```

### Run

```bash
bun install
bun run tauri dev
```

### Checks

```bash
bun run typecheck
bun run lint
cd src-tauri && cargo clippy && cargo test
```

## Contributing

```bash
git clone https://github.com/YOUR_USERNAME/linuxy.git
cd linuxy
git checkout -b feat/your-thing
# make changes
git commit -m "feat: what you did"
git push origin feat/your-thing
```

Open a pull request. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

[MIT](LICENSE)

---

<p align="center">
  <sub>Made with Rust + React for the Linux desktop.</sub>
</p>
