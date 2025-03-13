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
