use std::env::current_dir;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
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
            let (found, path) = search_path(cmd);
            if found {
                let arg0 = Path::new(&path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(cmd);

                Command::new(&path)
                    .arg0(arg0)
                    .args(args) 
                    .status()
                    .expect("Failed to execute process");
            } else {
                println!("{}", path);
            }
        }
        None => {}
    }
}

fn search_path(command: &str) -> (bool, String) {
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
                return (true, file.display().to_string());
            }
        }
    }
    (false, format!("{}: not found", command))
}

fn handle_type(command: Option<&str>) {
    match command {
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some("pwd") => println!("pwd is a shell builtin"),
        Some(cmd) => {
            let result = search_path(cmd);
            if result.0 {
                println!("{} is {}", cmd, result.1)
            } else {
                println!("{}", result.1)
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
