use std::fs;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use tauri::Window;
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
        let possible_icons = vec![
            squashfs_root.join(format!("{}.png", parsed_icon_name)),
            squashfs_root.join(format!("{}.svg", parsed_icon_name)),
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

    let install_result = (|| -> Result<String, String> {
        let extraction_binary = tmp_dir.join(&file_name);
        fs::copy(source_path, &extraction_binary).map_err(|e| e.to_string())?;
        set_executable(&extraction_binary)?;

        if let Some(ref w) = window {
            let _ = w.emit("install-progress", "Extracting metadata (SquashFS)...");
        }

        let output = Command::new(&extraction_binary)
            .arg("--appimage-extract")
            .current_dir(&tmp_dir)
            .output()
            .map_err(|e| format!("Failed to extract AppImage: {}", e))?;

        if !output.status.success() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(format!("AppImage extraction failed: {}", err_msg.trim()));
        }

        let squashfs_root = tmp_dir.join("squashfs-root");
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
        let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
        let final_app_path = appimages_dir.join(&file_name);
        let staging_app_path =
            appimages_dir.join(format!(".{}.{}.part.AppImage", base_name, Uuid::new_v4()));
        let desktop_path = apps_dir.join(format!("{}.desktop", base_name));

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
                    new_desktop.push_str(&format!("Exec={}\n", final_app_path.to_string_lossy()));
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
    install_result
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
