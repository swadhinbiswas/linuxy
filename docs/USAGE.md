# Usage Guide

A comprehensive guide to using Linuxy for managing your AppImages.

## Table of Contents

- [Getting Started](#getting-started)
- [User Interface](#user-interface)
- [Installing AppImages](#installing-appimages)
- [Managing Apps](#managing-apps)
- [Sandboxing with Firejail](#sandboxing-with-firejail)
- [Updating Apps](#updating-apps)
- [Settings & Configuration](#settings--configuration)
- [Keyboard Shortcuts](#keyboard-shortcuts)
- [Tips & Tricks](#tips--tricks)

---

## Getting Started

### First Launch

When you first launch Linuxy, you'll see an empty library. This is normal - you haven't installed any AppImages yet!

```bash
# Launch from terminal
linuxy

# Or find it in your application menu under "Utilities"
```

### Quick Tour

1. **Library** - Your installed AppImages (default view)
2. **Discover** - Browse and find new AppImages
3. **Settings** - Configure Linuxy preferences

---

## User Interface

### Library View

The Library is where you manage your installed AppImages.

```
┌─────────────────────────────────────────────────┐
│  Library                                        │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌─────────────┐  ┌─────────────┐              │
│  │   📝 Icon   │  │   🎨 Icon   │              │
│  │   App Name  │  │   App Name  │              │
│  │  120 MB     │  │  85 MB      │              │
│  │ [▶][🔄][🗑] │  │ [▶][🔄][🗑] │              │
│  └─────────────┘  └─────────────┘              │
│                                                 │
└─────────────────────────────────────────────────┘
```

Each app card shows:
- **Icon** - Application icon (auto-extracted)
- **Name** - Application name
- **Size & Date** - File size and installation date
- **Actions** - Launch, Update, Remove buttons
- **Shield Icon** - Sandbox status (top-right corner)

### Sidebar Navigation

```
┌──────────────────┐
│  🐧 Linuxy       │
├──────────────────┤
│  📚 Library      │  ← Your apps
│  🏪 Discover     │  ← Find new apps
│  ⚙️  Settings     │  ← Configuration
├──────────────────┤
│  🔄 Refresh      │  ← Reload library
│  ❓ Help         │  ← Documentation
└──────────────────┘
```

---

## Installing AppImages

### Method 1: Drag and Drop

1. Open Linuxy
2. Drag an `.AppImage` file from your file manager
3. Drop it onto the Linuxy window
4. Installation begins automatically

### Method 2: Browse Button

1. Click the "Browse" or "+" button in the Library view
2. Navigate to your `.AppImage` file
3. Select and click "Open"
4. Wait for installation to complete

### Method 3: From Discover

1. Navigate to the "Discover" tab
2. Browse available AppImages
3. Click "Install" on your chosen app
4. Linuxy downloads and installs automatically

### Installation Process

During installation, you'll see:

```
┌─────────────────────────────────┐
│  Installing Application...      │
│  ████████░░░░░░░░  65%          │
│  Extracting icon...             │
└─────────────────────────────────┘
```

Steps performed:
1. Copy AppImage to `~/.local/appimages/`
2. Extract application icon
3. Create desktop entry
4. Update system menus

### Post-Installation

After installation:
- App appears in your Library
- Desktop entry created in application menu
- Icon available in system icon cache

---

## Managing Apps

### Launching Apps

**From Linuxy:**
1. Find the app in your Library
2. Click the ▶️ **Launch** button

**From System Menu:**
1. Open your application menu
2. Find the app (usually under the app's category)
3. Click to launch

### Removing Apps

1. Click the 🗑️ **Remove** button on the app card
2. Confirm the removal dialog
3. AppImage, desktop entry, and icons are deleted

**Note:** This permanently deletes the AppImage file.

### Viewing App Details

Hover over or click an app card to see:
- File size
- Installation date
- Sandbox status
- Version (if available)

---

## Sandboxing with Firejail

### What is Firejail?

Firejail is a Linux sandbox program that restricts the running environment of untrusted applications. When enabled for an AppImage, it limits access to:

- Your home directory files
- System configuration
- Network (optional)
- Other running processes

### Checking Firejail Status

```bash
# Check if Firejail is installed
which firejail

# If not installed
sudo apt install firejail  # Debian/Ubuntu
sudo dnf install firejail  # Fedora
sudo pacman -S firejail    # Arch
```

### Enabling Sandboxing

**Per-App Basis:**

1. Find the app in your Library
2. Click the 🛡️ **Shield** icon (top-right of app card)
3. The shield turns green when sandboxing is enabled

**Visual Indicators:**
- 🛡️ Gray = Not sandboxed
- ✅ Green = Sandboxed

### How Sandboxing Works

When you enable sandboxing for an app:

1. Linuxy modifies the app's `.desktop` file
2. Adds `firejail --appimage` to the Exec command
3. Next launch runs the app in a sandbox

**Example:**
```bash
# Without sandboxing
Exec=/home/user/.local/appimages/Firefox.AppImage

# With sandboxing
Exec=firejail --appimage /home/user/.local/appimages/Firefox.AppImage
```

### Disabling Sandboxing

1. Click the shield icon again
2. Confirm you want to disable
3. App will run without restrictions next launch

### Sandboxing Limitations

Some AppImages may not work correctly with Firejail due to:
- Required system access
- Hardware acceleration needs
- Specific kernel features

If an app fails to launch with sandboxing, disable it and try again.

---

## Updating Apps

### Manual Update Check

1. Click the 🔄 **Update** button on an app card
2. Linuxy checks for newer versions when `appimageupdatetool` is installed
3. If available, confirms download
4. Downloads and replaces the AppImage

### Update Indicators

- **Up to date**: "Your AppImage is already up to date"
- **Update available**: "An update is available! Download now?"
- **Check failed**: Error message with details

### Auto-Update Support

Not all AppImages support automatic updates. Support depends on:
- AppImage having update metadata (zsync2)
- Developer providing update information
- Accessible update server
- `appimageupdatetool` being installed on your system

### Best Practices

- Check for updates regularly
- Review changelogs before updating
- Keep backups of important AppImages

---

## Settings & Configuration

### Appearance

**Theme Options:**
- 🌙 **Dark** - Default dark theme
- ☀️ **Light** - Light theme for bright environments
- 💻 **System** - Follows system preference

To change theme:
1. Go to Settings
2. Click your preferred theme
3. Changes apply immediately

### Storage

**View Storage Usage:**
- Total disk space used by AppImages
- Number of installed apps

**Open AppImage Folder:**
```
Settings → Library Storage → Open Folder
```
Opens `~/.local/appimages/` in your file manager

### Security

**Firejail Status:**
- Shows if Firejail is installed
- Provides installation instructions
- Explains sandboxing benefits

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl + O` | Open/Browse for AppImage |
| `Ctrl + R` | Refresh library |
| `Ctrl + ,` | Open settings |
| `Ctrl + Q` | Quit application |
| `Delete` | Remove selected app |
| `F5` | Refresh library |
| `F1` | Open help/documentation |

---

## Tips & Tricks

### Organizing Your Library

While Linuxy doesn't support folders, you can:
- Use descriptive app names
- Keep related AppImages together
- Remove unused apps regularly

### Performance Tips

1. **Keep library small**: Remove unused AppImages
2. **Use SSD**: Faster app launches
3. **Limit sandboxing**: Only sandbox untrusted apps

### Backup Your Apps

```bash
# Backup all AppImages
tar -czf appimages-backup.tar.gz ~/.local/appimages/

# Restore
tar -xzf appimages-backup.tar.gz -C ~/
```

### Custom Icons

If an app has a missing or wrong icon:

1. Find or create a PNG/SVG icon
2. Name it `{app-name}_icon.png`
3. Place in `~/.local/share/icons/`
4. Refresh Linuxy

### Command Line Usage

While Linuxy is GUI-focused, you can:

```bash
# Launch Linuxy
linuxy

# Check version
linuxy --version

# Show help
linuxy --help
```

### Troubleshooting Common Issues

**App won't launch:**
```bash
# Make AppImage executable
chmod +x ~/.local/appimages/app-name.AppImage

# Try launching directly
~/.local/appimages/app-name.AppImage
```

**Missing from system menu:**
```bash
# Refresh desktop database
update-desktop-database ~/.local/share/applications/
```

**Icon not showing:**
```bash
# Refresh icon cache
gtk-update-icon-cache -f ~/.local/share/icons/
```

---

## Advanced Usage

### Manual Desktop File Editing

Location: `~/.local/share/applications/`

Example `.desktop` file:
```ini
[Desktop Entry]
Type=Application
Name=My App
Exec=firejail --appimage /home/user/.local/appimages/myapp.AppImage
Icon=myapp_icon
Categories=Utility;
```

### Custom Firejail Profiles

Create custom profiles in `~/.config/firejail/`:

```bash
# Custom profile for specific app
mkdir -p ~/.config/firejail
nano ~/.config/firejail/myapp.profile
```

### Environment Variables

```bash
# Enable debug logging
export RUST_LOG=debug
linuxy

# Custom config directory
export XDG_CONFIG_HOME=/custom/path
```

---

## Getting Help

- **In-App Help**: Click the Help button in sidebar
- **Documentation**: [GitHub Wiki](https://github.com/swadhinbiswas/linuxy/wiki)
- **Issues**: [Report a bug](https://github.com/swadhinbiswas/linuxy/issues)
- **Discussions**: [Community forum](https://github.com/swadhinbiswas/linuxy/discussions)

---

<div align="center">
  <p><strong>Enjoy managing your AppImages with Linuxy!</strong></p>
  <p>🐧 Built with love for the Linux community</p>
</div>
