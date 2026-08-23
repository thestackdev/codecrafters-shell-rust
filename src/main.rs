#[allow(unused_imports)]
use std::io::{self, Write};
use std::str::FromStr;

enum AvailableCommands {
    Exit,
}

impl FromStr for AvailableCommands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(Self::Exit),
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

        let builtin: Result<AvailableCommands, String> = command.trim().parse();

        match builtin {
            Ok(AvailableCommands::Exit) => {
                break 'main;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
