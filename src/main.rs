use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;
use std::{env, path};

/// Enum for built-in commands
enum BuiltinCommand {
    Pwd,
    Echo,
    Exit,
    Type,
}

impl BuiltinCommand {
    fn execute(&self, args: &[&str], builtins: &HashMap<&str, BuiltinCommand>) {
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
                    if builtins.contains_key(*command) {
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
        }
    }
}

/// Find an executable in PATH
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

/// Shell struct encapsulating built-ins and execution logic
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

            let parts: Vec<&str> = trimmed_input.split_whitespace().collect();
            let command = parts[0];
            let args = &parts[1..];

            if let Some(builtin) = self.builtins.get(command) {
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
