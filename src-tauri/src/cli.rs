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
    if is_appimage_path(path) {
        println!("Installing {}...", path);
        match installer::install_appimage_internal(path.to_string()).await {
            Ok(msg) => println!("{}", msg),
            Err(err) => { eprintln!("Error: {}", err); std::process::exit(1); }
        }
        std::process::exit(0);
    } else if is_deb_path(path) {
        println!("Installing {}...", path);
        match installer::install_deb_internal(path.to_string()).await {
            Ok(msg) => println!("{}", msg),
            Err(err) => { eprintln!("Error: {}", err); std::process::exit(1); }
        }
        std::process::exit(0);
    } else if is_rpm_path(path) {
        println!("Installing {}...", path);
        match installer::install_rpm_internal(path.to_string()).await {
            Ok(msg) => println!("{}", msg),
            Err(err) => { eprintln!("Error: {}", err); std::process::exit(1); }
        }
        std::process::exit(0);
    } else if is_exe_path(path) {
        println!("Installing {}...", path);
        match installer::install_executable_internal(path.to_string()).await {
            Ok(msg) => println!("{}", msg),
            Err(err) => { eprintln!("Error: {}", err); std::process::exit(1); }
        }
        std::process::exit(0);
    } else {
        match manager::get_installed_apps().await {
            Ok(apps) => {
                println!("{:<30} {:<10} {:<20}", "NAME", "SANDBOX", "PATH");
                for app in apps { println!("{:<30} {:<10} {:<20}", app.name, app.sandboxed, app.path); }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        std::process::exit(0);
    }
}
