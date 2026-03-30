# Production Readiness Checklist

This checklist tracks the minimum baseline Linuxy should satisfy before release.

## Runtime Safety

- [x] Restrict Tauri allowlist to the APIs actually used by the app
- [x] Define a production CSP for remote catalog access and local asset loading
- [x] Keep external URL opening in the backend and validate allowed URL schemes
- [x] Gate optional update functionality when `appimageupdatetool` is
      unavailable

## Install / Update / Remove Reliability

- [x] Reject non-AppImage / non-ELF input before installation
- [x] Validate AppImage extraction before copying into `~/.local/appimages`
- [x] Avoid leaving partially installed binaries behind on failed install
- [x] Reject failed HTTP downloads before attempting installation
- [x] Remove installed app binaries, desktop entries, and icons consistently

## Background Behavior

- [x] Avoid prompting on partially downloaded AppImages
- [x] Bound watcher state so it does not grow forever during long sessions
- [x] Handle missing home/downloads directories without panicking

## Repository Quality Gates

- [x] Make frontend CI run the declared project checks
- [x] Make security audit jobs fail on real findings
- [x] Restore valid `clippy.toml` configuration
- [x] Use stable `rustfmt` configuration
- [x] Add backend unit tests for critical helper behavior

## Release Verification

- [x] Run frontend checks in a clean dependency install
- [x] Run `cargo clippy -- -D warnings`
- [x] Run `cargo fmt -- --check`
- [x] Run full Tauri bundle build on release target(s)
- [x] Perform install/remove QA via CI on clean Linux VM
- [x] Verify catalog fetch and image loading under the production CSP
