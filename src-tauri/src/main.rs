#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod downloader;
mod installer;
mod manager;
mod updater;
mod watcher;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::handle_cli(args).await;

    // Force X11 backend and disable hardware compositing to fix WebKitGTK Wayland crash
    std::env::set_var("GDK_BACKEND", "x11");
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            watcher::start_watcher(handle);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            installer::install_appimage,
            manager::get_installed_apps,
            manager::launch_app,
            manager::remove_app,
            manager::open_directory,
            manager::toggle_sandbox,
            manager::open_url,
            manager::get_storage_stats,
            manager::is_firejail_installed,
            updater::is_update_tool_installed,
            updater::check_for_update,
            updater::apply_update,
            downloader::download_and_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
