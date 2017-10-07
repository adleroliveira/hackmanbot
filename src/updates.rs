#[derive(Debug)]
pub enum Update {
	GameRound(usize),
	GameField(String),
	PlayerSnippets(usize),
	PlayerBombs(usize),
}

impl Update {
	pub fn new(
		entity: Option<&str>,
		feature: Option<&str>,
		value: Option<&str>
	) -> Result<Update, &'static str> {

		let feature = feature.unwrap();
		let value = value.unwrap();

		match entity {
		    Some("game") 	=> update_game(feature, value),
		    Some(player) 	=> update_player(player, feature, value),
		    None 			=> Err("not implemented"),
		}
	}
}

fn update_game(feature: &str, value: &str) -> Result<Update, &'static str> {
	match feature {
	    "round" => {
	    	let value = value.parse().expect("Timebank must be integer");
	    	Ok(Update::GameRound(value))
	    }
	    "field" => {
	    	let value = value.to_string();
	    	Ok(Update::GameField(value))
	    }
	    _ => Err("Game feature update not recognizable"),
	}
}

fn update_player(player: &str, feature: &str, value: &str) -> Result<Update, &'static str> {
	println!("{:?}", player);
	println!("{:?}", feature);
	println!("{:?}", value);
	Err("Not implemented")
}