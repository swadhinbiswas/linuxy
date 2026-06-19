use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, Window};
use tauri_plugin_shell::ShellExt;

const UPDATE_TOOL: &str = "appimageupdatetool";

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateInfo {
    pub path: String,
    pub name: String,
    pub has_update: bool,
    pub error: Option<String>,
}

fn update_check_indicates_available(code: Option<i32>, stdout: &str, stderr: &str) -> bool {
    let stdout_lower = stdout.to_ascii_lowercase();
    let stderr_lower = stderr.to_ascii_lowercase();

    code == Some(1)
        || stdout_lower.contains("update is available")
        || stderr_lower.contains("update is available")
}

#[tauri::command]
pub async fn is_update_tool_installed(app_handle: AppHandle) -> Result<bool, String> {
    let cmd = app_handle.shell().sidecar(UPDATE_TOOL);
    match cmd {
        Ok(c) => {
            let output = c.args(["--help"]).output().await;
            Ok(output.is_ok())
        },
        _ => Ok(false),
    }
}

#[tauri::command]
pub async fn check_for_update(path: String, window: Window) -> Result<bool, String> {
    let _ = window.emit("install-progress", "Checking for updates...");

    let app_handle = window.app_handle();
    let cmd = app_handle
        .shell()
        .sidecar(UPDATE_TOOL)
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?;

    let output = cmd
        .args(["--check-for-update", &path])
        .output()
        .await
        .map_err(|e| format!("Failed to run sidecar: {}", e))?;

    let _ = window.emit("install-progress", "Done");

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || output.status.code() == Some(1) {
        Ok(update_check_indicates_available(
            output.status.code(),
            &stdout_str,
            &stderr_str,
        ))
    } else {
        Err(format!("Update check failed: {}", stderr_str.trim()))
    }
}

#[tauri::command]
pub async fn apply_update(path: String, window: Window) -> Result<String, String> {
    let _ = window.emit("install-progress", "Downloading delta update...");

    let app_handle = window.app_handle();
    let cmd = app_handle
        .shell()
        .sidecar(UPDATE_TOOL)
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?;

    let output = cmd
        .args([&path])
        .output()
        .await
        .map_err(|e| format!("Failed to run sidecar: {}", e))?;

    let stderr_str = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        let _ = window.emit("install-progress", "Update applied successfully.");
        Ok("Update applied successfully.".into())
    } else {
        let _ = window.emit("install-progress", "Update failed.");
        Err(format!("Update failed: {}", stderr_str.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::update_check_indicates_available;

    #[test]
    fn detects_update_from_exit_code() {
        assert!(update_check_indicates_available(Some(1), "", ""));
    }

    #[test]
    fn detects_update_from_output_text() {
        assert!(update_check_indicates_available(
            Some(0),
            "An update is available",
            ""
        ));
    }

    #[test]
    fn reports_no_update_when_signal_absent() {
        assert!(!update_check_indicates_available(
            Some(0),
            "already up to date",
            ""
        ));
    }
}

#[tauri::command]
pub async fn check_all_updates(app_handle: AppHandle) -> Result<Vec<UpdateInfo>, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let appimages_dir = home_dir.join(".local/appimages");
    let apps_dir = home_dir.join(".local/share/applications");

    if !appimages_dir.exists() {
        return Ok(Vec::new());
    }

    let entries = std::fs::read_dir(&appimages_dir).map_err(|e| e.to_string())?;
    let mut results = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|s| s.to_str());
        if ext != Some("AppImage") && ext != Some("appimage") {
            continue;
        }

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let base_name = file_name.replace(".AppImage", "").replace(".appimage", "");
        let desktop_path = apps_dir.join(format!("{}.desktop", base_name));

        let mut name = base_name.clone();
        if desktop_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&desktop_path) {
                for line in content.lines() {
                    if line.starts_with("Name=") {
                        name = line.trim_start_matches("Name=").to_string();
                        break;
                    }
                }
            }
        }

        let path_str = path.to_string_lossy().to_string();
        let cmd_result = app_handle
            .shell()
            .sidecar(UPDATE_TOOL)
            .map_err(|e| e.to_string());

        let output_result = match cmd_result {
            Ok(cmd) => cmd
                .args(["--check-for-update", &path_str])
                .output()
                .await
                .map_err(|e| e.to_string()),
            Err(e) => Err(e),
        };

        match output_result {
            Ok(out) => {
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                let stderr_str = String::from_utf8_lossy(&out.stderr);
                let has_update =
                    update_check_indicates_available(out.status.code(), &stdout_str, &stderr_str);
                results.push(UpdateInfo {
                    path: path_str,
                    name,
                    has_update,
                    error: if out.status.success()
                        || out.status.code() == Some(1)
                        || out.status.code() == Some(0)
                    {
                        None
                    } else {
                        Some(stderr_str.trim().to_string())
                    },
                });
            },
            Err(e) => {
                results.push(UpdateInfo {
                    path: path_str,
                    name,
                    has_update: false,
                    error: Some(e),
                });
            },
        }
    }

    Ok(results)
}
