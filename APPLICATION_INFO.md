# Linuxy App Information

## ✅ Fixed Issues

### 1. Window Controls (Minimize, Maximize, Close)
**Fixed!** Added `decorations: true` to `tauri.conf.json`

The app window now shows:
- ✅ Minimize button
- ✅ Maximize/Restore button
- ✅ Close button
- ✅ Native window border

### 2. App Icons
**Already included!** Icons are in `src-tauri/icons/`:
- `32x32.png` - Small icon
- `128x128.png` - Standard icon
- `128x128@2x.png` - Retina icon
- `icon.icns` - macOS icon
- `icon.ico` - Windows icon

After installation, the app will show properly in:
- ✅ Application menu
- ✅ Desktop (if you create a shortcut)
- ✅ Taskbar/Dock
- ✅ Window title bar

### 3. Desktop Integration
**Configured!** The DEB package includes:
- Desktop entry file (`.desktop`)
- Icon installation
- Menu category (Utility/System)
- MIME type associations

### 4. Developer Signature & Verification
**Setup complete!**

#### Package Metadata:
```
Product Name: Linuxy
Version: 0.1.0
Maintainer: Linuxy Contributors
Developer: @swadhinbiswas
Identifier: com.linuxy.dev
License: MIT (Open Source)
```

#### Code Signing:
- **Script**: `./scripts/sign-package.sh`
- **Guide**: `docs/CODE_SIGNING.md`
- **Method**: GPG signature for DEB packages

## 🚀 How to Run

### Development Mode
```bash
npm run tauri dev
```

This will:
1. Start Vite dev server on `http://localhost:1420`
2. Compile Rust backend
3. **Launch the app window with decorations**

### Build for Production
```bash
npm run tauri build
```

Output location:
```
src-tauri/target/release/bundle/deb/linuxy_0.1.0_amd64.deb
```

### Install the DEB Package
```bash
sudo apt install ./src-tauri/target/release/bundle/deb/linuxy_0.1.0_amd64.deb
```

After installation:
- Find "Linuxy" in your application menu
- Or run from terminal: `linuxy`

### Sign the Package (Optional but Recommended)
```bash
./scripts/sign-package.sh
```

## 📋 Window Configuration

Current window settings in `tauri.conf.json`:

```json
{
  "width": 800,
  "height": 600,
  "resizable": true,
  "decorations": true,    // ← Shows window controls
  "transparent": false,   // ← Native window background
  "center": true,         // ← Center on screen
  "title": "Linuxy",
  "fileDropEnabled": true
}
```

## 🎨 App Icon Preview

The app icon is a 🐧 (penguin emoji) displayed in:
- Window title bar
- Taskbar
- Application menu
- Desktop shortcuts

## 🔐 Verification

### Verify DEB Package Signature
```bash
dpkg-sig --verify linuxy_*.deb
```

### Check Package Info
```bash
dpkg-deb --info linuxy_*.deb
```

### View Package Contents
```bash
dpkg-deb --contents linuxy_*.deb
```

Expected output includes:
```
/usr/bin/linuxy
/usr/share/applications/linuxy.desktop
/usr/share/icons/hicolor/128x128/apps/linuxy.png
```

## 📦 What's Included in DEB Package

| File | Location | Purpose |
|------|----------|---------|
| Binary | `/usr/bin/linuxy` | Executable |
| Desktop | `/usr/share/applications/linuxy.desktop` | Menu entry |
| Icon 128 | `/usr/share/icons/hicolor/128x128/apps/linuxy.png` | App icon |
| Icon 32 | `/usr/share/icons/hicolor/32x32/apps/linuxy.png` | Small icon |

## 🐛 Troubleshooting

### App doesn't show in menu
```bash
update-desktop-database ~/.local/share/applications/
```

### Icon not showing
```bash
gtk-update-icon-cache -f /usr/share/icons/hicolor/
```

### Window has no decorations
This is handled by your window manager (GNOME, KDE, etc.). The app requests native decorations. If still not showing:
- Check your WM settings
- Try restarting the app

### Package installation fails
```bash
sudo apt --fix-broken install
```

## 📞 Support

- **GitHub**: https://github.com/swadhinbiswas/linuxy
- **Issues**: https://github.com/swadhinbiswas/linuxy/issues
- **License**: MIT (Open Source)

---

**Last Updated**: March 2024
**Version**: 0.1.0
**License**: MIT - Open Source
