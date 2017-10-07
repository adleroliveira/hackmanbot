#[derive(Debug)]
pub enum Action {
    Character(usize),
    Move(usize),
}

impl Action {
    pub fn new(action_type: Option<&str>, time: Option<&str>) -> Result<Action, &'static str> {
        let time = time.unwrap().parse().expect("Action time must be integer");
        match action_type {
            Some("character") => Ok(Action::Character(time)),
            Some("move") => Ok(Action::Move(time)),
            Some(_) => Err("Unknown Action"),
            None => Err("Invalid Action"),
        }
    }
}