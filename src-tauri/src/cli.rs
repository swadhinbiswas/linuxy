use crate::installer;
use crate::manager;

fn is_appimage_path(arg: &str) -> bool {
    arg.to_ascii_lowercase().ends_with(".appimage")
}

fn is_deb_path(arg: &str) -> bool {
    arg.to_ascii_lowercase().ends_with(".deb")
}

fn is_rpm_path(arg: &str) -> bool {
    arg.to_ascii_lowercase().ends_with(".rpm")
}

fn is_exe_path(arg: &str) -> bool {
    let l = arg.to_ascii_lowercase();
    l.ends_with(".exe") || l.ends_with(".msi")
}

pub async fn handle_cli(path: &str) {
    let owned = path.to_string();
    // Only treat as installer input if the path exists locally; deep-link URIs
    // like linuxy://host/file.deb must not trigger install+exit and should
    // fall through to the GUI-safe path.
    let is_local_file = !path.contains("://") && std::path::Path::new(path).exists();
    let install_result = if is_appimage_path(path) && is_local_file {
        println!("Installing {}...", path);
        Some(installer::install_appimage_internal(owned).await)
    } else if is_deb_path(path) && is_local_file {
        println!("Installing {}...", path);
        Some(installer::install_deb_internal(owned).await)
    } else if is_rpm_path(path) && is_local_file {
        println!("Installing {}...", path);
        Some(installer::install_rpm_internal(owned).await)
    } else if is_exe_path(path) && is_local_file {
        println!("Installing {}...", path);
        Some(installer::install_executable_internal(owned).await)
    } else {
        None
    };

    if let Some(result) = install_result {
        match result {
            Ok(msg) => println!("{}", msg),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            },
        }
        std::process::exit(0);
    } else {
        // Unrecognized argument: only treat as CLI "list" when running in a
        // terminal. Deep links / desktop-file launches that hit this fallback
        // should not terminate the GUI.
        if std::io::IsTerminal::is_terminal(&std::io::stdout()) {
            match manager::get_installed_apps().await {
                Ok(apps) => {
                    println!("{:<30} {:<10} {:<20}", "NAME", "SANDBOX", "PATH");
                    for app in apps {
                        println!("{:<30} {:<10} {:<20}", app.name, app.sandboxed, app.path);
                    }
                },
                Err(e) => eprintln!("Error: {}", e),
            }
            std::process::exit(0);
        } else {
            eprintln!(
                "Unrecognized argument '{}' — ignoring, continuing to GUI",
                path
            );
        }
    }
}
