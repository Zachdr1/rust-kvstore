use std::collections::HashMap;
use std::io;

mod memory;
mod store;

//use store::memory::{Action, Command};

use std::str::FromStr;

#[derive(Debug)]
pub struct Command {
    pub action: Action,
    pub key: String,
    pub val: Option<String>,
}

#[derive(Debug)]
pub enum Action {
    Set,
    Get,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "set" => Ok(Action::Set),
            "get" => Ok(Action::Get),
            _ => Err(()),
        }
    }
}

impl Command {
    pub fn build<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<Command, &'static str> {
        let action_str = args.next().ok_or("Didn't get a command")?;
        let action = action_str.parse::<Action>().map_err(|_| "Invalid action")?;

        let key = args.next().ok_or("Didn't get a key")?.to_string();

        let val = match action {
            Action::Set => Some(
                args.next()
                    .ok_or("Didn't get a value for 'set' command")?
                    .to_string(),
            ),
            Action::Get => None,
        };

        Ok(Command { action, key, val })
    }
}

fn main() {
    let mut input = String::new();
    let mut map = HashMap::<String, String>::new();

    loop {
        input.clear();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e)
        }

        let words_iter = input.trim().split_whitespace();
        let command = match Command::build(words_iter) {
            Ok(cmd) => cmd,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        match command.action {
            Action::Set => {
                map.insert(command.key.clone(), command.val.clone().unwrap());
                println!(
                    "inserted key: {}, value: {}",
                    command.key,
                    command.val.unwrap()
                );
            }
            Action::Get => {
                let val = map.get(&command.key).expect("Failed to get value").clone();
                println!("Value: {}", val);
            }
        };
    }
}
