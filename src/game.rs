use std::io;
use std::io::Stdin;
use std::error::Error;

use commands::InputCommand;
use settings::Setting;
use updates::Update;
use player::Player;
use action::Action;

const CHARACTER: &str = "bixie";

#[derive(Debug)]
enum GameStatus {
	New,
	Started,
	// Stopped
}

#[derive(Debug)]
pub struct Game {
	stdin: 			Stdin,
	character:		&'static str,
	status:			GameStatus,
	timebank: 		Option<usize>,
	time_per_move: 	Option<usize>,
	player_names: 	Option<String>,
	your_bot: 		Option<String>,
	your_botid: 	Option<usize>,
	field_width: 	Option<usize>,
	field_height: 	Option<usize>,
	max_rounds: 	Option<usize>,
	player:			Option<Player>,
	enemy:			Option<Player>,
	round: 			usize,
}

impl Game {
	pub fn new(stdin: io::Stdin) -> Game {
		Game {
			stdin,
			status:			GameStatus::New,
			character:		CHARACTER,
			timebank: 		None,
			time_per_move: 	None,
			player_names: 	None,
			your_bot: 		None,
			your_botid: 	None,
			field_width: 	None,
			field_height: 	None,
			max_rounds: 	None,
			player:			None,
			enemy:			None,
			round: 			0,
		}
	}

	pub fn start(&mut self) -> Result<(), Box<Error>> {
		loop {
			let mut command_str = String::new();
			self.stdin.read_line(&mut command_str).unwrap();
			self.parse_command(&command_str).unwrap();

			match self.status {
			    GameStatus::New => (),
			    GameStatus::Started => {
			    	if self.round == self.max_rounds.unwrap() {
			    		continue;
			    	}
			    }
			}
		}
	}

	fn parse_command(&mut self, cmd: &str) -> Result<(), &'static str> {
		let command = InputCommand::new(cmd).unwrap();
		match command {
		    InputCommand::Setting(val) 	=> self.update_setting(val),
		    InputCommand::Update(val) 	=> self.update_game_state(val),
		    InputCommand::Action(val) 	=> self.perform_action(val),
		}
		println!("{:#?}", self);
		Ok(())
	}

	fn update_setting(&mut self, setting: Setting) {
		match setting {
		    Setting::Timebank(val) 		=> self.timebank 		= Some(val),
		    Setting::TimePerMove(val) 	=> self.time_per_move 	= Some(val),
		    Setting::PlayerNames(val) 	=> self.player_names 	= Some(val),
		    Setting::YourBot(val) 		=> self.your_bot 		= Some(val),
		    Setting::YourBotID(val) 	=> self.your_botid 		= Some(val),
		    Setting::FieldWidth(val) 	=> self.field_width 	= Some(val),
		    Setting::FieldHeight(val) 	=> self.field_height 	= Some(val),
		    Setting::MaxRounds(val) 	=> self.max_rounds 		= Some(val),
		}
	}

	fn update_game_state(&mut self, update: Update) {
		match update {
		    Update::GameRound(val) 		=> self.round = val,
		    Update::GameField(_) 		=> (),
		    Update::PlayerSnippets(_) 	=> (),
		    Update::PlayerBombs(_) 		=> (),
		}
	}

	fn perform_action(&mut self, action: Action) {
		println!("action {:?}", action);
	}
}