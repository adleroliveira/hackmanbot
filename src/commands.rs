use settings::Setting;
use updates::Update;
use action::Action;

#[derive(Debug)]
pub enum InputCommand {
	Setting(Setting),
	Update(Update),
	Action(Action),
}

impl InputCommand {
	pub fn new(cmd: &str) -> Result<InputCommand, &'static str> {
		let cmd_str = cmd.to_string();
		let mut parts = cmd_str.split_whitespace();
		match parts.next() {
		    Some("settings") => {
		    	let setting = Setting::new(parts.next(), parts.next());
		    	Ok(InputCommand::Setting(setting.unwrap()))
		    }
		    Some("update") => {
		    	let update = Update::new(parts.next(), parts.next(), parts.next());
		    	Ok(InputCommand::Update(update.unwrap()))
		    }
		    Some("action") => {
		    	let action = Action::new();
		    	Ok(InputCommand::Action(action))
		    }
		    _ => Err("Command not found")
		}
	}
}