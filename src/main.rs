use std::env::{current_dir, home_dir, set_current_dir};
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn get_command_and_args(input: &str) -> (Option<String>, Vec<String>) {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut pushing = false;

    for char in input.chars() {
        if char == '\'' {
            pushing = !pushing;
            continue;
        }

        if pushing {
            buffer.push(char);
        } else if char == ' ' {
            if !buffer.is_empty() {
                tokens.push(buffer.clone());
                buffer.clear();
            }
        } else {
            buffer.push(char);
        }
    }

    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    let command = tokens.get(0).cloned();
    let args = tokens.into_iter().skip(1).collect();

    (command, args)
}

fn handle_command(command: Option<&str>, args: &[&str]) {
    match command {
        Some("exit") => std::process::exit(0),
        Some("echo") => {
            println!("{}", args.join(" "));
        }
        Some("type") => handle_type(args.first().copied()),
        Some("pwd") => {
            println!("{}", current_dir().unwrap().display())
        }
        Some("cd") => {
            let path_str = args.first().copied();
            cd(path_str);
        }
        Some(cmd) => {
            let path = search_path(cmd);
            match path {
                Some(path) => {
                    execute_command(cmd, &path, args);
                }
                _ => eprintln!("{}: not found", cmd),
            }
        }
        None => {}
    }
}

fn execute_command(cmd: &str, path: &Path, args: &[&str]) {
    let arg0 = path.file_name().and_then(|s| s.to_str()).unwrap_or(cmd); // cringeeeee
    Command::new(&path)
        .arg0(arg0)
        .args(args)
        .status()
        .expect("Failed to execute process");
}

fn cd(path: Option<&str>) {
    let raw = match path {
        None | Some("~") => match home_dir() {
            Some(home) => home,
            None => {
                eprintln!("cd: HOME not set");
                return;
            }
        },
        Some(s) => PathBuf::from(s),
    };

    if let Err(_e) = set_current_dir(&raw) {
        eprintln!("cd: {}: No such file or directory", raw.display());
    }
}

fn search_path(command: &str) -> Option<PathBuf> {
    let path_env_var = match std::env::var("PATH") {
        Ok(p) => p,
        Err(_) => return None,
    };
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
        get_command_and_args(input);
        // handle_command(command, &args);
    }
}
