#![allow(unused)]

mod cli;
mod tracker;

use cli::*;
use tracker::*;

fn main() {
    println!(
        "Pathfinder 2E Combat Tracker CLI v.{}",
        env!("CARGO_PKG_VERSION")
    );
    println!("Enter the 'help' command to see available commands");

    let mut turn = 0;
    let mut tracker = Vec::new();

    loop {
        let cmd = Command::try_from(prompt_input("Enter Command").as_str());

        match cmd {
            Err(e) => log_error(e),
            Ok(Command::Help) => print_help(),
            Ok(Command::Quit) => break,
            Ok(Command::Create) => create_actor(&mut tracker, &mut turn),
            Ok(Command::Read) => read_actor(&tracker),
            Ok(Command::Update) => update_actor(&mut tracker),
            Ok(Command::Delete) => delete_actor(&mut tracker),
            Ok(Command::List) => list_actors(&tracker, turn),
            Ok(Command::Next) => next_turn(&tracker, &mut turn),
        }
    }

    println!("Exiting tracker...")
}
