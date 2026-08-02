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
use tauri::Manager;

#[tokio::main]
async fn main() {
    #[cfg(target_os = "linux")]
    {
        // GTK natively handles backend negotiation (Wayland/X11).
        // We set an env var to prevent WebKit2GTK from freezing or crashing on some Wayland compositors.
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            if std::env::var("WEBKIT_DISABLE_DMABUF_RENDERER").is_err() {
                std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
            }
        }
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
            if let Some(w) = app.get_webview_window("main") {
                let _ = w.show();
                let _ = w.set_focus();
            }
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .on_window_event(|_app, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Brief sleep to let background threads (watcher, downloads) flush
                std::thread::sleep(std::time::Duration::from_millis(100));
                std::process::exit(0);
            }
        })
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                let handle = app.handle().clone();
                watcher::start_watcher(handle);

                // Self-register Linuxy in the application menu if running as an AppImage
                if let Ok(appimage_path) = std::env::var("APPIMAGE") {
                    if let Some(home) = dirs::home_dir() {
                        let apps_dir = home.join(".local/share/applications");
                        let _ = std::fs::create_dir_all(&apps_dir);
                        let desktop_path = apps_dir.join("linuxy.desktop");

                        let content = format!(
                            "[Desktop Entry]\nType=Application\nName=Linuxy\nComment=Application Manager\nExec=\"{}\" %U\nIcon=linuxy\nTerminal=false\nCategories=Utility;\n",
                            appimage_path
                        );

                        // Update if it doesn't exist or if the path has changed
                        let should_write = std::fs::read_to_string(&desktop_path)
                            .map(|existing| existing != content)
                            .unwrap_or(true);

                        if should_write {
                            let _ = std::fs::write(&desktop_path, content);
                            let _ = std::process::Command::new("update-desktop-database")
                                .arg(&apps_dir)
                                .output();
                            let _ = filetime::set_file_mtime(&apps_dir, filetime::FileTime::now());
                        }
                    }
                }
            }

            // Build tray icon menu
            let quit_item =
                tauri::menu::MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item =
                tauri::menu::MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = tauri::menu::MenuBuilder::new(app)
                .item(&show_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let _tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Linuxy - Application Manager")
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    use tauri::tray::TrayIconEvent;
                    if let TrayIconEvent::DoubleClick { .. } = event {
                        let app = tray.app_handle();
                        if let Some(w) = app.get_webview_window("main") {
                            let _ = w.show();
                            let _ = w.set_focus();
                        }
                    }
                })
                .build(app.handle())?;

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
            manager::get_app_checksum,
            manager::recreate_desktop_entry,
            updater::is_update_tool_installed,
            updater::check_for_update,
            updater::apply_update,
            updater::check_all_updates,
            downloader::download_and_install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
