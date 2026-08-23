#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

enum AvailableCommands {
    Echo,
    Exit,
}

impl FromStr for AvailableCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Self::Echo),
            "exit" => Ok(Self::Exit),
            _ => Err(format!("{}: command not found", s)),
        }
    }
}

fn echo(s: String) {
    println!("{}", s);
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

        let builtin: Result<AvailableCommands, String> =
            command.split_whitespace().next().unwrap().parse();

        let args = command
            .split_whitespace()
            .skip(1)
            .collect::<Vec<_>>()
            .join(" ");

        match builtin {
            Ok(AvailableCommands::Echo) => {
                echo(args);
            }
            Ok(AvailableCommands::Exit) => {
                break 'main;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
