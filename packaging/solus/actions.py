actions: ['build', 'install']
build:
  - npm install
  - npm run build
  - cd src-tauri && cargo build --release --locked
install:
  - install -Dm755 src-tauri/target/release/linuxy /usr/bin/linuxy
  - install -Dm644 src-tauri/debian/desktop-template.desktop /usr/share/applications/linuxy.desktop
  - for size in 32 128 256 512; do
      icon="src-tauri/icons/''${size}x''${size}.png";
      [ -f "$icon" ] && install -Dm644 "$icon" "/usr/share/icons/hicolor/''${size}x''${size}/apps/linuxy.png";
    done
  - [ -f "src-tauri/icons/icon.png" ] && install -Dm644 src-tauri/icons/icon.png /usr/share/icons/hicolor/512x512/apps/linuxy.png
  - install -Dm644 LICENSE /usr/share/licenses/linuxy/LICENSE
