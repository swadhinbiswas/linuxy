use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::io::Read;
#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
use std::process::Command;

use tauri::Emitter;
use uuid::Uuid;

#[cfg(target_os = "linux")]
fn is_appimage_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("AppImage") | Some("appimage")
    )
}

#[cfg(target_os = "linux")]
fn is_deb_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("deb") | Some("DEB")
    )
}

#[cfg(target_os = "linux")]
fn is_rpm_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rpm") | Some("RPM")
    )
}

#[allow(dead_code)]
fn is_exe_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("exe") | Some("EXE") | Some("msi") | Some("MSI")
    )
}

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
fn set_executable(path: &Path) -> Result<(), String> {
    let mut perms = fs::metadata(path).map_err(|e| e.to_string())?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).map_err(|e| e.to_string())
}

fn emit_progress(app: Option<&tauri::AppHandle>, message: &str) {
    if let Some(handle) = app {
        let _ = handle.emit("install-progress", message);
    }
}

#[cfg(target_os = "linux")]
fn replace_file(from: &Path, to: &Path) -> Result<(), String> {
    if let Err(error) = std::fs::rename(from, to) {
        if to.exists() {
            std::fs::remove_file(to).map_err(|e| e.to_string())?;
            std::fs::rename(from, to).map_err(|e| e.to_string())?;
        } else {
            return Err(error.to_string());
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
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
                if std::fs::copy(&source, &dest).is_ok() {
                    copied_icons.push(dest);
                    return copied_icons;
                }
            }
        }
    }
    if let Ok(entries) = std::fs::read_dir(squashfs_root) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|ext| ext.to_str()) == Some("png") {
                let dest = icons_dir.join(format!("{}_icon.png", base_name));
                if std::fs::copy(entry.path(), &dest).is_ok() {
                    copied_icons.push(dest);
                }
                break;
            }
        }
    }
    copied_icons
}

#[cfg(target_os = "linux")]
fn cleanup_failed_install(app_path: &Path, desktop_path: &Path, icon_paths: &[PathBuf]) {
    let _ = std::fs::remove_file(app_path);
    let _ = std::fs::remove_file(desktop_path);
    for icon_path in icon_paths {
        let _ = std::fs::remove_file(icon_path);
    }
}

#[cfg(target_os = "linux")]
fn detect_package_manager() -> Option<(&'static str, &'static str)> {
    let managers: &[(&str, &str)] = &[
        ("pacman", "sudo pacman -U"),
        ("apt", "sudo apt install"),
        ("dpkg", "sudo dpkg -i"),
        ("dnf", "sudo dnf install"),
        ("yum", "sudo yum localinstall"),
        ("zypper", "sudo zypper install"),
    ];
    for (cmd, _) in managers {
        if Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some((cmd, managers.iter().find(|(c, _)| c == cmd).map(|(_, i)| *i).unwrap()));
        }
    }
    None
}

// ── DEB Install (Linux-only) ──
#[cfg(target_os = "linux")]
fn install_deb_direct(path: &Path, app: Option<&tauri::AppHandle>) -> Result<String, String> {
    let (manager, _) =
        detect_package_manager().ok_or("No supported package manager found".to_string())?;
    emit_progress(app, &format!("Installing DEB package with {}...", manager));

    let install_status = match manager {
        "pacman" => return install_deb_with_debtap(path, app),
        "apt" => Command::new("sudo").arg("apt").arg("install").arg("-y").arg(path).status(),
        "dpkg" => {
            let s = Command::new("sudo").arg("dpkg").arg("-i").arg(path).status();
            if s.as_ref().map(|s| s.success()).unwrap_or(false) {
                let _ = Command::new("sudo").arg("apt").arg("install").arg("-f").arg("-y").status();
            }
            s
        }
        "dnf" => Command::new("sudo").arg("dnf").arg("install").arg("-y").arg(path).status(),
        "yum" => Command::new("sudo").arg("yum").arg("localinstall").arg("-y").arg(path).status(),
        "zypper" => Command::new("sudo").arg("zypper").arg("install").arg("-y").arg(path).status(),
        _ => return Err(format!("Unsupported package manager: {}", manager)),
    }
    .map_err(|e| format!("Failed to install package: {}", e))?;

    if !install_status.success() {
        return Err("Package installation failed. Check terminal output for details.".to_string());
    }
    emit_progress(app, "Done");
    Ok(format!("Successfully installed DEB package via {}", manager))
}

#[cfg(target_os = "linux")]
fn install_deb_with_debtap(path: &Path, app: Option<&tauri::AppHandle>) -> Result<String, String> {
    emit_progress(app, "Checking debtap availability...");
    let debtap_check = Command::new("which")
        .arg("debtap")
        .output()
        .map_err(|_| "debtap is not installed".to_string())?;
    if !debtap_check.status.success() {
        return Err("debtap is not installed. Please install it first: yay -S debtap".to_string());
    }
    emit_progress(app, "Updating debtap database...");
    let update = Command::new("debtap").arg("-u").output().map_err(|e| format!("debtap -u failed: {}", e))?;
    if !update.status.success() {
        return Err(format!("debtap database update failed: {}", String::from_utf8_lossy(&update.stderr).trim()));
    }
    emit_progress(app, "Converting DEB package...");
    let convert = Command::new("debtap").arg(path).output().map_err(|e| format!("debtap failed: {}", e))?;
    if !convert.status.success() {
        return Err(format!("debtap conversion failed: {}", String::from_utf8_lossy(&convert.stderr).trim()));
    }
    let stdout = String::from_utf8_lossy(&convert.stdout);
    let pkg_path = stdout
        .lines()
        .find(|line| line.contains(".pkg.tar"))
        .map(|line| line.trim().to_string())
        .ok_or("Could not find converted package path".to_string())?;
    let pkg_path = Path::new(&pkg_path);
    if !pkg_path.exists() {
        return Err("Converted package file not found".to_string());
    }
    emit_progress(app, "Installing converted package...");
    let install = Command::new("sudo")
        .arg("pacman")
        .arg("-U")
        .arg("--noconfirm")
        .arg(pkg_path)
        .status()
        .map_err(|e| format!("Failed to install: {}", e))?;
    if !install.success() {
        return Err("Package installation failed".to_string());
    }
    emit_progress(app, "Done");
    Ok("Successfully installed DEB package via debtap".to_string())
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn install_deb(path: String) -> Result<String, String> {
    install_deb_internal(path).await
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub async fn install_deb(_path: String) -> Result<String, String> {
    Err("DEB packages are only supported on Linux".to_string())
}

#[cfg(target_os = "linux")]
pub async fn install_deb_internal(path: String) -> Result<String, String> {
    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("DEB file does not exist.".into());
    }
    if !is_deb_path(source_path) {
        return Err("Only .deb files are supported.".into());
    }
    install_deb_direct(source_path, None)
}

// ── RPM Install (Linux-only) ──
#[cfg(target_os = "linux")]
fn install_rpm_direct(path: &Path, app: Option<&tauri::AppHandle>) -> Result<String, String> {
    let managers: &[(&str, &[&str])] = &[
        ("dnf", &["install", "-y"]),
        ("zypper", &["install", "-y"]),
        ("yum", &["localinstall", "-y"]),
        ("rpm", &["-i", "--nodeps"]),
    ];
    let (manager, args) = managers
        .iter()
        .find(|(cmd, _)| {
            Command::new("which")
                .arg(cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .ok_or("No supported RPM package manager found".to_string())?;

    emit_progress(app, &format!("Installing RPM package with {}...", manager));
    let mut cmd = Command::new("sudo");
    cmd.arg(manager);
    for arg in *args {
        cmd.arg(arg);
    }
    cmd.arg(path);

    let status = cmd.status().map_err(|e| format!("Failed to install: {}", e))?;
    if !status.success() {
        return Err("Package installation failed. Check terminal output for details.".to_string());
    }
    emit_progress(app, "Done");
    Ok(format!("Successfully installed RPM package via {}", manager))
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn install_rpm(path: String) -> Result<String, String> {
    install_rpm_internal(path).await
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub async fn install_rpm(_path: String) -> Result<String, String> {
    Err("RPM packages are only supported on Linux".to_string())
}

#[cfg(target_os = "linux")]
pub async fn install_rpm_internal(path: String) -> Result<String, String> {
    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("RPM file does not exist.".into());
    }
    if !is_rpm_path(source_path) {
        return Err("Only .rpm files are supported.".into());
    }
    install_rpm_direct(source_path, None)
}

// ── AppImage Install (Linux-only) ──
#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn install_appimage(path: String) -> Result<String, String> {
    install_appimage_internal(path).await
}

#[cfg(not(target_os = "linux"))]
#[tauri::command]
pub async fn install_appimage(_path: String) -> Result<String, String> {
    Err("AppImages are only supported on Linux".to_string())
}

#[cfg(target_os = "linux")]
pub async fn install_appimage_internal(path: String) -> Result<String, String> {
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

    std::fs::create_dir_all(&appimages_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&icons_dir).map_err(|e| e.to_string())?;

    let tmp_dir = std::env::temp_dir().join(format!("linuxy_{}", Uuid::new_v4()));
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;

    let install_result = (|| -> Result<String, String> {
        let extraction_binary = tmp_dir.join(&file_name);
        std::fs::copy(source_path, &extraction_binary).map_err(|e| e.to_string())?;
        set_executable(&extraction_binary)?;

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
        let desktop_files = std::fs::read_dir(&squashfs_root)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry.path().extension().and_then(|ext| ext.to_str()) == Some("desktop")
            })
            .collect::<Vec<_>>();

        if desktop_files.is_empty() {
            return Err("No .desktop file found in AppImage".into());
        }

        let desktop_content = std::fs::read_to_string(desktop_files[0].path()).map_err(|e| e.to_string())?;
        let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
        let final_app_path = appimages_dir.join(&file_name);
        let staging_app_path = appimages_dir.join(format!(".{}.{}.part.AppImage", base_name, Uuid::new_v4()));
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
            new_desktop = new_desktop.replace("[Desktop Entry]\n", "[Desktop Entry]\nType=Application\n");
        }
        if !has_terminal {
            new_desktop = new_desktop.replace("[Desktop Entry]\n", "[Desktop Entry]\nTerminal=false\n");
        }
        if !has_categories {
            new_desktop = new_desktop.replace("[Desktop Entry]\n", "[Desktop Entry]\nCategories=Utility;\n");
        }

        std::fs::copy(source_path, &staging_app_path).map_err(|e| format!("Failed to copy file: {}", e))?;
        if let Err(error) = set_executable(&staging_app_path) {
            let _ = std::fs::remove_file(&staging_app_path);
            return Err(error);
        }
        if let Err(error) = replace_file(&staging_app_path, &final_app_path) {
            let _ = std::fs::remove_file(&staging_app_path);
            return Err(format!("Failed to finalize installation: {}", error));
        }

        let copied_icons = Vec::new();
        if let Err(error) = std::fs::write(&desktop_path, new_desktop) {
            cleanup_failed_install(&final_app_path, &desktop_path, &copied_icons);
            return Err(error.to_string());
        }

        let _copied_icons = copy_icon_from_extract(&squashfs_root, &icons_dir, &base_name, &parsed_icon_name);

        let _ = Command::new("update-desktop-database").arg(&apps_dir).output();

        Ok("Successfully installed AppImage".into())
    })();

    let _ = std::fs::remove_dir_all(&tmp_dir);
    install_result
}

// ── Executable Install (Cross-platform) ──
#[tauri::command]
pub async fn install_executable(path: String) -> Result<String, String> {
    install_executable_internal(path).await
}

pub async fn install_executable_internal(path: String) -> Result<String, String> {
    let source_path = Path::new(&path);
    if !source_path.exists() {
        return Err("File does not exist.".into());
    }

    let file_name = source_path
        .file_name()
        .ok_or("Invalid file name")?
        .to_string_lossy()
        .to_string();

    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let bin_dir = home_dir.join(".local/bin");
        let apps_dir = home_dir.join(".local/share/applications");
        std::fs::create_dir_all(&bin_dir).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;

        let final_app_path = bin_dir.join(&file_name);
        let desktop_path = apps_dir.join(format!("{}.desktop", file_name));

        std::fs::copy(source_path, &final_app_path).map_err(|e| format!("Failed to copy file: {}", e))?;
        set_executable(&final_app_path)?;

        let new_desktop = format!(
            "[Desktop Entry]\nType=Application\nName={}\nExec={}\nTerminal=false\nCategories=Utility;\n",
            file_name,
            final_app_path.to_string_lossy()
        );
        if let Err(error) = std::fs::write(&desktop_path, new_desktop) {
            let _ = std::fs::remove_file(&final_app_path);
            return Err(error.to_string());
        }
        let _ = Command::new("update-desktop-database").arg(&apps_dir).output();
        Ok("Successfully installed executable".into())
    }

    #[cfg(target_os = "windows")]
    {
        let app_data = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("APPDATA"))
            .map_err(|_| "Could not find AppData directory".to_string())?;
        let apps_dir = Path::new(&app_data).join("Linuxy").join("apps");
        std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
        let final_app_path = apps_dir.join(&file_name);
        std::fs::copy(source_path, &final_app_path).map_err(|e| format!("Failed to copy file: {}", e))?;

        // Create Start Menu shortcut via PowerShell script
        let ps_script = format!(
            "$WScriptShell = New-Object -ComObject WScript.Shell;\
             $Shortcut = $WScriptShell.CreateShortcut(\"$env:APPDATA\\Microsoft\\Windows\\Start Menu\\Programs\\Linuxy\\{file_name}.lnk\");\
             $Shortcut.TargetPath = \"{target_path}\";\
             $Shortcut.Save()",
            file_name = file_name,
            target_path = final_app_path.to_string_lossy().replace('\\', "\\\\")
        );
        let _ = Command::new("powershell")
            .args(["-Command", &ps_script])
            .output();

        Ok("Successfully installed executable on Windows".into())
    }

    #[cfg(target_os = "macos")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let apps_dir = home_dir.join("Applications").join("Linuxy");
        std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;
        let final_app_path = apps_dir.join(&file_name);
        std::fs::copy(source_path, &final_app_path).map_err(|e| format!("Failed to copy file: {}", e))?;

        let mut perms = std::fs::metadata(&final_app_path)
            .map_err(|e| e.to_string())?
            .permissions();
        std::fs::set_permissions(&final_app_path, perms).map_err(|e| e.to_string())?;

        Ok("Successfully installed executable on macOS".into())
    }
}

#[cfg(test)]
#[cfg(target_os = "linux")]
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
