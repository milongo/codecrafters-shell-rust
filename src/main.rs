use std::fmt::format;
#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(input: &str) {
    let mut input_parts = input.split_ascii_whitespace();
    let command = input_parts.next();
    let mut args = input_parts;

    match command {
        Some("exit") => std::process::exit(0),
        Some("echo") => println!("{}", args.collect::<Vec<&str>>().join(" ")),
        Some("type") => handle_type(args.next()),
        _ => println!("{}: command not found", input),
    }
}

fn search_path(command: &str) -> String {
    let path_env_var= std::env::var("PATH").unwrap();
    let paths = path_env_var.split(":").collect::<Vec<&str>>();

    for path in paths {
        let file = format!("{}/{}", path, command);

        match std::fs::exists(&file) {
            Ok(true) => return format!("{} is {}", command, path),
            Ok(false) => continue,
            Err(_) => continue,
        }
    }
    format!("{}: not found", command)
}

fn handle_type(command: Option<&str>) {
    match command {
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some(cmd) => println!("{}", search_path(cmd)),
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
