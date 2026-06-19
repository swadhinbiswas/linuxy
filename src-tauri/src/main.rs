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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle();
            watcher::start_watcher(handle.clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            installer::install_appimage,
            installer::install_deb,
            installer::install_rpm,
            installer::install_executable,
            manager::get_installed_apps,
            manager::launch_app,
            manager::remove_app,
            manager::open_directory,
            manager::toggle_sandbox,
            manager::open_url,
            manager::get_storage_stats,
            manager::analyze_storage,
            manager::cleanup_storage,
            manager::is_firejail_installed,
            manager::export_library,
            manager::import_library,
            updater::is_update_tool_installed,
            updater::check_for_update,
            updater::apply_update,
            updater::check_all_updates,
            downloader::download_and_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
