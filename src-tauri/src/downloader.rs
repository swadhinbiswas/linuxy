use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use tauri::{Emitter, Window};

use crate::installer;

#[tauri::command]
pub async fn download_and_install(
    url: String,
    name: String,
    window: Window,
) -> Result<String, String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("Only http:// and https:// downloads are allowed.".into());
    }

    let _ = window.emit("install-progress", format!("Downloading {}...", name));

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("Download failed with HTTP status {}.", status));
    }

    let total_size = response.content_length().unwrap_or(0);
    let temp_path = std::env::temp_dir().join(format!("{}.AppImage", uuid::Uuid::new_v4()));
    let mut file = File::create(&temp_path).map_err(|e| e.to_string())?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| e.to_string())?;
        if chunk.is_empty() {
            continue;
        }

        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        if total_size > 0 {
            let percentage = (downloaded as f64 / total_size as f64) * 100.0;
            let _ = window.emit(
                "install-progress",
                format!("Downloading: {:.1}%", percentage),
            );
        }
    }

    file.flush().map_err(|e| e.to_string())?;

    if downloaded == 0 {
        let _ = std::fs::remove_file(&temp_path);
        return Err("Download completed with no data.".into());
    }

    let _ = window.emit("install-progress", "Download complete. Installing...");

    let result = installer::install_appimage_internal(
        temp_path.to_string_lossy().to_string(),
        Some(window.clone()),
    )
    .await;

    let _ = std::fs::remove_file(&temp_path);
    result
}
