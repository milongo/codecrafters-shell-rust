use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::{env, path};

enum BuiltinCommand {
    Pwd,
    Echo,
    Exit,
    Type,
    Cd,
}

impl BuiltinCommand {
    fn execute(&self, args: &[String], builtins: &HashMap<&str, BuiltinCommand>) {
        match self {
            BuiltinCommand::Pwd => match env::current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(err) => eprintln!("pwd: error retrieving current directory: {}", err),
            },
            BuiltinCommand::Echo => {
                println!("{}", args.join(" "));
            }
            BuiltinCommand::Exit => {
                let code = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                std::process::exit(code);
            }
            BuiltinCommand::Type => {
                if let Some(command) = args.get(0) {
                    if builtins.contains_key(command.as_str()) {
                        println!("{} is a shell builtin", command);
                    } else if let Some(path) = find_command_path(command) {
                        println!("{} is {}", command, path.to_string_lossy());
                    } else {
                        println!("{}: not found", command);
                    }
                } else {
                    println!("type: missing argument");
                }
            }
            BuiltinCommand::Cd => {
                cd(args);
            }
        }
    }
}

fn cd(args: &[String]) {
    let target_dir = if let Some(path) = args.get(0) {
        if path == &"~" {
            match env::var("HOME") {
                Ok(home) => home,
                Err(_) => {
                    eprintln!("cd: $HOME not set");
                    return;
                }
            }
        } else {
            path.to_string()
        }
    } else {
        match env::var("HOME") {
            Ok(home) => home, // Default to home if no arguments
            Err(_) => {
                eprintln!("cd: $HOME not set");
                return;
            }
        }
    };

    let path = path::Path::new(&target_dir);
    if let Err(_) = env::set_current_dir(path) {
        eprintln!("cd: {}: No such file or directory", target_dir);
    }
}

fn find_command_path(cmd: &str) -> Option<std::path::PathBuf> {
    if cmd.contains("/") {
        let p = path::PathBuf::from(cmd);
        if p.exists() {
            return Some(p);
        }
        return None;
    }

    if let Ok(path_env) = env::var("PATH") {
        for p in path_env.split(":") {
            let full_path = path::Path::new(p).join(cmd);
            if full_path.exists() {
                return Some(full_path);
            }
        }
    }
    None
}

fn parse_input(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut inside_single_quotes = false;
    let mut inside_double_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !inside_double_quotes => {
                // Toggle single quote state (preserve everything inside single quotes)
                inside_single_quotes = !inside_single_quotes;
            }
            '"' if !inside_single_quotes => {
                // Toggle double quote state
                inside_double_quotes = !inside_double_quotes;
            }
            '\\' if inside_double_quotes => {
                // Handle escape sequences inside double quotes
                match chars.peek() {
                    Some(&'"') => {
                        chars.next(); // Consume the escaped quote
                        current_part.push('"');
                    }
                    Some(&'\\') => {
                        chars.next(); // Consume the escaped backslash
                        current_part.push('\\');
                    }
                    Some(&'$') => {
                        chars.next(); // Consume the escaped dollar sign
                        current_part.push('$');
                    }
                    Some(&'\n') => {
                        chars.next(); // Ignore escaped newlines
                    }
                    _ => current_part.push('\\'), // Keep other backslashes
                }
            }
            ' ' if !inside_single_quotes && !inside_double_quotes => {
                // If we encounter a space outside of quotes, push the current word
                if !current_part.is_empty() {
                    parts.push(current_part.clone());
                    current_part.clear();
                }
            },
            '\\' if !inside_single_quotes && !inside_double_quotes => {
                if let Some(next_char) = chars.next() {
                    current_part.push(next_char);
                }
            }

            _ => current_part.push(c),
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }

    parts
}



struct Shell {
    builtins: HashMap<&'static str, BuiltinCommand>,
}

impl Shell {
    fn new() -> Self {
        let mut builtins = HashMap::new();
        builtins.insert("pwd", BuiltinCommand::Pwd);
        builtins.insert("echo", BuiltinCommand::Echo);
        builtins.insert("exit", BuiltinCommand::Exit);
        builtins.insert("type", BuiltinCommand::Type);
        builtins.insert("cd", BuiltinCommand::Cd);

        Shell { builtins }
    }

    fn run(&self) {
        let stdin = io::stdin();
        let mut input = String::new();

        loop {
            input.clear();
            print!("$ ");
            io::stdout().flush().unwrap();

            if stdin.read_line(&mut input).is_err() {
                eprintln!("Error reading input.");
                continue;
            }

            let trimmed_input = input.trim();
            if trimmed_input.is_empty() {
                continue;
            }
            
            let parts = parse_input(trimmed_input);
            let command = &parts[0];
            let args = &parts[1..];

            if let Some(builtin) = self.builtins.get(command.as_str()) {
                builtin.execute(args, &self.builtins);
            } else {
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
                    Err(_err) => println!("{}: command not found", command),
                }
            }
        }
    }
}

fn main() {
    let shell = Shell::new();
    shell.run();
}
