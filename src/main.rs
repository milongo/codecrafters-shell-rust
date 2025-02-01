use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{env, path}; // Required for UNIX `mode` checks

/// Given a command name, try to find its full path by checking the PATH environment variable.
/// If the command contains a slash, we assume it’s a path and check it directly.
/// Given a command name, try to find its full path by checking the PATH environment variable.
/// If the command contains a slash, we assume it’s a path and check it directly.
fn find_command_path(cmd: &str) -> Option<std::path::PathBuf> {
    // If cmd contains a slash, assume it's already a path.
    if cmd.contains("/") {
        let p = std::path::PathBuf::from(cmd);
        if p.exists() {
            return Some(p);
        } else {
            return None;
        }
    }

    // Otherwise, search through the PATH environment variable.
    if let Ok(path_env) = std::env::var("PATH") {
        for p in path_env.split(":") {
            let full_path = std::path::Path::new(p).join(cmd);
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

    let builtins = ["echo", "exit", "type", "pwd"]
        .iter()
        .copied()
        .collect::<HashSet<_>>();

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
            "type" => {
                let next_command = args[0];
                let command_path = find_command_path(next_command);
                if builtins.contains(&next_command) {
                    println!("{} is a shell builtin", next_command);
                } else if let Some(command_path) = command_path {
                    println!("{} is {}", next_command, command_path.to_string_lossy());
                } else {
                    println!("{}: not found", next_command);
                }
            }
            _ => {
                let output = Command::new(command).args(args).output();
                if let Err(err) = output {
                    println!("Error executing command: {}", err);
                    continue;
                }
            }
        }
    }
}
