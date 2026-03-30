use std::process::{Command, Output};

use tauri::Window;

const UPDATE_TOOL: &str = "appimageupdatetool";

fn update_check_indicates_available(output: &Output) -> bool {
    let stdout = String::from_utf8_lossy(&output.stdout).to_ascii_lowercase();
    let stderr = String::from_utf8_lossy(&output.stderr).to_ascii_lowercase();

    output.status.code() == Some(1)
        || stdout.contains("update is available")
        || stderr.contains("update is available")
}

fn tool_missing_message() -> String {
    "appimageupdatetool is not installed. Install it to enable delta update checks and apply AppImage updates.".into()
}

#[tauri::command]
pub async fn is_update_tool_installed() -> Result<bool, String> {
    Ok(Command::new(UPDATE_TOOL).arg("--help").output().is_ok())
}

#[tauri::command]
pub async fn check_for_update(path: String, window: Window) -> Result<bool, String> {
    let _ = window.emit("install-progress", "Checking for updates...");

    let output = Command::new(UPDATE_TOOL)
        .arg("--check-for-update")
        .arg(&path)
        .output();

    match output {
        Ok(out) => {
            let _ = window.emit("install-progress", "Done");

            if out.status.success() || out.status.code() == Some(1) {
                Ok(update_check_indicates_available(&out))
            } else {
                let err = String::from_utf8_lossy(&out.stderr);
                Err(format!("Update check failed: {}", err.trim()))
            }
        },
        Err(_) => {
            let _ = window.emit("install-progress", "Update tool not found.");
            Err(tool_missing_message())
        },
    }
}

#[tauri::command]
pub async fn apply_update(path: String, window: Window) -> Result<String, String> {
    let _ = window.emit("install-progress", "Downloading delta update...");

    let output = Command::new(UPDATE_TOOL).arg(&path).output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let _ = window.emit("install-progress", "Update applied successfully.");
                Ok("Update applied successfully.".into())
            } else {
                let err = String::from_utf8_lossy(&out.stderr);
                let _ = window.emit("install-progress", "Update failed.");
                Err(format!("Update failed: {}", err.trim()))
            }
        },
        Err(_) => Err(tool_missing_message()),
    }
}

#[cfg(test)]
mod tests {
    use std::process::ExitStatus;

    use super::update_check_indicates_available;

    #[cfg(unix)]
    fn exit_status(code: i32) -> ExitStatus {
        use std::os::unix::process::ExitStatusExt;

        ExitStatus::from_raw(code << 8)
    }

    #[test]
    fn detects_update_from_exit_code() {
        let output = std::process::Output {
            status: exit_status(1),
            stdout: Vec::new(),
            stderr: Vec::new(),
        };

        assert!(update_check_indicates_available(&output));
    }

    #[test]
    fn detects_update_from_output_text() {
        let output = std::process::Output {
            status: exit_status(0),
            stdout: b"An update is available".to_vec(),
            stderr: Vec::new(),
        };

        assert!(update_check_indicates_available(&output));
    }

    #[test]
    fn reports_no_update_when_signal_absent() {
        let output = std::process::Output {
            status: exit_status(0),
            stdout: b"already up to date".to_vec(),
            stderr: Vec::new(),
        };

        assert!(!update_check_indicates_available(&output));
    }
}
