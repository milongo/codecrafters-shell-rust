#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
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
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                if let Some(next_command) = args.get(0) {
                    match *next_command {
                        "echo" => println!("echo is a shell builtin"),
                        "exit" => println!("exit is a shell builtin"),
                        "type" => println!("type is a shell builtin"),
                        _ => {
                            let path = env::var("PATH").unwrap_or_else(|_| String::new());
                            let paths = path.split(":");
                            let mut found = false;

                            for path in paths {
                                let full_path = path::Path::new(path).join(next_command);
                                if full_path.exists() {
                                    println!("{} is {}", next_command, full_path.display());
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                println!("{}: not found", next_command);
                            }
                        },
                    }
                } else {
                    println!("type: missing argument");
                }
            },
            _ => {
                let path = env::var("PATH").unwrap_or_else(|_| String::new());
                let paths = path.split(":");
                let mut found = false;

                for path in paths {
                    let full_path = path::Path::new(path).join(command);
                    if full_path.exists() {
                        println!("Program was passed {} args (including program name).",
                            command_parts.len());

                        println!("Arg #0 (program name): {}", command);
                        for (i, arg) in args.iter().enumerate() {
                            println!("Arg #{}: {}", i + 1, arg);
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    println!("{}: command not found", command);
                }
            },
        }
    }
}
