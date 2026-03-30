use crate::installer;
use crate::manager;

fn is_appimage_path(arg: &str) -> bool {
    let lower = arg.to_ascii_lowercase();
    lower.ends_with(".appimage")
}

pub async fn handle_cli(args: Vec<String>) {
    if args.len() < 2 {
        return;
    }

    let command = &args[1];
    match command.as_str() {
        path if is_appimage_path(path) => {
            println!("Installing {}...", path);
            match installer::install_appimage_internal(path.to_string(), None).await {
                Ok(msg) => println!("{}", msg),
                Err(err) => {
                    println!("Error: {}", err);
                    std::process::exit(1);
                },
            }
            std::process::exit(0);
        },
        "list" => {
            if let Ok(apps) = manager::get_installed_apps().await {
                println!("{:<30} {:<10} {:<20}", "NAME", "SANDBOX", "PATH");
                for app in apps {
                    println!("{:<30} {:<10} {:<20}", app.name, app.sandboxed, app.path);
                }
            }
            std::process::exit(0);
        },
        "install" => {
            if args.len() < 3 {
                println!("Usage: linuxy install <path>");
                std::process::exit(1);
            }
            let path = &args[2];
            println!("Installing {}...", path);
            match installer::install_appimage_internal(path.clone(), None).await {
                Ok(msg) => println!("{}", msg),
                Err(err) => println!("Error: {}", err),
            }
            std::process::exit(0);
        },
        "help" | "--help" | "-h" => {
            println!("linuxy - AppImage Manager");
            println!("Usage:");
            println!("  linuxy <path>         Install an AppImage by file path");
            println!("  linuxy install <path> Install an AppImage by file path");
            println!("  linuxy list           List installed AppImages");
            println!("  linuxy help           Show this help");
            std::process::exit(0);
        },
        _ => {},
    }
}
