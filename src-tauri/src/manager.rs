use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
    pub path: String,
    pub desktop_path: String,
    pub sandboxed: bool,
    pub size_bytes: u64,
    pub installed_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageStats {
    pub total_size_bytes: u64,
    pub app_count: u32,
}

#[tauri::command]
pub async fn get_installed_apps() -> Result<Vec<AppInfo>, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let appimages_dir = home_dir.join(".local/appimages");
    let apps_dir = home_dir.join(".local/share/applications");
    let icons_dir = home_dir.join(".local/share/icons");

    let mut apps = Vec::new();

    if !appimages_dir.exists() {
        return Ok(apps);
    }

    let entries = fs::read_dir(&appimages_dir).map_err(|e| e.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && (path.extension().and_then(|s| s.to_str()) == Some("AppImage")
                || path.extension().and_then(|s| s.to_str()) == Some("appimage"))
        {
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
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

            let mut name = base_name.clone();
            let mut icon_path = None;
            let mut sandboxed = false;

            if desktop_path.exists() {
                if let Ok(content) = fs::read_to_string(&desktop_path) {
                    for line in content.lines() {
                        if line.starts_with("Name=") {
                            name = line.trim_start_matches("Name=").to_string();
                        } else if line.starts_with("Exec=") {
                            if line.contains("firejail") {
                                sandboxed = true;
                            }
                        } else if line.starts_with("Icon=") {
                            let icon_name = line.trim_start_matches("Icon=").to_string();
                            // look for icon in icons_dir
                            let possible_png = icons_dir.join(format!("{}.png", icon_name));
                            let possible_svg = icons_dir.join(format!("{}.svg", icon_name));
                            if possible_png.exists() {
                                icon_path = Some(possible_png.to_string_lossy().to_string());
                            } else if possible_svg.exists() {
                                icon_path = Some(possible_svg.to_string_lossy().to_string());
                            } else {
                                // fallback if icon is an absolute path
                                let abs_path = Path::new(&icon_name);
                                if abs_path.exists() {
                                    icon_path = Some(abs_path.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }

            apps.push(AppInfo {
                name,
                exec: path.to_string_lossy().to_string(),
                icon: icon_path,
                path: path.to_string_lossy().to_string(),
                desktop_path: desktop_path.to_string_lossy().to_string(),
                sandboxed,
                size_bytes,
                installed_at,
            });
        }
    }

    Ok(apps)
}

#[tauri::command]
pub async fn get_storage_stats() -> Result<StorageStats, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let appimages_dir = home_dir.join(".local/appimages");

    let mut total_size_bytes = 0;
    let mut app_count = 0;

    if appimages_dir.exists() {
        let entries = fs::read_dir(&appimages_dir).map_err(|e| e.to_string())?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && (path.extension().and_then(|s| s.to_str()) == Some("AppImage")
                    || path.extension().and_then(|s| s.to_str()) == Some("appimage"))
            {
                if let Ok(metadata) = fs::metadata(&path) {
                    total_size_bytes += metadata.len();
                    app_count += 1;
                }
            }
        }
    }

    Ok(StorageStats {
        total_size_bytes,
        app_count,
    })
}

#[tauri::command]
pub async fn launch_app(path: String) -> Result<(), String> {
    // Check if the corresponding desktop file has firejail
    let base_name = Path::new(&path)
        .file_name()
        .map(|n| {
            n.to_string_lossy()
                .to_string()
                .replace(".AppImage", "")
                .replace(".appimage", "")
        })
        .ok_or("Invalid AppImage path")?;
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let desktop_path = home_dir
        .join(".local/share/applications")
        .join(format!("{}.desktop", base_name));

    let mut use_firejail = false;
    if desktop_path.exists() {
        if let Ok(content) = fs::read_to_string(&desktop_path) {
            for line in content.lines() {
                if line.starts_with("Exec=") && line.contains("firejail") {
                    use_firejail = true;
                }
            }
        }
    }

    if use_firejail {
        // Check if firejail is installed
        let firejail_check = Command::new("which").arg("firejail").output();
        match firejail_check {
            Ok(output) if output.status.success() => {
                Command::new("firejail")
                    .arg("--appimage")
                    .arg(&path)
                    .spawn()
                    .map_err(|e| format!("Failed to launch app with firejail: {}", e))?;
            },
            _ => {
                return Err(
                    "Firejail is not installed. Please install firejail to run sandboxed apps."
                        .to_string(),
                );
            },
        }
    } else {
        Command::new(&path)
            .spawn()
            .map_err(|e| format!("Failed to launch app: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_sandbox(desktop_path: String, enable: bool) -> Result<(), String> {
    let path = Path::new(&desktop_path);
    if !path.exists() {
        return Err("Desktop file not found".into());
    }

    // Check if firejail is installed when trying to enable sandbox
    if enable {
        let firejail_check = Command::new("which").arg("firejail").output();
        match firejail_check {
            Ok(output) if output.status.success() => {},
            _ => {
                return Err("Firejail is not installed. Please install firejail first (e.g., sudo apt install firejail).".to_string());
            },
        }
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut new_content = String::new();

    for line in content.lines() {
        if line.starts_with("Exec=") {
            let current_exec = line.trim_start_matches("Exec=");
            let has_firejail = current_exec.contains("firejail");

            if enable && !has_firejail {
                new_content.push_str(&format!(
                    "Exec=firejail --appimage {}\n",
                    current_exec.replace("\"", "")
                ));
            } else if !enable && has_firejail {
                let clean_exec = current_exec.replace("firejail --appimage ", "");
                new_content.push_str(&format!("Exec={}\n", clean_exec));
            } else {
                new_content.push_str(&format!("{}\n", line));
            }
        } else {
            new_content.push_str(&format!("{}\n", line));
        }
    }

    fs::write(path, new_content).map_err(|e| e.to_string())?;

    // Attempt to update desktop database
    if let Some(home_dir) = dirs::home_dir() {
        let apps_dir = home_dir.join(".local/share/applications");
        let _ = Command::new("update-desktop-database")
            .arg(&apps_dir)
            .output();
    }

    Ok(())
}

#[tauri::command]
pub async fn remove_app(path: String, desktop_path: Option<String>) -> Result<(), String> {
    let app_path = Path::new(&path);
    let file_name = app_path
        .file_name()
        .ok_or("Invalid AppImage path")?
        .to_string_lossy()
        .to_string();
    let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
    if Path::new(&path).exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let resolved_desktop_path = desktop_path.unwrap_or_else(|| {
        home_dir
            .join(".local/share/applications")
            .join(format!("{}.desktop", base_name))
            .to_string_lossy()
            .to_string()
    });

    let mut icon_name = None;
    if Path::new(&resolved_desktop_path).exists() {
        if let Ok(content) = fs::read_to_string(&resolved_desktop_path) {
            for line in content.lines() {
                if line.starts_with("Icon=") {
                    icon_name = Some(line.trim_start_matches("Icon=").trim().to_string());
                    break;
                }
            }
        }

        fs::remove_file(&resolved_desktop_path).map_err(|e| e.to_string())?;
    }

    let icons_dir = home_dir.join(".local/share/icons");

    if let Some(icon_name) = icon_name {
        let icon_path = Path::new(&icon_name);
        if icon_path.is_absolute() {
            let _ = fs::remove_file(icon_path);
        } else {
            let icon_extensions = ["png", "svg", "xpm"];
            for extension in icon_extensions {
                let _ = fs::remove_file(icons_dir.join(format!("{}.{}", icon_name, extension)));
            }
        }
    }

    let _ = fs::remove_file(icons_dir.join(format!("{}_icon.png", base_name)));
    let _ = fs::remove_file(icons_dir.join(format!("{}_icon.svg", base_name)));
    let _ = fs::remove_file(icons_dir.join(format!("{}_icon.xpm", base_name)));

    let apps_dir = home_dir.join(".local/share/applications");
    let _ = Command::new("update-desktop-database")
        .arg(&apps_dir)
        .output();

    Ok(())
}

#[tauri::command]
pub async fn open_directory(dir_name: String) -> Result<(), String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let path = match dir_name.as_str() {
        "appimages" => home_dir.join(".local/appimages"),
        _ => return Err("Invalid directory".into()),
    };

    Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("Failed to open directory: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn open_url(url: String) -> Result<(), String> {
    // Only allow http and https schemas for security
    if url.starts_with("http://") || url.starts_with("https://") {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open url: {}", e))?;
        Ok(())
    } else {
        Err("Invalid URL scheme".into())
    }
}

#[tauri::command]
pub async fn is_firejail_installed() -> Result<bool, String> {
    let output = Command::new("which")
        .arg("firejail")
        .output()
        .map_err(|e| format!("Failed to check for firejail: {}", e))?;

    Ok(output.status.success())
}
