#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
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
                        Ok(code) => std::process::exit(code),
                        Err(_) => println!("exit: invalid exit code"),
                    }
                } else {
                    std::process::exit(0); // Default exit code is 0
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
                        }
                    }
                } else {
                    println!("type: missing argument");
                }
            }
            _ => {
                let output = Command::new(command).args(args).output();

                match output {
                    Ok(output) => {
                        if !output.stdout.is_empty() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(err) => println!("{}: command not found ({})", command, err),
                }
            }
        }
    }
}
