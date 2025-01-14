#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::{env, path};

fn main() {
    // Uncomment this block to pass the first stage
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

        let mut command_parts = input.split_whitespace();
        let command = command_parts.next().unwrap();
        match command {
            "exit" => {
                if let Some(exit_code) = command_parts.next() {
                    match exit_code.parse::<i32>() {
                        Ok(code) => exit(code),
                        Err(_) => println!("exit: invalid exit code"),
                    }
                }
            },
            "echo" => println!("{}", command_parts.collect::<Vec<&str>>().join(" ")),
            "type" => {
                if let Some(next_command) = command_parts.next() {
                    match next_command {
                        "echo" => println!("echo is a shell builtin"),
                        "exit" => println!("exit is a shell builtin"),
                        "type" => println!("type is a shell builtin"),
                        _ => { 
                            let path = env::var("PATH").unwrap();
                            let paths = path.split(":");
                            for path in paths {
                                let full_path = path::Path::new(path).join(next_command);
                                if full_path.exists() {
                                    println!("{} is {}", next_command, full_path.display());
                                    return;
                                }
                            }
                            println!("{}: not found", next_command)
                        },
                    }
                }
            }
            _ => println!("{}: command not found", command),
        }
    }
}
