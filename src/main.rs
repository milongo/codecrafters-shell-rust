#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, path};

/// Given a command name, try to find its full path by checking the PATH environment variable.
/// If the command contains a slash, we assume it’s a path and check it directly.
fn find_command_path(cmd: &str) -> Option<path::PathBuf> {
    // If cmd contains a slash, assume it's already a path.
    if cmd.contains('/') {
        let p = path::PathBuf::from(cmd);
        if p.exists() {
            return Some(p);
        } else {
            return None;
        }
    }

    // Otherwise, search through the PATH environment variable.
    if let Ok(path_env) = env::var("PATH") {
        for p in path_env.split(':') {
            let full_path = path::Path::new(p).join(cmd);
            if full_path.exists() {
                return Some(full_path);
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Split the input into command parts
        let command_parts = trimmed_input.split_whitespace().collect::<Vec<&str>>();
        let command = command_parts[0];
        let args = &command_parts[1..];

        match command {
            "exit" => {
                if let Some(exit_code) = args.get(0) {
                    match exit_code.parse::<i32>() {
                        Ok(code) => std::process::exit(code),
                        Err(_) => println!("exit: invalid exit code"),
                    }
                } else {
                    std::process::exit(0); // Default exit code is 0.
                }
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                if let Some(next_command) = args.get(0) {
                    match *next_command {
                        "echo" => println!("echo is a shell builtin"),
                        "exit" => println!("exit is a shell builtin"),
                        "type" => println!("type is a shell builtin"),
                        // "pwd" => println!("pwd is a shell builtin"),
                        _ => {
                            // Here we use the same PATH-search logic.
                            if let Some(full_path) = find_command_path(next_command) {
                                println!("{} is {}", next_command, full_path.display());
                            } else {
                                println!("{}: not found", next_command);
                            }
                        }
                    }
                } else {
                    println!("type: missing argument");
                }
            }
            _ => {
                // Before executing an external command, search for its full path.
                let full_command = if command.contains('/') {
                    path::PathBuf::from(command)
                } else {
                    match find_command_path(command) {
                        Some(p) => p,
                        None => {
                            println!("{}: command not found", command);
                            continue;
                        }
                    }
                };
                println!("Executing: {:?}", full_command);
                let output = Command::new(full_command).args(args).output();

                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            println!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(err) => println!("Error executing {}: {}", command, err),
                }
            }
        }
    }
}
