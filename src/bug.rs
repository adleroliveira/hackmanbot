#[derive(Debug, Clone, PartialEq)]
pub enum Bug {
    Chase,
    Predict,
    Lever,
    FarChase,
    Unknown
}

impl Bug {
    pub fn new(bug_type: &str) -> Result<Bug, &'static str> {
        match bug_type {
            "0" => Ok(Bug::Chase),
            "1" => Ok(Bug::Predict),
            "2" => Ok(Bug::Lever),
            "3" => Ok(Bug::FarChase),
            _ => Err("Unknown bug type"),
        }
    }
}