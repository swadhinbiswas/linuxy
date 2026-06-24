#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(target_os = "linux")]
mod cli;
mod downloader;
mod installer;
mod manager;
mod updater;
#[cfg(target_os = "linux")]
mod watcher;

use tauri::Emitter;

#[tokio::main]
async fn main() {
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None::<Vec<&str>>,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            let _ = app.emit("single-instance", ());
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                let handle = app.handle().clone();
                watcher::start_watcher(handle);
            }

            // Handle CLI file paths passed as arguments
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let path = &args[1];
                if !path.starts_with('-') {
                    let path = path.clone();
                    tauri::async_runtime::spawn(async move {
                        #[cfg(target_os = "linux")]
                        cli::handle_cli(&path).await;
                    });
                }
            }

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
