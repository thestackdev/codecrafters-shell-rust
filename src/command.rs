use which::which;

pub enum AvailableCommands {
    Echo,
    Exit,
    Type,
    Pwd,
    CD,
}

impl AvailableCommands {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "echo" => Some(Self::Echo),
            "exit" => Some(Self::Exit),
            "type" => Some(Self::Type),
            "pwd" => Some(Self::Pwd),
            "cd" => Some(Self::CD),
            _ => None,
        }
    }

    pub fn is_builtin(command: &str) -> bool {
        Self::from_str(command).is_some()
    }

    pub fn execute(self, args: &[String]) {
        match self {
            AvailableCommands::Echo => {
                println!("{}", args[1..].join(" "));
            }
            AvailableCommands::Exit => {
                unimplemented!()
            }
            AvailableCommands::CD => {
                let homedir = std::env::home_dir()
                    .expect("Failed to get home dir")
                    .to_string_lossy()
                    .to_string();

                let result = if args.len() <= 1 {
                    std::env::set_current_dir(homedir)
                } else if args[1].starts_with("~") {
                    std::env::set_current_dir(homedir + args[1].replace("~", "").as_str())
                } else {
                    std::env::set_current_dir(&args[1])
                };

                if result.is_err() {
                    println!("cd: {}: No such file or directory", args[1]);
                }
            }
            AvailableCommands::Type => {
                if Self::is_builtin(&args[1]) {
                    println!("{} is a shell builtin", args[1]);
                } else {
                    if let Ok(path) = which(&args[1]) {
                        println!("{} is {}", args[1], path.to_string_lossy());
                    } else {
                        println!("{}: not found", args[1]);
                    }
                }
            }
            AvailableCommands::Pwd => {
                let current_dir = std::env::current_dir().expect("Failed to get current_dir");
                println!("{}", current_dir.to_string_lossy());
            }
        }
    }
}
