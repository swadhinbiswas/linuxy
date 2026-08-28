{
  description = "Linuxy - Multi-platform Desktop Application Manager";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };
        bun = pkgs.bun;

        linuxy = pkgs.stdenv.mkDerivation rec {
          pname = "linuxy";
          version = "2.0.4";
          src = self;

          nativeBuildInputs = with pkgs; [
            rustToolchain
            bun
            pkg-config
            wrapGAppsHook
            autoPatchelfHook
            llvmPackages.clangUseLLVM
            makeWrapper
          ];

          buildInputs = with pkgs; [
            webkitgtk_4_1
            gtk3
            libappindicator-gtk3
            librsvg
            openssl
            libxdo
            xdg-utils
            firejail
            glib
            cairo
            pango
            gdk-pixbuf
            atk
            libsoup_3
            harfbuzz
            freetype
            gsettings-desktop-schemas
          ];

          dontWrapGApps = true;

          buildPhase = ''
            export HOME=$TMPDIR
            export BUN_INSTALL_CACHE_DIR=$TMPDIR/bun-cache
            bun install --offline --frozen-lockfile
            bun run build
            cd src-tauri
            cargo build --release --locked
            cd ..
          '';

          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/share/applications
            mkdir -p $out/share/icons/hicolor
            mkdir -p $out/share/doc/linuxy

            cp src-tauri/target/release/linuxy $out/bin/linuxy
            cp src-tauri/debian/desktop-template.desktop $out/share/applications/linuxy.desktop

            for size in 32 128 256 512; do
              icon="src-tauri/icons/''${size}x''${size}.png"
              dir="$out/share/icons/hicolor/''${size}x''${size}/apps"
              mkdir -p "$dir"
              [ -f "$icon" ] && cp "$icon" "$dir/linuxy.png"
            done
            [ -f "src-tauri/icons/icon.png" ] && cp "src-tauri/icons/icon.png" \
              "$out/share/icons/hicolor/512x512/apps/linuxy.png"

            cp LICENSE $out/share/doc/linuxy/copyright

            wrapProgram $out/bin/linuxy \
              "''${gappsWrapperArgs[@]}" \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.firejail pkgs.xdg-utils ]} \
              --set WEBKIT_DISABLE_COMPOSITING_MODE 1 \
              --set WEBKIT_FORCE_SANDBOX 0
          '';

          meta = with pkgs.lib; {
            description = "Multi-platform Desktop Application Manager with Firejail sandboxing";
            homepage = "https://github.com/swadhinbiswas/linuxy";
            license = licenses.mit;
            platforms = [ "x86_64-linux" "aarch64-linux" ];
            maintainers = [ ];
            mainProgram = "linuxy";
          };
        };
      in
      {
        packages.default = linuxy;
        packages.linuxy = linuxy;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            bun
            pkg-config
            webkitgtk_4_1
            gtk3
            libappindicator-gtk3
            librsvg
            openssl
            libxdo
            firejail
            xdg-utils
          ];
          WEBKIT_DISABLE_COMPOSITING_MODE = "1";
          WEBKIT_FORCE_SANDBOX = "0";
        };

        apps.default = {
          type = "app";
          program = "${linuxy}/bin/linuxy";
        };
      });
}
