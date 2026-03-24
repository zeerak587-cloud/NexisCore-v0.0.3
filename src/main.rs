use std::env;
use std::fs;
use std::io::{self, Write};
use std::collections::HashMap;
use std::path::PathBuf;

const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

#[derive(Clone)]
struct Config {
    safe_mode: bool,
    confirm_delete: bool,
    symbol_for_root: String,
}

fn load_config() -> Config {
    let path = "config.lst";

    if !std::path::Path::new(path).exists() {
        let default = "safe_mode = true\nconfirm_delete = true\nsymbol_for_root = \"R\"";
        fs::write(path, default).ok();
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    let mut map = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            map.insert(parts[0].trim(), parts[1].trim().trim_matches('"'));
        }
    }

    Config {
        safe_mode: map.get("safe_mode").unwrap_or(&"true") == &"true",
        confirm_delete: map.get("confirm_delete").unwrap_or(&"true") == &"true",
        symbol_for_root: map.get("symbol_for_root").unwrap_or(&"R").to_string(),
    }
}

// 🔥 YOUR exact syntax parser
fn extract_name(input: &str) -> Option<String> {
    let start = input.find("(\"")? + 2;
    let end = input[start..].find("\")")? + start;
    Some(input[start..end].to_string())
}

// 🔥 Run .nxcr script files
fn run_nxcr_file(path: &str, current_dir: &mut PathBuf, config: &Config) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("[ERR] Failed to read script: {}", e);
            return;
        }
    };

    println!("[NXCR] Running script: {}\n", path);

    for line in content.lines() {
        let input = line.trim();

        if input.is_empty() {
            continue;
        }

        execute_command(input, current_dir, config);
    }

    println!("\n[NXCR] Script finished");
}

// 🔥 Central command executor (shared by CLI + scripts)
fn execute_command(input: &str, current_dir: &mut PathBuf, config: &Config) {
    if input == "view_directory" {
        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    if path.is_dir() {
                        println!("[DIR] {}", name);
                    } else {
                        println!("      {}", name);
                    }
                }
            }
            Err(_) => println!("Failed to read directory"),
        }
    }

    else if input == "back" {
        if let Some(parent) = current_dir.parent() {
            current_dir.clone_from(&parent.to_path_buf());
        }
    }

    else if input.starts_with("make_directory") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);
            match fs::create_dir_all(path) {
                Ok(_) => println!("[ OK ] Directory created"),
                Err(_) => println!("Failed"),
            }
        } else {
            println!("Invalid format");
        }
    }

    else if input.starts_with("open_directory") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);
            if path.exists() && path.is_dir() {
                *current_dir = path;
            } else {
                println!("Directory not found");
            }
        } else {
            println!("Invalid format");
        }
    }

    else if input.starts_with("write_file") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);

            println!("Enter content (type END to finish):");

            let mut content = String::new();
            loop {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                if line.trim() == "END" {
                    break;
                }
                content.push_str(&line);
            }

            match fs::write(path, content) {
                Ok(_) => println!("[ OK ] File written"),
                Err(_) => println!("Failed"),
            }
        }
    }

    else if input.starts_with("open_file") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);
            match fs::read_to_string(path) {
                Ok(content) => println!("{}", content),
                Err(_) => println!("Failed to open file"),
            }
        }
    }

    else if input.starts_with("delete_file") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);

            if !path.exists() {
                println!("File not found");
                return;
            }

            if config.safe_mode || config.confirm_delete {
                println!("Delete {}? (y/n)", name);
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim() != "y" {
                    return;
                }
            }

            match fs::remove_file(path) {
                Ok(_) => println!("[ OK ] File deleted"),
                Err(_) => println!("Failed"),
            }
        }
    }

    else if input.starts_with("delete_directory") {
        if let Some(name) = extract_name(input) {
            let path = current_dir.join(&name);

            if !path.exists() {
                println!("Folder not found");
                return;
            }

            if config.safe_mode || config.confirm_delete {
                println!("Delete folder {} and all contents? (y/n)", name);
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim() != "y" {
                    return;
                }
            }

            match fs::remove_dir_all(path) {
                Ok(_) => println!("[ OK ] Folder deleted"),
                Err(_) => println!("Failed"),
            }
        }
    }

    else {
        println!("Unknown command");
    }
}

fn main() {
    let config = load_config();

    let base_dir = env::current_dir().unwrap();
    let env_dir = base_dir.join("environment");
    fs::create_dir_all(&env_dir).ok();

    let mut current_dir = env_dir.clone();

    let args: Vec<String> = env::args().collect();

    // 🔥 SCRIPT MODE (double-click .nxcr)
    if args.len() > 1 {
        let file = &args[1];

        if file.ends_with(".nxcr") {
            run_nxcr_file(file, &mut current_dir, &config);

            println!("\nPress Enter to exit...");
            let mut s = String::new();
            io::stdin().read_line(&mut s).ok();
            return;
        }
    }

    // 🔥 NORMAL MODE
    println!("{}=============================={}", BLUE, RESET);
    println!("{}NEXISCORE v0.0.3{}", BLUE, RESET);
    println!("{}=============================={}", BLUE, RESET);

    println!("{}[ OK ] Core Initialized{}", BLUE, RESET);
    println!("{}[ OK ] File System Linked{}", BLUE, RESET);
    println!("{}[ OK ] Environment Ready{}\n", BLUE, RESET);

    loop {
        print!("{}{}:>> {}", BLUE, config.symbol_for_root, RESET);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" {
            println!("Exiting NexisCore...");
            break;
        }

        if input == "help" {
            println!("\nCommands:");
            println!("make_directory (\\name = (\"example\")/)");
            println!("open_directory (\\name = (\"example\")/)");
            println!("write_file (\\name = (\"file.txt\")/)");
            println!("open_file (\\name = (\"file.txt\")/)");
            println!("delete_file (\\name = (\"file.txt\")/)");
            println!("delete_directory (\\name = (\"folder\")/)");
            println!("view_directory");
            println!("back");
            println!("exit\n");
            continue;
        }

        execute_command(input, &mut current_dir, &config);
    }
}