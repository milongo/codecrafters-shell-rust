#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

fn handle_command(input: &str) {
    let mut input_parts = input.split_ascii_whitespace();
    let command = input_parts.next();
    let mut args = input_parts;

    match command {
        Some("exit") => std::process::exit(0),
        Some("echo") => println!("{}", args.collect::<Vec<&str>>().join(" ")),
        Some("type") => handle_type(args.next()),
        Some(cmd) => {
            let result = search_path(cmd);
            if result.0 {
                Command::new(result.1).args(args).status().expect("Failed to execute process");
            }
            else {
                println!("{}", result.1)
            }
        }
        None => {}
    }
}

fn search_path(command: &str) -> (bool, String) {
    let path_env_var= std::env::var("PATH").unwrap();
    let paths = path_env_var.split(":").collect::<Vec<&str>>();

    for path in paths {
        let file = format!("{}/{}", path, command);

        match std::fs::exists(&file) {
            Ok(true) => return (true, file),
            Ok(false) => continue,
            Err(_) => continue,
        }
    }
    (false, format!("{}: not found", command))
}

fn handle_type(command: Option<&str>) {
    match command {
        Some("echo") => println!("echo is a shell builtin"),
        Some("type") => println!("type is a shell builtin"),
        Some("exit") => println!("exit is a shell builtin"),
        Some(cmd) => {
            let result = search_path(cmd);
            if result.0 {
                println!("{} is {}", cmd, result.1)
            }
            else {
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
