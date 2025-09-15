use super::*;

const ACTOR_MAX: usize = 99;
const INITIATIVE_MAX: usize = 9999;
const ACTOR_NAME_MAX_CHARS: usize = 20;
const TABLE_WIDTH: usize = 36;

#[derive(Debug)]
pub struct Actor {
    name: String,
    initiative: usize,
}

impl Actor {
    const FIELD_NAMES: [&str; 2] = ["Name", "Initiative"];
}

#[derive(Debug)]
pub enum TrackerError {
    MaximumActorsReached,
    EmptyActorName,
    ActorNameTooLong,
    ActorNameAlreadyExists,
    InitiativeIsZero,
    InitiativeTooLarge,
    TrackerEmpty,
    IndexDoesNotExist,
    InvalidOption,
}

impl std::fmt::Display for TrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::TrackerEmpty => format!("tracker is empty"),
            Self::InitiativeIsZero => format!("initiative must be positive"),
            Self::InitiativeTooLarge => format!("initiative must be <={INITIATIVE_MAX}"),
            Self::IndexDoesNotExist => format!("index does not exist"),
            Self::EmptyActorName => format!("name cannot be empty"),
            Self::ActorNameTooLong => format!("name must be length <={ACTOR_NAME_MAX_CHARS}"),
            Self::ActorNameAlreadyExists => format!("name must be unique"),
            Self::MaximumActorsReached => format!("cannot add more than {ACTOR_MAX} actors"),
            Self::InvalidOption => format!("invalid option"),
        };

        write!(f, "{msg}")
    }
}

impl std::error::Error for TrackerError {}

fn reprompt_until_valid_option(options: &[&str]) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    let num_options = options.len();
    let mut option: usize = reprompt_until_ok("Choose an Option");

    if option > num_options {
        log_error(TrackerError::InvalidOption);
        return reprompt_until_valid_option(options);
    }

    option - 1
}

fn reprompt_until_valid_index(tracker: &Vec<Actor>) -> usize {
    let num_actors = tracker.len();

    if num_actors == 0 {
        log_error(TrackerError::TrackerEmpty);
        return reprompt_until_valid_index(tracker);
    }

    let mut index: usize = reprompt_until_ok("Enter Index");

    while index >= num_actors {
        index = reprompt_until_ok("Enter Index");

        if index >= num_actors {
            log_error(TrackerError::IndexDoesNotExist);
        }
    }

    index
}

fn reprompt_until_valid_name(tracker: &Vec<Actor>) -> String {
    let names = tracker.iter().map(|a| &a.name);
    let name = prompt_input("Enter Name");
    let length = name.len();
    let name_already_exists = names.clone().any(|n| n == &name);

    if length == 0 || length > ACTOR_NAME_MAX_CHARS || name_already_exists {
        if length == 0 {
            log_error(TrackerError::EmptyActorName);
        } else if length > ACTOR_NAME_MAX_CHARS {
            log_error(TrackerError::ActorNameTooLong);
        } else if name_already_exists {
            log_error(TrackerError::ActorNameAlreadyExists);
        }

        return reprompt_until_valid_name(tracker);
    }

    name
}

fn reprompt_until_valid_initiative() -> usize {
    let mut initiative = reprompt_until_ok("Enter Initiative");

    while initiative < 1 || initiative > INITIATIVE_MAX {
        initiative = reprompt_until_ok("Enter Initiative");

        if initiative == 0 {
            log_error(TrackerError::InitiativeIsZero);
        } else if initiative > INITIATIVE_MAX {
            log_error(TrackerError::InitiativeTooLarge);
        }

        return reprompt_until_valid_initiative();
    }

    initiative
}

pub fn create_actor(tracker: &mut Vec<Actor>, turn: &mut usize) {
    if tracker.len() == ACTOR_MAX {
        return log_error(TrackerError::MaximumActorsReached);
    }

    let name = reprompt_until_valid_name(tracker);
    let initiative = reprompt_until_valid_initiative();
    let actor = Actor { name, initiative };
    let mut index = 0;

    if !tracker.is_empty() {
        while tracker[index].initiative > actor.initiative {
            index += 1;
        }
    }

    tracker.insert(index, actor);

    if tracker.len() != 1 && index <= *turn {
        *turn += 1;
    }
}

pub fn read_actor(tracker: &Vec<Actor>) {
    let index = reprompt_until_valid_index(tracker);
    println!("{:?}", tracker[index]);
}

pub fn update_actor(tracker: &mut Vec<Actor>) {
    let index = reprompt_until_valid_index(&tracker);
    let option = reprompt_until_valid_option(&Actor::FIELD_NAMES);
    let field = Actor::FIELD_NAMES[option];

    match field {
        "Name" => {
            let name = reprompt_until_valid_name(&tracker);
            tracker[option].name = name;
        }
        "Initiative" => {
            let initiative = reprompt_until_valid_initiative();
            tracker[option].initiative = initiative;
        }
        _ => unreachable!(),
    }
}

pub fn delete_actor(tracker: &mut Vec<Actor>) {
    let index = reprompt_until_valid_index(tracker);
    tracker.remove(index);
}

pub fn next_turn(tracker: &Vec<Actor>, turn: &mut usize) {
    if tracker.is_empty() {
        log_error(TrackerError::TrackerEmpty);
        return;
    }

    *turn += 1;
    *turn %= tracker.len();
}

pub fn list_actors(tracker: &Vec<Actor>, turn: usize) {
    let print_row = || println!("  {}", "-".repeat(TABLE_WIDTH));

    print_row();
    println!("  | {0: <2} | {1: <20} | {2: <4} |", "i", "name", "init");
    print_row();

    for (i, actor) in tracker.iter().enumerate() {
        if i == turn {
            println!(
                ">>| {0: <2} | {1: <20} | {2: <4} |",
                i, actor.name, actor.initiative
            )
        } else {
            println!(
                "  | {0: <2} | {1: <20} | {2: <4} |",
                i, actor.name, actor.initiative
            );
        }
        print_row();
    }
}
