#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

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
        let mut command_parts = input.split_whitespace();
        
        match command_parts.next().unwrap() {
            "exit" => exit(command_parts.next().unwrap().parse().unwrap()),
            "echo" => println!("{}", command_parts.collect::<Vec<&str>>().join(" ")),
            "type" => match command_parts.next().unwrap() {
                "echo" => println!("echo is a shell builtin"),
                "exit" => println!("exit is a shell builtin"),
                "type" => println!("type is a shell builtin"),
                _ => println!("{}: not found", input)
            },
            _ => println!("{}: command not found", input),
        }
    }
}
