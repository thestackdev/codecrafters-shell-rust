use std::env::current_dir;
use std::io::{self, Write};
use std::process::Command;
use std::str::FromStr;
use which::which;

enum AvailableCommands {
    Echo,
    Type,
    Exit,
    PWD,
    CD,
}

impl FromStr for AvailableCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo),
            "type" => Ok(Self::Type),
            "exit" => Ok(Self::Exit),
            "pwd" => Ok(Self::PWD),
            "cd" => Ok(Self::CD),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}

fn main() {
    let mut command = String::new();

    'main: loop {
        command.clear();

        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();

        if command.is_empty() {
            continue;
        }

        let args = command.split_whitespace().collect::<Vec<_>>();

        let builtin: Result<AvailableCommands, String> = args[0].parse();

        match builtin {
            Ok(AvailableCommands::Echo) => {
                println!("{}", args[1..].join(" "));
            }
            Ok(AvailableCommands::CD) => {
                if std::env::set_current_dir(args[1]).is_err() {
                    println!("cd: {}: No such file or directory", args[1]);
                }
            }
            Ok(AvailableCommands::Type) => {
                if args[1].parse::<AvailableCommands>().is_ok() {
                    println!("{} is a shell builtin", args[1]);
                } else {
                    if let Ok(path) = which(args[1]) {
                        println!("{} is {}", args[1], path.to_string_lossy());
                    } else {
                        println!("{}: not found", args[1]);
                    }
                }
            }
            Ok(AvailableCommands::PWD) => {
                let current_dir = std::env::current_dir().expect("Failed to get current_dir");
                println!("{}", current_dir.to_string_lossy());
            }
            Ok(AvailableCommands::Exit) => {
                break 'main;
            }
            Err(e) => {
                if let Ok(path) = which(args[0]) {
                    let result = Command::new(args[0])
                        .args(&args[1..])
                        .output()
                        .expect("failed to execute command");
                    if result.status.success() {
                        if let Ok(output) = str::from_utf8(&result.stdout) {
                            print!("{}", output);
                        }
                    } else {
                        if let Ok(output) = str::from_utf8(&result.stderr) {
                            eprint!("{}", output);
                        }
                    }
                } else {
                    println!("{}", e);
                }
            }
        }
        io::stderr().flush().unwrap();
        io::stdout().flush().unwrap();
    }
}
