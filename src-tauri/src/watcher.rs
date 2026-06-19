use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tauri::{AppHandle, Emitter};

const POLL_INTERVAL_SECS: u64 = 5;
const STABLE_POLLS_REQUIRED: u8 = 2;

#[derive(Clone, Debug, Default)]
struct TrackedFile {
    last_size: u64,
    stable_polls: u8,
    notified: bool,
}

fn is_appimage_path(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("AppImage") | Some("appimage")
    )
}

fn update_tracking_state(state: &mut TrackedFile, current_size: u64) {
    if state.last_size == current_size {
        state.stable_polls = state.stable_polls.saturating_add(1);
    } else {
        state.last_size = current_size;
        state.stable_polls = 0;
    }
}

pub fn start_watcher(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let Some(home_dir) = dirs::home_dir() else {
            return;
        };
        let downloads_dir = home_dir.join("Downloads");
        if !downloads_dir.exists() {
            return;
        }

        let mut tracked_files: HashMap<PathBuf, TrackedFile> = HashMap::new();

        loop {
            std::thread::sleep(Duration::from_secs(POLL_INTERVAL_SECS));

            let mut seen_paths = HashSet::new();
            if let Ok(entries) = fs::read_dir(&downloads_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() || !is_appimage_path(&path) {
                        continue;
                    }

                    let Ok(metadata) = entry.metadata() else {
                        continue;
                    };

                    let current_size = metadata.len();
                    if current_size == 0 {
                        continue;
                    }

                    seen_paths.insert(path.clone());
                    let state = tracked_files.entry(path.clone()).or_default();
                    update_tracking_state(state, current_size);

                    if !state.notified && state.stable_polls >= STABLE_POLLS_REQUIRED {
                        let _ = app_handle.emit("appimage-detected", path.to_string_lossy());
                        state.notified = true;
                    }
                }
            }

            tracked_files.retain(|path, _| seen_paths.contains(path));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{update_tracking_state, TrackedFile};

    #[test]
    fn marks_file_stable_only_after_repeated_same_size() {
        let mut state = TrackedFile::default();

        update_tracking_state(&mut state, 128);
        assert_eq!(state.stable_polls, 0);

        update_tracking_state(&mut state, 128);
        assert_eq!(state.stable_polls, 1);

        update_tracking_state(&mut state, 128);
        assert_eq!(state.stable_polls, 2);
    }

    #[test]
    fn resets_stability_when_size_changes() {
        let mut state = TrackedFile::default();

        update_tracking_state(&mut state, 128);
        update_tracking_state(&mut state, 128);
        update_tracking_state(&mut state, 256);

        assert_eq!(state.last_size, 256);
        assert_eq!(state.stable_polls, 0);
    }
}
