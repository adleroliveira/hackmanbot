#[derive(Debug)]
pub enum Action {
	Character,
	Move
}

impl Action {
	pub fn new() -> Action {
		Action::Character
	}
}