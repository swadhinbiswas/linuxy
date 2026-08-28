Name:           linuxy
Version:        2.0.4
Release:        1%{?dist}
Summary:        Multi-platform Desktop Application Manager

License:        MIT
URL:            https://github.com/swadhinbiswas/linuxy
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  nodejs
BuildRequires:  npm
BuildRequires:  pkgconfig(webkit2gtk-4.1)
BuildRequires:  pkgconfig(gtk+-3.0)
BuildRequires:  (pkgconfig(libappindicator3-0.1) or pkgconfig(ayatana-appindicator3-0.1))
BuildRequires:  pkgconfig(librsvg-2.0)
BuildRequires:  openssl-devel
BuildRequires:  libxdo-devel
BuildRequires:  systemd-rpm-macros

Requires:       firejail
Requires:       xdg-utils
Requires:       gtk3
Requires:       webkit2gtk4.1
Requires:       (libappindicator-gtk3 or libayatana-appindicator-gtk3)

%description
Linuxy is a one-click application manager for Linux (and Windows/macOS)
that lets you discover, install, and manage desktop applications
with optional Firejail sandboxing for enhanced security.

Features:
  * AppImage discovery from multiple sources
  * One-click AppImage installation and management
  * Firejail sandboxing support
  * Automatic AppImage updates
  * Cross-platform (Linux, Windows, macOS)

%prep
%autosetup -n linuxy-%{version}

%build
npm install
npm run build
cd src-tauri
cargo build --release --locked

%install
install -Dm755 src-tauri/target/release/linuxy %{buildroot}%{_bindir}/linuxy
install -Dm644 src-tauri/debian/desktop-template.desktop %{buildroot}%{_datadir}/applications/linuxy.desktop

for size in 32 128 256 512; do
  icon="src-tauri/icons/${size}x${size}.png"
  if [ -f "$icon" ]; then
    install -Dm644 "$icon" "%{buildroot}%{_datadir}/icons/hicolor/${size}x${size}/apps/linuxy.png"
  fi
done
if [ -f "src-tauri/icons/icon.png" ]; then
  install -Dm644 "src-tauri/icons/icon.png" "%{buildroot}%{_datadir}/icons/hicolor/512x512/apps/linuxy.png"
fi

install -Dm644 LICENSE %{buildroot}%{_datadir}/licenses/linuxy/LICENSE

%files
%{_bindir}/linuxy
%{_datadir}/applications/linuxy.desktop
%{_datadir}/icons/hicolor/*/apps/linuxy.png
%{_datadir}/licenses/linuxy/LICENSE

%doc README.md

%changelog
* Sat Aug 29 2026 Swadhin Biswas <swadhin.biswas@example.com> - 2.0.4-1
- Fix all CodeRabbit review findings (PR #41)
- Harden desktop entry handling and storage cleanup
- Fix packaging and CI/CD

* Mon Jun 24 2026 Swadhin Biswas <swadhin.biswas@example.com> - 2.0.0-1
- Tauri 2.0 migration with cross-platform support
- Added Windows and macOS support
- Added system tray, auto-updater, single-instance, CLI, deep linking

* Tue Jan 01 2025 Swadhin Biswas <swadhin.biswas@example.com> - 1.2.0-1
- Initial release
