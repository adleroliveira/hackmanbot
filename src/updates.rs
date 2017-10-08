#[derive(Debug)]
pub enum Update {
    GameRound(usize),
    GameField(String),
    PlayerSnippets((String, usize)),
    PlayerBombs((String, usize)),
}

impl Update {
    pub fn new(
        entity: Option<&str>,
        feature: Option<&str>,
        value: Option<&str>,
    ) -> Result<Update, &'static str> {

        let feature = feature.expect("Malformed update");
        let value = value.expect("Malformed update");

        match entity {
            Some("game") => update_game(feature, value),
            Some(player) => update_player(player, feature, value),
            None => Err("Unknown update"),
        }
    }
}

fn update_game(feature: &str, value: &str) -> Result<Update, &'static str> {
    match feature {
        "round" => {
            let value = value.parse().expect("Game round must be integer");
            Ok(Update::GameRound(value))
        }
        "field" => {
            let value = value.to_string();
            Ok(Update::GameField(value))
        }
        _ => Err("Unknown game feature"),
    }
}

fn update_player(player: &str, feature: &str, value: &str) -> Result<Update, &'static str> {
    match feature {
        "snippets" => {
            let value = value.parse().expect("Snippets must be integer");
            Ok(Update::PlayerSnippets((player.to_string(), value)))
        }
        "bombs" => {
            let value = value.parse().expect("Bombs must be integer");
            Ok(Update::PlayerSnippets((player.to_string(), value)))
        }
        _ => Err("Unknown player feature"),
    }
}