use super::*;

const ERROR_EXIT_CODE: i32 = -1;
const INPUT_BUFFER_INITIAL_CAPACITY: usize = 8;
const PROMPT_SUFFIX: &str = " > ";
const ERROR_MSG_PREFIX: &str = "[ERROR] ";

#[derive(Debug)]
pub enum CLIError {
    IOException,
    InvalidCommand,
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::IOException => format!("a fatal IO exception has occured"),
            Self::InvalidCommand => format!("invalid command"),
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for CLIError {}

#[derive(Debug)]
pub enum Command {
    Help,
    Quit,
    Create,
    Read,
    Update,
    Delete,
    List,
    Next,
}

impl Command {
    const NUM_COMMANDS: usize = 8;
    const VALUES: [Self; Self::NUM_COMMANDS] = [
        Self::Help,
        Self::Quit,
        Self::Create,
        Self::Read,
        Self::Update,
        Self::Delete,
        Self::List,
        Self::Next,
    ];
}

impl TryFrom<&str> for Command {
    type Error = CLIError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "h" | "help" => Ok(Self::Help),
            "q" | "quit" => Ok(Self::Quit),
            "a" | "add" => Ok(Self::Create),
            "d" | "delete" => Ok(Self::Delete),
            "p" | "print" => Ok(Self::List),
            "c" | "change" => Ok(Self::Update),
            "r" | "read" => Ok(Self::Read),
            "n" | "next" => Ok(Self::Next),
            _ => Err(CLIError::InvalidCommand),
        }
    }
}

pub fn log_error<E: std::error::Error>(err: E) {
    eprintln!("{ERROR_MSG_PREFIX}{err}");
}

pub fn prompt_input(prompt: &str) -> String {
    use std::io::{self, Error, Write};
    use std::process::exit;

    let mut buf = String::with_capacity(INPUT_BUFFER_INITIAL_CAPACITY);
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    let on_io_error = |e: Error| -> Result<(), Error> {
        log_error(e);
        exit(ERROR_EXIT_CODE);
        Err(e)
    };

    print!("{prompt}{PROMPT_SUFFIX}");
    stdout.flush().or_else(on_io_error);
    // map result throws away Ok(n) value to satisfy type constraints of on_io_error
    stdin.read_line(&mut buf).map(|n| ()).or_else(on_io_error);
    buf.trim().to_string()
}

pub fn reprompt_until_ok<T: std::str::FromStr>(prompt: &str) -> T
where
    T::Err: std::error::Error,
{
    let input = prompt_input(prompt);

    match input.parse() {
        Err(e) => {
            log_error(e);
            reprompt_until_ok(prompt)
        }
        Ok(v) => v,
    }
}

pub fn print_help() {
    for cmd in Command::VALUES {
        match cmd {
            Command::Help => println!("help\n\tDisplay this output."),
            Command::Quit => println!("quit\n\tExit the tracker."),
            Command::Create => println!("add\n\tAdd an actor to the tracker."),
            Command::Read => println!("read\n\tDisplay details of an actor."),
            Command::Update => println!("change\n\tModify an attribute of an actor."),
            Command::Delete => println!("delete\n\tRemove an actor from the tracker."),
            Command::List => println!("print\n\tList all actors in the tracker."),
            Command::Next => println!("next\n\tGo to the next turn."),
        }
    }
}
