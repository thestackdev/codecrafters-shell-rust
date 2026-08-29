mod command;
mod utils;

use std::{
    io::{self, Write},
    process::Command,
};

use crate::command::AvailableCommands;

pub fn start() {
    let mut command = String::new();

    'main: loop {
        command.clear();

        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();

        let command = command.trim();

        if command == "exit" {
            break 'main;
        }

        if command.is_empty() {
            continue;
        }

        let args = utils::parse_args(command);

        let current_command = AvailableCommands::from_str(&args[0]);

        match current_command {
            Some(current_command) => current_command.execute(&args),
            None => {
                if which::which(&args[0]).is_err() {
                    println!("{}: command not found", &args[0]);
                    continue;
                }

                let result = Command::new(&args[0])
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
            }
        }

        io::stderr().flush().unwrap();
        io::stdout().flush().unwrap();
    }
}
