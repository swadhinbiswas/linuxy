#!/bin/bash
# Universal installer for Linuxy
# Detects the operating system/distro and installs using the appropriate method
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

detect_distro() {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO_ID="$ID"
    DISTRO_ID_LIKE="$ID_LIKE"
    DISTRO_NAME="$NAME"
  elif [ -f /etc/debian_version ]; then
    DISTRO_ID="debian"
    DISTRO_ID_LIKE="debian"
    DISTRO_NAME="Debian"
  elif [ -f /etc/redhat-release ]; then
    DISTRO_ID="rhel"
    DISTRO_ID_LIKE="fedora"
    DISTRO_NAME="RHEL"
  elif command -v pacman &>/dev/null; then
    DISTRO_ID="arch"
    DISTRO_ID_LIKE="arch"
    DISTRO_NAME="Arch Linux"
  elif command -v apk &>/dev/null; then
    DISTRO_ID="alpine"
    DISTRO_ID_LIKE="alpine"
    DISTRO_NAME="Alpine Linux"
  elif command -v xbps-install &>/dev/null; then
    DISTRO_ID="void"
    DISTRO_ID_LIKE="void"
    DISTRO_NAME="Void Linux"
  elif command -v eopkg &>/dev/null; then
    DISTRO_ID="solus"
    DISTRO_ID_LIKE="solus"
    DISTRO_NAME="Solus"
  elif [ -f /etc/NIXOS ]; then
    DISTRO_ID="nixos"
    DISTRO_ID_LIKE="nixos"
    DISTRO_NAME="NixOS"
  elif uname -s | grep -qi darwin; then
    DISTRO_ID="macos"
    DISTRO_ID_LIKE="darwin"
    DISTRO_NAME="macOS"
  elif uname -s | grep -qi MINGW\|CYGWIN\|MSYS; then
    DISTRO_ID="windows"
    DISTRO_ID_LIKE="windows"
    DISTRO_NAME="Windows"
  else
    DISTRO_ID="unknown"
    DISTRO_ID_LIKE=""
    DISTRO_NAME="Unknown"
  fi
}

install_deps() {
  log_info "Installing dependencies for $DISTRO_NAME..."

  case "$DISTRO_ID" in
    debian|ubuntu|pop|linuxmint|elementary|zorin)
      sudo apt update
      sudo apt install -y firejail xdg-utils libgtk-3-0 libwebkit2gtk-4.1-0 \
        libayatana-appindicator3-1 libxdo3
      ;;
    fedora|rhel|centos)
      sudo dnf install -y firejail xdg-utils gtk3 webkit2gtk4.1 libappindicator-gtk3
      ;;
    arch|manjaro|endeavouros|artix)
      sudo pacman -S --needed firejail xdg-utils gtk3 webkit2gtk libappindicator-gtk3 xdo
      ;;
    alpine)
      sudo apk add firejail xdg-utils gtk+3.0 webkit2gtk-4.1 libappindicator-gtk3 libxdo
      ;;
    void)
      sudo xbps-install -S firejail xdg-utils gtk+3 webkit2gtk4.1 libappindicator-gtk3 libXdo
      ;;
    solus)
      sudo eopkg install firejail xdg-utils gtk3 webkit2gtk-4.1 libappindicator-gtk3 libxdo
      ;;
    nixos)
      log_warn "On NixOS, add linuxy to environment.systemPackages in your configuration.nix"
      log_warn "  inputs.linuxy.url = github:swadhinbiswas/linuxy;"
      log_warn "  or use: nix run github:swadhinbiswas/linuxy"
      return 0
      ;;
    opensuse*|suse)
      sudo zypper install firejail xdg-utils gtk3 webkit2gtk4-1 typelib-1_0-AppIndicator3-0_1
      ;;
    gentoo)
      sudo emerge --ask app-admin/firejail x11-misc/xdg-utils x11-libs/gtk+:3 net-libs/webkit-gtk:4.1
      ;;
    *)
      log_error "Unknown distro. Please install dependencies manually."
      log_info "Required: firejail, xdg-utils, gtk3, webkit2gtk, libappindicator"
      exit 1
      ;;
  esac
}

install_linuxy() {
  log_info "Installing Linuxy..."

  # Use built packages if available
  local deb="$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/deb/" -name '*.deb' 2>/dev/null | head -1)"
  local rpm="$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/rpm/" -name '*.rpm' 2>/dev/null | head -1)"
  local appimage="$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/appimage/" -name '*.AppImage' 2>/dev/null | head -1)"
  local binary="$PROJECT_ROOT/src-tauri/target/release/linuxy"

  case "$DISTRO_ID" in
    debian|ubuntu|pop|linuxmint|elementary|zorin|kali|deepin|parrot)
      if [ -n "$deb" ]; then
        install_deb "$deb"
      elif [ -f "$binary" ]; then
        install_binary
      else
        log_error "No build artifacts found. Run 'scripts/build.sh' first."
        exit 1
      fi
      ;;
    fedora|rhel|centos)
      if [ -n "$rpm" ]; then
        install_rpm "$rpm"
      elif [ -f "$binary" ]; then
        install_binary
      else
        log_error "No build artifacts found."
        exit 1
      fi
      ;;
    arch|manjaro*|endeavouros|artix)
      if [ -f "$binary" ]; then
        install_binary
      else
        log_info "On Arch Linux, install linuxy from AUR: yay -S linuxy"
        exit 0
      fi
      ;;
    alpine|void|solus|opensuse*|gentoo|slackware)
      install_binary
      ;;
    nixos)
      log_info "On NixOS, use: nix profile install github:swadhinbiswas/linuxy"
      exit 0
      ;;
    macos)
      local mac_bundle="$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/dmg/" -name '*.dmg' 2>/dev/null | head -1)"
      if [ -n "$mac_bundle" ]; then
        hdiutil attach "$mac_bundle"
        cp -R "/Volumes/Linuxy/Linuxy.app" /Applications/
        hdiutil detach "/Volumes/Linuxy"
        log_ok "Linuxy installed to /Applications/"
      else
        log_error "macOS bundle not found."
        exit 1
      fi
      ;;
    windows)
      local exe="$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/nsis/" -name '*.exe' 2>/dev/null | head -1)"
      if [ -n "$exe" ]; then
        log_info "Run $exe to install Linuxy on Windows."
      else
        log_error "Windows installer not found."
        exit 1
      fi
      ;;
    *)
      if [ -n "$appimage" ]; then
        install_appimage "$appimage"
      elif [ -f "$binary" ]; then
        install_binary
      else
        log_error "No install method available."
        exit 1
      fi
      ;;
  esac
}

install_deb() {
  local pkg="$1"
  log_info "Installing DEB package..."
  if command -v gdebi &>/dev/null; then
    sudo gdebi -n "$pkg"
  else
    sudo dpkg -i "$pkg" && sudo apt install -f -y
  fi
  log_ok "Linuxy installed via DEB."
}

install_rpm() {
  local pkg="$1"
  log_info "Installing RPM package..."
  if command -v dnf &>/dev/null; then
    sudo dnf install -y "$pkg"
  elif command -v zypper &>/dev/null; then
    sudo zypper install --allow-unsigned-rpm "$pkg"
  else
    sudo rpm -i "$pkg"
  fi
  log_ok "Linuxy installed via RPM."
}

install_appimage() {
  local path="$1"
  local bin_dest="$HOME/.local/appimages"
  local desktop_dest="$HOME/.local/share/applications"
  mkdir -p "$bin_dest" "$desktop_dest"
  cp "$path" "$bin_dest/linuxy.AppImage"
  chmod +x "$bin_dest/linuxy.AppImage"

  cat > "$desktop_dest/linuxy.desktop" << EOF
[Desktop Entry]
Type=Application
Name=Linuxy
Comment=One-click Linux Application Manager
Exec="$bin_dest/linuxy.AppImage" %U
Icon=linuxy
Categories=Utility;System;
Terminal=false
EOF

  # Integrate with system
  if command -v update-desktop-database &>/dev/null; then
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
  fi

  log_ok "Linuxy AppImage installed to $bin_dest/linuxy.AppImage"
  log_info "You can run it from your application menu or directly: $bin_dest/linuxy.AppImage"
}

install_binary() {
  local src="$PROJECT_ROOT/src-tauri/target/release/linuxy"
  log_info "Installing binary to /usr/local/bin..."
  sudo install -Dm755 "$src" "/usr/local/bin/linuxy"
  sudo install -Dm644 "$PROJECT_ROOT/src-tauri/debian/desktop-template.desktop" "/usr/share/applications/linuxy.desktop"

  for size in 32 128 256 512; do
    icon="$PROJECT_ROOT/src-tauri/icons/${size}x${size}.png"
    if [ -f "$icon" ]; then
      sudo install -Dm644 "$icon" "/usr/share/icons/hicolor/${size}x${size}/apps/linuxy.png"
    fi
  done
  if [ -f "$PROJECT_ROOT/src-tauri/icons/icon.png" ]; then
    sudo install -Dm644 "$PROJECT_ROOT/src-tauri/icons/icon.png" "/usr/share/icons/hicolor/512x512/apps/linuxy.png"
  fi

  if command -v update-desktop-database &>/dev/null; then
    sudo update-desktop-database /usr/share/applications 2>/dev/null || true
  fi

  log_ok "Linuxy installed to /usr/local/bin/linuxy"
}

uninstall_linuxy() {
  log_info "Removing Linuxy..."
  sudo rm -f /usr/local/bin/linuxy
  sudo rm -f /usr/share/applications/linuxy.desktop
  for size in 32 128 256 512; do
    sudo rm -f "/usr/share/icons/hicolor/${size}x${size}/apps/linuxy.png"
  done
  sudo rm -f "/usr/share/icons/hicolor/512x512/apps/linuxy.png"
  rm -f "$HOME/.local/appimages/linuxy.AppImage"
  rm -f "$HOME/.local/share/applications/linuxy.desktop"
  rm -f "$HOME/.local/share/applications/linuxy.AppImage"
  if command -v update-desktop-database &>/dev/null; then
    sudo update-desktop-database /usr/share/applications 2>/dev/null || true
    update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true
  fi
  log_ok "Linuxy removed."
}

main() {
  echo ""
  echo "  Linuxy Universal Installer"
  echo "  ========================="
  echo ""

  detect_distro
  log_info "Detected: $DISTRO_NAME ($DISTRO_ID)"

  case "$1" in
    deps|dependencies|--deps)
      install_deps
      log_ok "Dependencies installed."
      exit 0
      ;;
    uninstall|remove|--remove)
      uninstall_linuxy
      exit 0
      ;;
    help|--help|-h)
      echo "Usage: $0 [command]"
      echo ""
      echo "Commands:"
      echo "  (no command)   Install Linuxy on detected OS"
      echo "  deps           Install runtime dependencies only"
      echo "  uninstall      Remove Linuxy from system"
      echo "  help           Show this help"
      exit 0
      ;;
  esac

  install_deps
  install_linuxy

  echo ""
  log_ok "Installation complete! Run 'linuxy' to start."
}

main "$@"
