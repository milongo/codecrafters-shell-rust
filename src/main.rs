#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

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

        // Collect command parts into a Vec
        let command_parts: Vec<&str> = input.split_whitespace().collect();
        let command = command_parts[0];
        let args = &command_parts[1..];

        match command {
            "exit" => {
                if let Some(exit_code) = args.get(0) {
                    match exit_code.parse::<i32>() {
                        Ok(code) => exit(code),
                        Err(_) => println!("exit: invalid exit code"),
                    }
                } else {
                    exit(0); // Default exit code is 0
                }
            },
            _ => {
                let output = Command::new(command)
                    .args(args)
                    .output();
            },
        }
    }
}
