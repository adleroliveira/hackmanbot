#[derive(Debug)]
pub enum Setting {
    Timebank(usize),
    TimePerMove(usize),
    PlayerNames(String),
    YourBot(String),
    YourBotID(usize),
    FieldWidth(usize),
    FieldHeight(usize),
    MaxRounds(usize),
}

impl Setting {
    pub fn new(
        setting_name: Option<&str>,
        setting_value: Option<&str>,
    ) -> Result<Setting, &'static str> {
        let value = setting_value.unwrap();

        match setting_name {
            Some("timebank") => {
                let value = value.parse().expect("Timebank must be integer");
                Ok(Setting::Timebank(value))
            }

            Some("time_per_move") => {
                let value = value.parse().expect("Time per move must be integer");
                Ok(Setting::TimePerMove(value))
            }

            Some("player_names") => {
                let value = value.to_string();
                Ok(Setting::PlayerNames(value))
            }

            Some("your_bot") => {
                let value = value.to_string();
                Ok(Setting::YourBot(value))
            }

            Some("your_botid") => {
                let value = value.parse().expect("BotID must be integer");
                Ok(Setting::YourBotID(value))
            }

            Some("field_width") => {
                let value = value.parse().expect("Field Width must be integer");
                Ok(Setting::FieldWidth(value))
            }

            Some("field_height") => {
                let value = value.parse().expect("Field Height must be integer");
                Ok(Setting::FieldHeight(value))
            }

            Some("max_rounds") => {
                let value = value.parse().expect("MaxRounds must be integer");
                Ok(Setting::MaxRounds(value))
            }

            Some(_) => Err("Unrecognizible setting"),
            None => Err("Invalid setting"),
        }
    }
}