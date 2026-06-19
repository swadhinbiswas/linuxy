use std::fs;
use std::io::{Read, Seek};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::{Emitter, Window};
use uuid::Uuid;

fn is_appimage_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("AppImage") | Some("appimage")
    )
}

fn is_deb_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("deb") | Some("DEB")
    )
}

fn is_rpm_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rpm") | Some("RPM")
    )
}

fn ensure_elf_header(path: &Path) -> Result<(), String> {
    let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut header = [0_u8; 4];
    file.read_exact(&mut header)
        .map_err(|_| "AppImage file is too small to be valid.".to_string())?;

    if header == [0x7f, b'E', b'L', b'F'] {
        Ok(())
    } else {
        Err("The selected file is not a valid ELF/AppImage binary.".into())
    }
}

fn set_executable(path: &Path) -> Result<(), String> {
    let mut perms = fs::metadata(path).map_err(|e| e.to_string())?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).map_err(|e| e.to_string())
}

fn replace_file(from: &Path, to: &Path) -> Result<(), String> {
    if let Err(error) = fs::rename(from, to) {
        if to.exists() {
            fs::remove_file(to).map_err(|e| e.to_string())?;
            fs::rename(from, to).map_err(|e| e.to_string())?;
        } else {
            return Err(error.to_string());
        }
    }

    Ok(())
}

fn copy_icon_from_extract(
    squashfs_root: &Path,
    icons_dir: &Path,
    base_name: &str,
    parsed_icon_name: &str,
) -> Vec<PathBuf> {
    let mut copied_icons = Vec::new();

    if !parsed_icon_name.is_empty() {
        let clean_icon_name = parsed_icon_name
            .trim_end_matches(".png")
            .trim_end_matches(".svg")
            .trim_end_matches(".xpm")
            .trim_end_matches(".jpg")
            .trim_end_matches(".jpeg")
            .to_string();

        let possible_icons = vec![
            squashfs_root.join(format!("{}.png", clean_icon_name)),
            squashfs_root.join(format!("{}.svg", clean_icon_name)),
            squashfs_root.join(format!("{}.xpm", clean_icon_name)),
            squashfs_root.join(parsed_icon_name),
            squashfs_root.join(".DirIcon"),
        ];

        for source in possible_icons {
            if source.exists() {
                let extension = source
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .filter(|ext| !ext.is_empty())
                    .unwrap_or("png");
                let dest = icons_dir.join(format!("{}_icon.{}", base_name, extension));
                if fs::copy(&source, &dest).is_ok() {
                    copied_icons.push(dest);
                    return copied_icons;
                }
            }
        }
    }

    if let Ok(entries) = fs::read_dir(squashfs_root) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|ext| ext.to_str()) == Some("png") {
                let dest = icons_dir.join(format!("{}_icon.png", base_name));
                if fs::copy(entry.path(), &dest).is_ok() {
                    copied_icons.push(dest);
                }
                break;
            }
        }
    }

    copied_icons
}

fn cleanup_failed_install(app_path: &Path, desktop_path: &Path, icon_paths: &[PathBuf]) {
    let _ = fs::remove_file(app_path);
    let _ = fs::remove_file(desktop_path);

    for icon_path in icon_paths {
        let _ = fs::remove_file(icon_path);
    }
}

fn detect_package_manager() -> Option<(String, String)> {
    let managers = vec![
        ("pacman", "sudo pacman -U"),
        ("apt", "sudo apt install"),
        ("dpkg", "sudo dpkg -i"),
        ("dnf", "sudo dnf install"),
        ("yum", "sudo yum localinstall"),
        ("zypper", "sudo zypper install"),
    ];

    for (cmd, _) in &managers {
        if Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some((
                cmd.to_string(),
                managers
                    .iter()
                    .find(|(c, _)| c == cmd)
                    .map(|(_, i)| i.to_string())
                    .unwrap(),
            ));
        }
    }
    None
}

fn install_deb_with_debtap(path: &Path, window: Option<&Window>) -> Result<String, String> {
    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Checking debtap availability...");
    }

    let debtap_check = Command::new("which").arg("debtap").output().map_err(|_| {
        "debtap is not installed. Please install it first: yay -S debtap".to_string()
    })?;

    if !debtap_check.status.success() {
        return Err("debtap is not installed. Please install it first: yay -S debtap".to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Updating debtap database...");
    }

    let update_output = Command::new("debtap")
        .arg("-u")
        .output()
        .map_err(|e| format!("Failed to run debtap -u: {}", e))?;

    if !update_output.status.success() {
        let err = String::from_utf8_lossy(&update_output.stderr);
        return Err(format!("debtap database update failed: {}", err.trim()));
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Converting DEB package...");
    }

    let convert_output = Command::new("debtap")
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run debtap: {}", e))?;

    if !convert_output.status.success() {
        let err = String::from_utf8_lossy(&convert_output.stderr);
        return Err(format!("debtap conversion failed: {}", err.trim()));
    }

    let stdout = String::from_utf8_lossy(&convert_output.stdout);
    let pkg_path = stdout
        .lines()
        .find(|line| line.contains(".pkg.tar"))
        .map(|line| line.trim().to_string())
        .ok_or("Could not find converted package path from debtap output".to_string())?;

    let pkg_path = Path::new(&pkg_path);
    if !pkg_path.exists() {
        return Err("Converted package file not found".to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Installing converted package...");
    }

    let install_status = Command::new("sudo")
        .arg("pacman")
        .arg("-U")
        .arg("--noconfirm")
        .arg(pkg_path)
        .status()
        .map_err(|e| format!("Failed to install package: {}", e))?;

    if !install_status.success() {
        return Err("Package installation failed".to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Done");
    }

    Ok("Successfully installed DEB package via debtap".to_string())
}

fn install_deb_direct(path: &Path, window: Option<&Window>) -> Result<String, String> {
    let (manager, _install_cmd) = detect_package_manager()
        .ok_or("No supported package manager found (pacman, apt, dpkg, dnf, zypper)".to_string())?;

    if let Some(ref w) = window {
        let _ = w.emit(
            "install-progress",
            format!("Installing DEB package with {}...", manager),
        );
    }

    let install_status = match manager.as_str() {
        "pacman" => {
            return install_deb_with_debtap(path, window);
        },
        "apt" => Command::new("sudo")
            .arg("apt")
            .arg("install")
            .arg("-y")
            .arg(path)
            .status(),
        "dpkg" => {
            let dpkg_status = Command::new("sudo")
                .arg("dpkg")
                .arg("-i")
                .arg(path)
                .status();
            if dpkg_status.as_ref().map(|s| s.success()).unwrap_or(false) {
                let _ = Command::new("sudo")
                    .arg("apt")
                    .arg("install")
                    .arg("-f")
                    .arg("-y")
                    .status();
            }
            dpkg_status
        },
        "dnf" => Command::new("sudo")
            .arg("dnf")
            .arg("install")
            .arg("-y")
            .arg(path)
            .status(),
        "yum" => Command::new("sudo")
            .arg("yum")
            .arg("localinstall")
            .arg("-y")
            .arg(path)
            .status(),
        "zypper" => Command::new("sudo")
            .arg("zypper")
            .arg("install")
            .arg("-y")
            .arg(path)
            .status(),
        _ => return Err(format!("Unsupported package manager: {}", manager)),
    }
    .map_err(|e| format!("Failed to install package: {}", e))?;

    if !install_status.success() {
        return Err("Package installation failed. Check terminal output for details.".to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Done");
    }

    Ok(format!(
        "Successfully installed DEB package via {}",
        manager
    ))
}

#[tauri::command]
pub async fn install_deb(path: String, window: Window) -> Result<String, String> {
    install_deb_internal(path, Some(window)).await
}

pub async fn install_deb_internal(path: String, window: Option<Window>) -> Result<String, String> {
    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Initializing DEB installation...");
    }

    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("DEB file does not exist.".into());
    }
    if !is_deb_path(source_path) {
        return Err("Only .deb files are supported.".into());
    }

    install_deb_direct(source_path, window.as_ref())
}

fn install_rpm_direct(path: &Path, window: Option<&Window>) -> Result<String, String> {
    let managers = vec![
        ("dnf", vec!["install", "-y"]),
        ("zypper", vec!["install", "-y"]),
        ("yum", vec!["localinstall", "-y"]),
        ("rpm", vec!["-i", "--nodeps"]),
    ];

    let (manager, args) = managers
        .into_iter()
        .find(|(cmd, _)| {
            Command::new("which")
                .arg(cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .ok_or("No supported RPM package manager found (dnf, zypper, yum, rpm)".to_string())?;

    if let Some(ref w) = window {
        let _ = w.emit(
            "install-progress",
            format!("Installing RPM package with {}...", manager),
        );
    }

    let mut cmd = Command::new("sudo");
    cmd.arg(manager);
    for arg in &args {
        cmd.arg(arg);
    }
    cmd.arg(path);

    let install_status = cmd
        .status()
        .map_err(|e| format!("Failed to install package: {}", e))?;

    if !install_status.success() {
        return Err("Package installation failed. Check terminal output for details.".to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Done");
    }

    Ok(format!(
        "Successfully installed RPM package via {}",
        manager
    ))
}

#[tauri::command]
pub async fn install_rpm(path: String, window: Window) -> Result<String, String> {
    install_rpm_internal(path, Some(window)).await
}

pub async fn install_rpm_internal(path: String, window: Option<Window>) -> Result<String, String> {
    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Initializing RPM installation...");
    }

    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("RPM file does not exist.".into());
    }
    if !is_rpm_path(source_path) {
        return Err("Only .rpm files are supported.".into());
    }

    install_rpm_direct(source_path, window.as_ref())
}

fn is_fuse_available() -> bool {
    if !Path::new("/dev/fuse").exists() {
        return false;
    }

    let ldconfig_check = Command::new("ldconfig").arg("-p").output();

    if let Ok(output) = ldconfig_check {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("libfuse.so.2") {
                return true;
            }
        }
    }

    let common_paths = [
        "/lib/libfuse.so.2",
        "/usr/lib/libfuse.so.2",
        "/lib64/libfuse.so.2",
        "/usr/lib64/libfuse.so.2",
        "/lib/x86_64-linux-gnu/libfuse.so.2",
        "/usr/lib/x86_64-linux-gnu/libfuse.so.2",
        "/lib/aarch64-linux-gnu/libfuse.so.2",
        "/usr/lib/aarch64-linux-gnu/libfuse.so.2",
        "/lib/arm-linux-gnueabihf/libfuse.so.2",
        "/usr/lib/arm-linux-gnueabihf/libfuse.so.2",
        "/lib/i386-linux-gnu/libfuse.so.2",
        "/usr/lib/i386-linux-gnu/libfuse.so.2",
    ];

    for path in &common_paths {
        if Path::new(path).exists() {
            return true;
        }
    }

    false
}

fn find_squashfs_offset(path: &Path) -> Option<u64> {
    let mut file = fs::File::open(path).ok()?;
    let mut buffer = vec![0; 65536];
    let mut offset = 0;

    loop {
        let bytes_read = file.read(&mut buffer).ok()?;
        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read.saturating_sub(3) {
            let chunk = &buffer[i..i + 4];
            if chunk == [0x68, 0x73, 0x71, 0x73] || chunk == [0x73, 0x71, 0x73, 0x68] {
                let absolute_offset = offset + i as u64;

                let status = Command::new("unsquashfs")
                    .arg("-s")
                    .arg("-o")
                    .arg(absolute_offset.to_string())
                    .arg(path)
                    .status();

                if let Ok(s) = status {
                    if s.success() {
                        return Some(absolute_offset);
                    }
                }
            }
        }

        if bytes_read < 4 {
            break;
        }

        offset += (bytes_read - 3) as u64;
        if file.seek(std::io::SeekFrom::Start(offset)).is_err() {
            break;
        }
    }

    None
}

#[tauri::command]
pub async fn install_appimage(path: String, window: Window) -> Result<String, String> {
    install_appimage_internal(path, Some(window)).await
}

pub async fn install_appimage_internal(
    path: String,
    window: Option<Window>,
) -> Result<String, String> {
    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Initializing installation...");
    }

    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("AppImage file does not exist.".into());
    }
    if !is_appimage_path(source_path) {
        return Err("Only .AppImage files are supported.".into());
    }
    ensure_elf_header(source_path)?;

    let file_name = source_path
        .file_name()
        .ok_or("Invalid AppImage file name")?
        .to_string_lossy()
        .to_string();

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let appimages_dir = home_dir.join(".local/appimages");
    let apps_dir = home_dir.join(".local/share/applications");
    let icons_dir = home_dir.join(".local/share/icons");

    fs::create_dir_all(&appimages_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&icons_dir).map_err(|e| e.to_string())?;

    let tmp_dir = std::env::temp_dir().join(format!("linuxy_{}", Uuid::new_v4()));
    fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;

    let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
    let staging_extracted_dir =
        appimages_dir.join(format!(".{}_extracted.{}.part", base_name, Uuid::new_v4()));
    let extracted_dir = appimages_dir.join(format!("{}_extracted", base_name));

    // Determine if we need permanent extraction (FUSE missing or cross-architecture)
    let extraction_binary = tmp_dir.join(&file_name);
    fs::copy(source_path, &extraction_binary).map_err(|e| e.to_string())?;
    let _ = set_executable(&extraction_binary);

    let test_run = Command::new(&extraction_binary)
        .arg("--appimage-version")
        .output();
    let can_run_natively = match test_run {
        Ok(output) => output.status.success(),
        _ => false,
    };

    let fuse_ok = is_fuse_available();
    let use_permanent_extraction = !fuse_ok || !can_run_natively;

    if let Some(ref w) = window {
        if use_permanent_extraction {
            let _ = w.emit(
                "install-progress",
                if !fuse_ok {
                    "FUSE not detected. Installing in FUSE-less (extracted) mode..."
                } else {
                    "Architecture mismatch detected. Installing in emulation/extracted mode..."
                },
            );
        }
    }

    let install_result = (|| -> Result<String, String> {
        let squashfs_root = if use_permanent_extraction {
            staging_extracted_dir.clone()
        } else {
            tmp_dir.join("squashfs-root")
        };

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Extracting metadata (SquashFS)...");
        }

        if can_run_natively {
            let output = Command::new(&extraction_binary)
                .arg("--appimage-extract")
                .current_dir(if use_permanent_extraction {
                    &appimages_dir
                } else {
                    &tmp_dir
                })
                .output()
                .map_err(|e| format!("Failed to extract AppImage: {}", e))?;

            if !output.status.success() {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                return Err(format!("AppImage extraction failed: {}", err_msg.trim()));
            }

            if use_permanent_extraction {
                let default_extracted = appimages_dir.join("squashfs-root");
                if default_extracted.exists() {
                    fs::rename(&default_extracted, &staging_extracted_dir)
                        .map_err(|e| format!("Failed to rename squashfs-root: {}", e))?;
                } else {
                    return Err("Extraction succeeded but squashfs-root not found".into());
                }
            }
        } else {
            let offset = find_squashfs_offset(source_path)
                .ok_or_else(|| "Could not find SquashFS offset in AppImage. Make sure squashfs-tools is installed and the AppImage is valid.".to_string())?;

            let status = Command::new("unsquashfs")
                .arg("-d")
                .arg(&squashfs_root)
                .arg("-o")
                .arg(offset.to_string())
                .arg(source_path)
                .status()
                .map_err(|e| format!("Failed to run unsquashfs: {}", e))?;

            if !status.success() {
                return Err("unsquashfs extraction failed".into());
            }
        }

        let desktop_files = fs::read_dir(&squashfs_root)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().and_then(|ext| ext.to_str()) == Some("desktop")
            })
            .collect::<Vec<_>>();

        if desktop_files.is_empty() {
            return Err("No .desktop file found in AppImage".into());
        }

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Parsing .desktop entries...");
        }

        let desktop_content =
            fs::read_to_string(desktop_files[0].path()).map_err(|e| e.to_string())?;
        let final_app_path = appimages_dir.join(&file_name);
        let staging_app_path =
            appimages_dir.join(format!(".{}.{}.part.AppImage", base_name, Uuid::new_v4()));
        let desktop_path = apps_dir.join(format!("{}.desktop", base_name));

        let exec_path = if use_permanent_extraction {
            extracted_dir.join("AppRun").to_string_lossy().to_string()
        } else {
            final_app_path.to_string_lossy().to_string()
        };

        let mut new_desktop = String::new();
        let mut parsed_icon_name = String::new();
        let mut in_desktop_entry = false;
        let mut has_type = false;
        let mut has_terminal = false;
        let mut has_categories = false;

        for line in desktop_content.lines() {
            let trimmed = line.trim();
            if trimmed == "[Desktop Entry]" {
                in_desktop_entry = true;
                new_desktop.push_str("[Desktop Entry]\n");
                continue;
            } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                in_desktop_entry = false;
            }

            if in_desktop_entry {
                if trimmed.contains('[') && trimmed.contains("]=") {
                    continue;
                }

                if trimmed.starts_with("Exec=") {
                    new_desktop.push_str(&format!("Exec={}\n", exec_path));
                } else if trimmed.starts_with("Icon=") {
                    parsed_icon_name = trimmed.trim_start_matches("Icon=").to_string();
                    new_desktop.push_str(&format!("Icon={}_icon\n", base_name));
                } else if trimmed.starts_with("Type=") {
                    has_type = true;
                    new_desktop.push_str(&format!("{}\n", trimmed));
                } else if trimmed.starts_with("Terminal=") {
                    has_terminal = true;
                    new_desktop.push_str(&format!("{}\n", trimmed));
                } else if trimmed.starts_with("Categories=") {
                    has_categories = true;
                    new_desktop.push_str(&format!("{}\n", trimmed));
                } else if trimmed.starts_with("TryExec=") {
                    continue;
                } else {
                    new_desktop.push_str(&format!("{}\n", line));
                }
            } else {
                new_desktop.push_str(&format!("{}\n", line));
            }
        }

        if !has_type {
            new_desktop =
                new_desktop.replace("[Desktop Entry]\n", "[Desktop Entry]\nType=Application\n");
        }
        if !has_terminal {
            new_desktop =
                new_desktop.replace("[Desktop Entry]\n", "[Desktop Entry]\nTerminal=false\n");
        }
        if !has_categories {
            new_desktop = new_desktop.replace(
                "[Desktop Entry]\n",
                "[Desktop Entry]\nCategories=Utility;\n",
            );
        }

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Copying binary to storage...");
        }

        fs::copy(source_path, &staging_app_path)
            .map_err(|e| format!("Failed to copy file: {}", e))?;
        if let Err(error) = set_executable(&staging_app_path) {
            let _ = fs::remove_file(&staging_app_path);
            return Err(error);
        }

        if let Err(error) = replace_file(&staging_app_path, &final_app_path) {
            let _ = fs::remove_file(&staging_app_path);
            return Err(format!("Failed to finalize installation: {}", error));
        }

        let copied_icons = Vec::new();
        if let Err(error) = fs::write(&desktop_path, new_desktop) {
            cleanup_failed_install(&final_app_path, &desktop_path, &copied_icons);
            return Err(error.to_string());
        }

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Copying system icons...");
        }

        let _copied_icons =
            copy_icon_from_extract(&squashfs_root, &icons_dir, &base_name, &parsed_icon_name);

        if use_permanent_extraction {
            let apprun_path = staging_extracted_dir.join("AppRun");
            if apprun_path.exists() {
                let _ = set_executable(&apprun_path);
            }
            if extracted_dir.exists() {
                fs::remove_dir_all(&extracted_dir).map_err(|e| e.to_string())?;
            }
            fs::rename(&staging_extracted_dir, &extracted_dir)
                .map_err(|e| format!("Failed to finalize extracted directory: {}", e))?;
        }

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Finalizing setup...");
        }

        let _ = Command::new("update-desktop-database")
            .arg(&apps_dir)
            .output();

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Done");
        }

        Ok("Successfully installed AppImage".into())
    })();

    let _ = fs::remove_dir_all(&tmp_dir);
    if install_result.is_err() {
        let _ = fs::remove_dir_all(&staging_extracted_dir);
    }
    install_result
}

#[tauri::command]
pub async fn install_executable(path: String, window: Window) -> Result<String, String> {
    install_executable_internal(path, Some(window)).await
}

pub async fn install_executable_internal(
    path: String,
    window: Option<Window>,
) -> Result<String, String> {
    if let Some(ref w) = window {
        let _ = w.emit(
            "install-progress",
            "Initializing executable installation...",
        );
    }

    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("File does not exist.".into());
    }

    let file_name = source_path
        .file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let bin_dir = home_dir.join(".local/bin");
    let apps_dir = home_dir.join(".local/share/applications");

    fs::create_dir_all(&bin_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;

    let final_app_path = bin_dir.join(&file_name);
    let desktop_path = apps_dir.join(format!("{}.desktop", file_name));

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Copying binary to storage...");
    }

    fs::copy(source_path, &final_app_path).map_err(|e| format!("Failed to copy file: {}", e))?;

    set_executable(&final_app_path)?;

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Creating desktop entry...");
    }

    let new_desktop = format!(
        "[Desktop Entry]\nType=Application\nName={}\nExec={}\nTerminal=false\nCategories=Utility;\n",
        file_name,
        final_app_path.to_string_lossy()
    );

    if let Err(error) = fs::write(&desktop_path, new_desktop) {
        let _ = fs::remove_file(&final_app_path);
        return Err(error.to_string());
    }

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Finalizing setup...");
    }

    let _ = Command::new("update-desktop-database")
        .arg(&apps_dir)
        .output();

    if let Some(ref w) = window {
        let _ = w.emit("install-progress", "Done");
    }

    Ok("Successfully installed executable".into())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    use super::{ensure_elf_header, is_appimage_path};

    #[test]
    fn recognizes_appimage_extensions() {
        assert!(is_appimage_path(Path::new("/tmp/example.AppImage")));
        assert!(is_appimage_path(Path::new("/tmp/example.appimage")));
        assert!(!is_appimage_path(Path::new("/tmp/example.tar.gz")));
    }

    #[test]
    fn rejects_non_elf_files() {
        let test_path = std::env::temp_dir().join(format!("linuxy_test_{}", uuid::Uuid::new_v4()));
        let mut file = fs::File::create(&test_path).expect("create test file");
        file.write_all(b"not-elf").expect("write test file");

        let result = ensure_elf_header(&test_path);
        let _ = fs::remove_file(&test_path);

        assert!(result.is_err());
    }
}
