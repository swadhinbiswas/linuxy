use tauri::api::process::Command;
use tauri::Window;

const UPDATE_TOOL: &str = "appimageupdatetool";

fn update_check_indicates_available(code: Option<i32>, stdout: &str, stderr: &str) -> bool {
    let stdout_lower = stdout.to_ascii_lowercase();
    let stderr_lower = stderr.to_ascii_lowercase();

    code == Some(1)
        || stdout_lower.contains("update is available")
        || stderr_lower.contains("update is available")
}

#[tauri::command]
pub async fn is_update_tool_installed() -> Result<bool, String> {
    Ok(Command::new_sidecar(UPDATE_TOOL)
        .map_err(|e| format!("{}", e))?
        .args(["--help"])
        .output()
        .is_ok())
}

#[tauri::command]
pub async fn check_for_update(path: String, window: Window) -> Result<bool, String> {
    let _ = window.emit("install-progress", "Checking for updates...");

    let cmd = Command::new_sidecar(UPDATE_TOOL)
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?;

    let output = cmd.args(["--check-for-update", &path]).output();

    match output {
        Ok(out) => {
            let _ = window.emit("install-progress", "Done");

            if out.status.success() || out.status.code() == Some(1) {
                Ok(update_check_indicates_available(
                    out.status.code(),
                    &out.stdout,
                    &out.stderr,
                ))
            } else {
                Err(format!("Update check failed: {}", out.stderr.trim()))
            }
        },
        Err(e) => {
            let _ = window.emit("install-progress", "Update tool not found.");
            Err(e.to_string())
        },
    }
}

#[tauri::command]
pub async fn apply_update(path: String, window: Window) -> Result<String, String> {
    let _ = window.emit("install-progress", "Downloading delta update...");

    let cmd = Command::new_sidecar(UPDATE_TOOL)
        .map_err(|e| format!("Failed to create sidecar command: {}", e))?;

    let output = cmd.args([&path]).output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let _ = window.emit("install-progress", "Update applied successfully.");
                Ok("Update applied successfully.".into())
            } else {
                let _ = window.emit("install-progress", "Update failed.");
                Err(format!("Update failed: {}", out.stderr.trim()))
            }
        },
        Err(e) => Err(e.to_string()),
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
