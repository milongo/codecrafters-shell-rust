use std::env::current_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn handle_command(input: &str) {
    let mut input_parts = input.split_ascii_whitespace();
    let command = input_parts.next();
    let mut args = input_parts;

    match command {
        Some("exit") => std::process::exit(0),
        Some("echo") => println!("{}", args.collect::<Vec<&str>>().join(" ")),
        Some("type") => handle_type(args.next()),
        Some("pwd") => {
            println!(
                "{}",
                current_dir().expect("Something bad happened.").display()
            )
        }
        Some(cmd) => {
            let path = search_path(cmd);

            match path {
                Some(path) => {
                    let arg0 = path.file_name().and_then(|s| s.to_str()).unwrap_or(cmd);
                    Command::new(&path)
                        .arg0(arg0)
                        .args(args)
                        .status()
                        .expect("Failed to execute process");
                }
                _ => println!("{}: not found", cmd),
            }
        }
        None => {}
    }
}

fn search_path(command: &str) -> Option<PathBuf> {
    let path_env_var = std::env::var("PATH").unwrap();
    let paths = path_env_var.split(":");

    for path in paths {
        let file = Path::new(path).join(command);

        if file.exists() {
            let metadata = match file.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let mode = metadata.permissions().mode();

            if mode & 0o111 != 0 {
                return Some(file);
            }
        }
    }
    None
}

fn handle_type(command: Option<&str>) {
    match command {
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some("pwd") => println!("pwd is a shell builtin"),
        Some(cmd) => {
            let result = search_path(cmd);

            match result {
                Some(result) => println!("{} is {}", cmd, result.display()),
                _ => println!("{}: not found", cmd),
            }
        }
        _ => {}
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    // Wait for user input
    loop {
        input.clear();

        print!("$ ");
        io::stdout().flush().unwrap();

        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        handle_command(input);
    }
}
