use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub path: String,
    pub desktop_path: String,
    pub sandboxed: bool,
    pub size_bytes: u64,
    pub installed_at: u64,
    pub categories: Vec<String>,
    pub package_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageStats {
    pub total_size_bytes: u64,
    pub app_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CleanupStats {
    pub orphaned_icons: u32,
    pub orphaned_desktops: u32,
    pub temp_files: u32,
    pub reclaimable_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryBackup {
    pub version: String,
    pub exported_at: u64,
    pub apps: Vec<AppInfo>,
}

// ── Linux helpers ──
#[cfg(target_os = "linux")]
fn get_appimage_base_names(appimages_dir: &Path, bin_dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if appimages_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(appimages_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    names.push(name);
                }
            }
        }
    }
    if bin_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(bin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    names.push(name);
                }
            }
        }
    }
    names
}

#[cfg(target_os = "linux")]
fn read_desktop_file(desktop_path: &Path) -> Option<(String, Option<String>, Vec<String>, bool)> {
    // Returns (name, icon_path, categories, sandboxed)
    if !desktop_path.exists() {
        return None;
    }
    let content = std::fs::read_to_string(desktop_path).ok()?;
    let mut name = String::new();
    let mut icon_name = String::new();
    let mut sandboxed = false;
    let mut categories = Vec::new();
    let mut in_main_section = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[Desktop Entry]" {
            in_main_section = true;
            continue;
        } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_main_section = false;
            continue;
        }

        if in_main_section {
            if trimmed.starts_with("Name=") && name.is_empty() {
                name = trimmed.trim_start_matches("Name=").to_string();
            } else if trimmed.starts_with("Exec=") {
                if trimmed.contains("firejail") {
                    sandboxed = true;
                }
            } else if trimmed.starts_with("Icon=") && icon_name.is_empty() {
                icon_name = trimmed.trim_start_matches("Icon=").to_string();
            } else if trimmed.starts_with("Categories=") {
                let cats = trimmed.trim_start_matches("Categories=");
                categories = cats
                    .split(';')
                    .filter(|c| !c.is_empty())
                    .map(|c| c.to_string())
                    .collect();
            }
        }
    }

    let home_dir = dirs::home_dir()?;
    let icons_dir = home_dir.join(".local/share/icons");
    let icon_path = if icon_name.is_empty() {
        None
    } else {
        let icon_path_buf = Path::new(&icon_name);
        if icon_path_buf.is_absolute() && icon_path_buf.exists() {
            Some(icon_path_buf.to_string_lossy().to_string())
        } else {
            let png = icons_dir.join(format!("{}.png", icon_name));
            let svg = icons_dir.join(format!("{}.svg", icon_name));
            if png.exists() {
                Some(png.to_string_lossy().to_string())
            } else if svg.exists() {
                Some(svg.to_string_lossy().to_string())
            } else {
                None
            }
        }
    };

    if name.is_empty() {
        return None;
    }

    Some((name, icon_path, categories, sandboxed))
}

// ── Windows helpers ──
#[cfg(target_os = "windows")]
fn get_apps_dir() -> Option<std::path::PathBuf> {
    let app_data = std::env::var("LOCALAPPDATA")
        .or_else(|_| std::env::var("APPDATA"))
        .ok()?;
    Some(Path::new(&app_data).join("Linuxy").join("apps"))
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
fn get_apps_dir() -> Option<std::path::PathBuf> {
    Some(dirs::home_dir()?.join(".local/appimages"))
}

#[cfg(target_os = "macos")]
#[allow(dead_code)]
fn get_apps_dir() -> Option<std::path::PathBuf> {
    Some(dirs::home_dir()?.join("Applications").join("Linuxy"))
}

// ── Commands ──

#[tauri::command]
pub async fn get_installed_apps() -> Result<Vec<AppInfo>, String> {
    let Some(home_dir) = dirs::home_dir() else {
        return Err("Could not find home directory".into());
    };

    let mut apps = Vec::new();

    #[cfg(target_os = "linux")]
    {
        let appimages_dir = home_dir.join(".local/appimages");
        let bin_dir = home_dir.join(".local/bin");
        let apps_dir = home_dir.join(".local/share/applications");
        let _icons_dir = home_dir.join(".local/share/icons");

        // List AppImages
        if appimages_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&appimages_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let ext = path.extension().and_then(|s| s.to_str());
                    if ext != Some("AppImage") && ext != Some("appimage") {
                        continue;
                    }

                    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
                    let size_bytes = metadata.len();
                    let installed_at = metadata
                        .created()
                        .or_else(|_| metadata.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    let file_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
                    let desktop_path = apps_dir.join(format!("{}.desktop", base_name));

                    let (name, icon_path, categories, sandboxed) = read_desktop_file(&desktop_path)
                        .unwrap_or_else(|| (base_name.clone(), None, Vec::new(), false));

                    apps.push(AppInfo {
                        name,
                        exec: path.to_string_lossy().to_string(),
                        icon: icon_path,
                        path: path.to_string_lossy().to_string(),
                        desktop_path: desktop_path.to_string_lossy().to_string(),
                        sandboxed,
                        size_bytes,
                        installed_at,
                        categories,
                        package_type: "AppImage".to_string(),
                    });
                }
            }
        }

        // List executables with desktop files
        if bin_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&bin_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let file_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let desktop_path = apps_dir.join(format!("{}.desktop", file_name));
                    if !desktop_path.exists() {
                        continue;
                    }

                    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
                    let size_bytes = metadata.len();
                    let installed_at = metadata
                        .created()
                        .or_else(|_| metadata.modified())
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);

                    let (name, icon_path, categories, _sandboxed) =
                        read_desktop_file(&desktop_path)
                            .unwrap_or_else(|| (file_name.clone(), None, Vec::new(), false));

                    apps.push(AppInfo {
                        name,
                        exec: path.to_string_lossy().to_string(),
                        icon: icon_path,
                        path: path.to_string_lossy().to_string(),
                        desktop_path: desktop_path.to_string_lossy().to_string(),
                        sandboxed: false,
                        size_bytes,
                        installed_at,
                        categories,
                        package_type: "Executable".to_string(),
                    });
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                    if ext != "exe" && ext != "msi" && ext != "bat" && ext != "ps1" {
                        continue;
                    }

                    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
                    let file_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();

                    apps.push(AppInfo {
                        name: file_name
                            .trim_end_matches(&[
                                '.', 'e', 'x', 'e', 'm', 's', 'i', 'b', 'a', 't', 'p', 's', '1',
                            ])
                            .to_string(),
                        exec: path.to_string_lossy().to_string(),
                        icon: None,
                        path: path.to_string_lossy().to_string(),
                        desktop_path: String::new(),
                        sandboxed: false,
                        size_bytes: metadata.len(),
                        installed_at: metadata
                            .created()
                            .or_else(|_| metadata.modified())
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                        categories: Vec::new(),
                        package_type: "Executable".to_string(),
                    });
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let metadata = std::fs::metadata(&path).map_err(|e| e.to_string())?;
                    let file_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let is_app = path.extension().and_then(|s| s.to_str()) == Some("app");
                    let is_exec = path.extension().and_then(|s| s.to_str()) == Some("sh");

                    if !path.is_file() && !is_app {
                        continue;
                    }

                    apps.push(AppInfo {
                        name: file_name.trim_end_matches(".app").to_string(),
                        exec: path.to_string_lossy().to_string(),
                        icon: None,
                        path: path.to_string_lossy().to_string(),
                        desktop_path: String::new(),
                        sandboxed: false,
                        size_bytes: metadata.len(),
                        installed_at: 0,
                        categories: Vec::new(),
                        package_type: if is_app {
                            "macOS App".to_string()
                        } else {
                            "Executable".to_string()
                        },
                    });
                }
            }
        }
    }

    Ok(apps)
}

#[tauri::command]
pub async fn launch_app(path: String) -> Result<(), String> {
    let app_path = Path::new(&path);
    if !app_path.exists() {
        return Err("App file does not exist".into());
    }

    #[cfg(target_os = "linux")]
    {
        // Check if sandboxed
        let base_name = app_path
            .file_name()
            .map(|n| {
                n.to_string_lossy()
                    .to_string()
                    .replace(".AppImage", "")
                    .replace(".appimage", "")
            })
            .ok_or("Invalid app path")?;
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let desktop_path = home_dir
            .join(".local/share/applications")
            .join(format!("{}.desktop", base_name));

        let mut use_firejail = false;
        if desktop_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&desktop_path) {
                use_firejail = content
                    .lines()
                    .any(|l| l.starts_with("Exec=") && l.contains("firejail"));
            }
        }

        let is_appimage = path.to_lowercase().contains(".appimage");
        let has_libfuse = if is_appimage {
            std::process::Command::new("ldconfig")
                .arg("-p")
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("libfuse.so.2"))
                .unwrap_or(true)
        } else {
            true
        };

        if use_firejail {
            let firejail_check = std::process::Command::new("which").arg("firejail").output();
            match firejail_check {
                Ok(output) if output.status.success() => {
                    let mut cmd = std::process::Command::new("firejail");
                    if is_appimage {
                        cmd.arg("--appimage");
                    }
                    cmd.arg(&path);
                    if !has_libfuse {
                        cmd.env("APPIMAGE_EXTRACT_AND_RUN", "1");
                    }
                    cmd.spawn()
                        .map_err(|e| format!("Failed to launch with firejail: {}", e))?;
                },
                _ => {
                    return Err(
                        "Firejail is not installed. Please install firejail to run sandboxed apps."
                            .to_string(),
                    )
                },
            }
        } else {
            let mut cmd = std::process::Command::new(&path);
            if is_appimage && !has_libfuse {
                cmd.env("APPIMAGE_EXTRACT_AND_RUN", "1");
            }
            cmd.spawn()
                .map_err(|e| format!("Failed to launch app: {}", e))?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        let ext = app_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        match ext {
            "msi" => {
                std::process::Command::new("msiexec")
                    .args(["/i", app_path.to_string_lossy().as_ref()])
                    .spawn()
                    .map_err(|e| format!("Failed to launch MSI: {}", e))?;
            },
            _ => {
                std::process::Command::new(&path)
                    .spawn()
                    .map_err(|e| format!("Failed to launch app: {}", e))?;
            },
        }
    }

    #[cfg(target_os = "macos")]
    {
        let ext = app_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext == "app" {
            std::process::Command::new("open")
                .arg(&path)
                .spawn()
                .map_err(|e| format!("Failed to launch app: {}", e))?;
        } else {
            std::process::Command::new(&path)
                .spawn()
                .map_err(|e| format!("Failed to launch app: {}", e))?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn remove_app(path: String, desktop_path: Option<String>) -> Result<(), String> {
    let app_path = Path::new(&path);
    if app_path.exists() {
        std::fs::remove_file(app_path).map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        let base_name = app_path
            .file_name()
            .map(|n| {
                n.to_string_lossy()
                    .to_string()
                    .replace(".AppImage", "")
                    .replace(".appimage", "")
            })
            .unwrap_or_default();
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let resolved_desktop_path = desktop_path.unwrap_or_else(|| {
            home_dir
                .join(".local/share/applications")
                .join(format!("{}.desktop", base_name))
                .to_string_lossy()
                .to_string()
        });

        let mut icon_name = None;
        let dp = Path::new(&resolved_desktop_path);
        if dp.exists() {
            if let Ok(content) = std::fs::read_to_string(dp) {
                for line in content.lines() {
                    if line.starts_with("Icon=") {
                        icon_name = Some(line.trim_start_matches("Icon=").trim().to_string());
                        break;
                    }
                }
            }
            std::fs::remove_file(dp).map_err(|e| e.to_string())?;
        }

        let icons_dir = home_dir.join(".local/share/icons");
        if let Some(ref icon_name) = icon_name {
            let icon_path = Path::new(icon_name);
            if icon_path.is_absolute() {
                let _ = std::fs::remove_file(icon_path);
            } else {
                for ext in ["png", "svg", "xpm"] {
                    let _ = std::fs::remove_file(icons_dir.join(format!("{}.{}", icon_name, ext)));
                }
            }
        }
        for ext in ["png", "svg", "xpm"] {
            let _ = std::fs::remove_file(icons_dir.join(format!("{}_icon.{}", base_name, ext)));
        }

        let apps_dir = home_dir.join(".local/share/applications");
        let _ = std::process::Command::new("update-desktop-database")
            .arg(&apps_dir)
            .output();
    }

    #[cfg(target_os = "windows")]
    {
        // Remove start menu shortcut if present
        let file_name = app_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let start_menu = Path::new(&std::env::var("APPDATA").unwrap_or_default())
            .join("Microsoft\\Windows\\Start Menu\\Programs\\Linuxy");
        let shortcut = start_menu.join(format!("{}.lnk", file_name));
        let _ = std::fs::remove_file(&shortcut);
    }

    Ok(())
}

#[tauri::command]
pub async fn open_directory(dir_name: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let path = match dir_name.as_str() {
            "appimages" => home_dir.join(".local/appimages"),
            _ => return Err("Invalid directory".into()),
        };
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        std::process::Command::new("explorer")
            .arg(&apps_dir)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        std::process::Command::new("open")
            .arg(&apps_dir)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_sandbox(desktop_path: String, enable: bool) -> Result<(), String> {
    #[cfg(not(target_os = "linux"))]
    {
        return Err("Sandboxing is only supported on Linux with Firejail".into());
    }

    #[cfg(target_os = "linux")]
    {
        let path = Path::new(&desktop_path);
        if !path.exists() {
            return Err("Desktop file not found".into());
        }

        if enable {
            let check = std::process::Command::new("which")
                .arg("firejail")
                .output()
                .map_err(|_| "Failed to check for firejail".to_string())?;
            if !check.status.success() {
                return Err("Firejail is not installed. Please install firejail first.".to_string());
            }
        }

        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut new_content = String::new();

        for line in content.lines() {
            if line.starts_with("Exec=") {
                let current_exec = line.trim_start_matches("Exec=");
                let has_firejail = current_exec.contains("firejail");

                if enable && !has_firejail {
                    let clean_exec = current_exec.trim().trim_matches('"');
                    if clean_exec.to_lowercase().contains(".appimage") {
                        new_content
                            .push_str(&format!("Exec=firejail --appimage \"{}\"\n", clean_exec));
                    } else {
                        new_content.push_str(&format!("Exec=firejail \"{}\"\n", clean_exec));
                    }
                } else if !enable && has_firejail {
                    let clean_exec = current_exec
                        .replace("firejail --appimage ", "")
                        .replace("firejail ", "")
                        .trim()
                        .trim_matches('"')
                        .to_string();
                    new_content.push_str(&format!("Exec=\"{}\"\n", clean_exec));
                } else {
                    new_content.push_str(&format!("{}\n", line));
                }
            } else {
                new_content.push_str(&format!("{}\n", line));
            }
        }

        std::fs::write(path, new_content).map_err(|e| e.to_string())?;

        if let Some(home_dir) = dirs::home_dir() {
            let apps_dir = home_dir.join(".local/share/applications");
            let _ = std::process::Command::new("update-desktop-database")
                .arg(&apps_dir)
                .output();
            let _ = filetime::set_file_mtime(&apps_dir, filetime::FileTime::now());
        }

        Ok(())
    }
}

#[tauri::command]
pub async fn open_url(url: String) -> Result<(), String> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("Invalid URL scheme".into());
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open url: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", "", &url])
            .spawn()
            .map_err(|e| format!("Failed to open url: {}", e))?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open url: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn is_firejail_installed() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        let output = std::process::Command::new("which")
            .arg("firejail")
            .output()
            .map_err(|e| format!("Failed to check for firejail: {}", e))?;
        Ok(output.status.success())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_storage_stats() -> Result<StorageStats, String> {
    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let appimages_dir = home_dir.join(".local/appimages");
        let bin_dir = home_dir.join(".local/bin");
        let apps_dir = home_dir.join(".local/share/applications");

        let mut total_size_bytes = 0u64;
        let mut app_count = 0u32;

        if appimages_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&appimages_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let ext = path.extension().and_then(|s| s.to_str());
                    if ext == Some("AppImage") || ext == Some("appimage") {
                        if let Ok(meta) = std::fs::metadata(&path) {
                            total_size_bytes += meta.len();
                            app_count += 1;
                        }
                    }
                }
            }
        }

        if bin_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&bin_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let file_name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let desktop_path = apps_dir.join(format!("{}.desktop", file_name));
                    if desktop_path.exists() {
                        if let Ok(meta) = std::fs::metadata(&path) {
                            total_size_bytes += meta.len();
                            app_count += 1;
                        }
                    }
                }
            }
        }

        Ok(StorageStats {
            total_size_bytes,
            app_count,
        })
    }

    #[cfg(target_os = "windows")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        let mut total_size_bytes = 0u64;
        let mut app_count = 0u32;
        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(meta) = std::fs::metadata(&path) {
                            total_size_bytes += meta.len();
                            app_count += 1;
                        }
                    }
                }
            }
        }
        Ok(StorageStats {
            total_size_bytes,
            app_count,
        })
    }

    #[cfg(target_os = "macos")]
    {
        let apps_dir = get_apps_dir().ok_or("Could not find apps directory")?;
        let mut total_size_bytes = 0u64;
        let mut app_count = 0u32;
        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() || path.extension().and_then(|s| s.to_str()) == Some("app") {
                        let meta = std::fs::metadata(&path).ok();
                        if let Some(m) = meta {
                            total_size_bytes += m.len();
                            app_count += 1;
                        }
                    }
                }
            }
        }
        Ok(StorageStats {
            total_size_bytes,
            app_count,
        })
    }
}

#[tauri::command]
pub async fn analyze_storage() -> Result<CleanupStats, String> {
    #[cfg(not(target_os = "linux"))]
    {
        Ok(CleanupStats {
            orphaned_icons: 0,
            orphaned_desktops: 0,
            temp_files: 0,
            reclaimable_bytes: 0,
        })
    }

    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let appimages_dir = home_dir.join(".local/appimages");
        let bin_dir = home_dir.join(".local/bin");
        let apps_dir = home_dir.join(".local/share/applications");
        let icons_dir = home_dir.join(".local/share/icons");

        let app_names = get_appimage_base_names(&appimages_dir, &bin_dir);
        let mut orphaned_icons = 0;
        let mut orphaned_desktops = 0;
        let mut reclaimable_bytes = 0;

        if icons_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&icons_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if !name.ends_with("_icon") {
                            continue;
                        }
                        let base_name = name.replace("_icon", "");
                        if !app_names.contains(&base_name) {
                            orphaned_icons += 1;
                            if let Ok(meta) = std::fs::metadata(&path) {
                                reclaimable_bytes += meta.len();
                            }
                        }
                    }
                }
            }
        }

        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path.extension().and_then(|s| s.to_str()) == Some("desktop")
                    {
                        let content = std::fs::read_to_string(&path).unwrap_or_default();
                        if !content.contains("X-Linuxy=true") {
                            continue;
                        }
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if !app_names.contains(&name) {
                            orphaned_desktops += 1;
                            if let Ok(meta) = std::fs::metadata(&path) {
                                reclaimable_bytes += meta.len();
                            }
                        }
                    }
                }
            }
        }

        let tmp_dir = std::env::temp_dir();
        let mut temp_files = 0;
        if tmp_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&tmp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if name.starts_with("linuxy_") {
                            temp_files += 1;
                            if let Ok(meta) = std::fs::metadata(&path) {
                                reclaimable_bytes += meta.len();
                            }
                        }
                    }
                }
            }
        }

        Ok(CleanupStats {
            orphaned_icons,
            orphaned_desktops,
            temp_files,
            reclaimable_bytes,
        })
    }
}

#[tauri::command]
pub async fn cleanup_storage() -> Result<CleanupStats, String> {
    #[cfg(not(target_os = "linux"))]
    {
        Ok(CleanupStats {
            orphaned_icons: 0,
            orphaned_desktops: 0,
            temp_files: 0,
            reclaimable_bytes: 0,
        })
    }

    #[cfg(target_os = "linux")]
    {
        let stats = analyze_storage().await?;
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let appimages_dir = home_dir.join(".local/appimages");
        let bin_dir = home_dir.join(".local/bin");
        let apps_dir = home_dir.join(".local/share/applications");
        let icons_dir = home_dir.join(".local/share/icons");

        let app_names = get_appimage_base_names(&appimages_dir, &bin_dir);

        if icons_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&icons_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if !name.ends_with("_icon") {
                            continue;
                        }
                        let base_name = name.replace("_icon", "");
                        if !app_names.contains(&base_name) {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        if apps_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&apps_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file()
                        && path.extension().and_then(|s| s.to_str()) == Some("desktop")
                    {
                        let content = std::fs::read_to_string(&path).unwrap_or_default();
                        if !content.contains("X-Linuxy=true") {
                            continue;
                        }
                        let name = path
                            .file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if !app_names.contains(&name) {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        let tmp_dir = std::env::temp_dir();
        if tmp_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&tmp_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        if name.starts_with("linuxy_") {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        Ok(stats)
    }
}

#[tauri::command]
pub async fn export_library(backup_path: String) -> Result<String, String> {
    let apps = get_installed_apps().await?;
    let backup = LibraryBackup {
        version: "2.0".to_string(),
        exported_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
        apps,
    };
    let json = serde_json::to_string_pretty(&backup).map_err(|e| e.to_string())?;
    std::fs::write(&backup_path, json).map_err(|e| e.to_string())?;
    Ok(format!("Library exported to {}", backup_path))
}

#[tauri::command]
pub async fn import_library(backup_path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&backup_path).map_err(|e| e.to_string())?;
    let backup: LibraryBackup = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    #[cfg(target_os = "linux")]
    {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let apps_dir = home_dir.join(".local/share/applications");
        std::fs::create_dir_all(&apps_dir).map_err(|e| e.to_string())?;

        let mut restored = 0;
        for app in &backup.apps {
            if std::path::Path::new(&app.path).exists() {
                if app.name.contains('\n')
                    || app.exec.contains('\n')
                    || app.icon.as_deref().map(|i| i.contains('\n')).unwrap_or(false)
                    || app.name.contains('/') || app.name.contains('\\')
                {
                    continue;
                }
                let file_stem = std::path::Path::new(&app.desktop_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                if file_stem.is_empty() || file_stem.contains('\n') || file_stem.contains('/') || file_stem.contains('\\') {
                    continue;
                }
                let desktop_path = apps_dir.join(file_stem);
                let desktop_content = format!(
                    "[Desktop Entry]\nType=Application\nName={}\nExec={}\nIcon={}\nTerminal=false\nCategories={}\nX-Linuxy=true\n",
                    app.name,
                    if app.sandboxed { format!("firejail --appimage \"{}\"", app.exec) } else { format!("\"{}\"", app.exec) },
                    app.icon.as_deref().unwrap_or(""),
                    app.categories.join(";"),
                );
                if std::fs::write(&desktop_path, desktop_content).is_ok() {
                    restored += 1;
                }
            }
        }

        let _ = std::process::Command::new("update-desktop-database")
            .arg(&apps_dir)
            .output();
        let _ = filetime::set_file_mtime(&apps_dir, filetime::FileTime::now());
        Ok(format!("Restored {} apps from backup", restored))
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(format!(
            "Restored {} apps from backup (metadata only)",
            backup.apps.len()
        ))
    }
}
#[tauri::command]
pub async fn get_app_checksum(path: String) -> Result<String, String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Err("File does not exist".into());
    }
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut file = std::fs::File::open(p).map_err(|e| e.to_string())?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[tauri::command]
pub async fn recreate_desktop_entry(path: String) -> Result<String, String> {
    let app_path = Path::new(&path);
    if !app_path.exists() {
        return Err("Application file does not exist".into());
    }

    #[cfg(target_os = "linux")]
    {
        let file_name = app_path
            .file_name()
            .ok_or("Invalid file name")?
            .to_string_lossy()
            .to_string();
        let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let apps_dir = home_dir.join(".local/share/applications");
        let icons_dir = home_dir.join(".local/share/icons");
        let desktop_path = apps_dir.join(format!("{}.desktop", base_name));

        let icon_png = icons_dir.join(format!("{}_icon.png", base_name));
        let icon_svg = icons_dir.join(format!("{}_icon.svg", base_name));
        let icon_str = if icon_png.exists() {
            icon_png.to_string_lossy().to_string()
        } else if icon_svg.exists() {
            icon_svg.to_string_lossy().to_string()
        } else {
            format!("{}_icon", base_name)
        };

        let has_libfuse = std::process::Command::new("ldconfig")
            .arg("-p")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains("libfuse.so.2"))
            .unwrap_or(true);

        let exec_str = if !has_libfuse
            && app_path
                .to_string_lossy()
                .to_lowercase()
                .contains(".appimage")
        {
            format!(
                "env APPIMAGE_EXTRACT_AND_RUN=1 \"{}\" %U",
                app_path.to_string_lossy()
            )
        } else {
            format!("\"{}\" %U", app_path.to_string_lossy())
        };

        let desktop_content = format!(
            "[Desktop Entry]\nType=Application\nName={}\nExec={}\nIcon={}\nTerminal=false\nCategories=Utility;\nX-Linuxy=true\n",
            base_name,
            exec_str,
            icon_str
        );

        std::fs::write(&desktop_path, desktop_content).map_err(|e| e.to_string())?;

        let _ = std::process::Command::new("update-desktop-database")
            .arg(&apps_dir)
            .output();
        let _ = filetime::set_file_mtime(&apps_dir, filetime::FileTime::now());

        Ok(format!(
            "Desktop launcher recreated at {}",
            desktop_path.to_string_lossy()
        ))
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok("Recreating desktop entry is only supported on Linux".into())
    }
}
