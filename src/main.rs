use std::io;

mod storage;

use storage::hashmap::HashMapBackend;
use storage::kvstore::KeyValueStore;
use storage::log_storage::SimpleLogBackend;

use std::str::FromStr;

#[derive(Debug)]
pub struct Command {
    pub action: Action,
    pub key: Option<String>,
    pub val: Option<String>,
}

#[derive(Debug)]
pub enum Action {
    Set,
    Get,
    Del,
    Load,
    Exit,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "set" => Ok(Action::Set),
            "get" => Ok(Action::Get),
            "del" => Ok(Action::Del),
            "exit" => Ok(Action::Exit),
            "load" => Ok(Action::Load),
            _ => Err(()),
        }
    }
}

impl Command {
    pub fn build<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<Command, &'static str> {
        let action_str = args.next().ok_or("Didn't get a command")?;
        let action = action_str.parse::<Action>().map_err(|_| "Invalid action")?;

        let key = match action {
            Action::Exit => None,
            _ => Some(args.next().ok_or("Didn't get a key")?.to_string()),
        };

        let val = match action {
            Action::Set => Some(
                args.next()
                    .ok_or("Didn't get a value for 'set' command")?
                    .to_string(),
            ),
            _ => None,
        };

        Ok(Command { action, key, val })
    }
}

fn main() {
    let mut input = String::new();
    // let mut store: KeyValueStore<HashMapBackend<String, String>, String, String> =
    //     KeyValueStore::new("default.json");
    let mut store: KeyValueStore<SimpleLogBackend<String, String>, String, String> =
        KeyValueStore::new("default.json");
    println!("Enter a command:");
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
                store
                    .insert(command.key.clone().unwrap(), command.val.clone().unwrap())
                    .expect("Failed to write to disk");
                println!(
                    "inserted key: {}, value: {}",
                    command.key.unwrap(),
                    command.val.unwrap()
                );
            }
            Action::Get => {
                if let Some(value) = store.get(&command.key.unwrap()) {
                    let val = value;
                    println!("Value: {}", val);
                } else {
                    println!("Failed to get value");
                    continue;
                }
            }
            Action::Del => {
                store.remove(&command.key.clone().unwrap());
                println!("Deleted: {}", &command.key.clone().unwrap())
            }
            Action::Load => {
                store = KeyValueStore::new(&command.key.unwrap());
                println!("loaded!");
            }
            Action::Exit => break,
        };
    }

    store.flush().expect("Failed to flush data to disk.");
    println!("saved")
}
