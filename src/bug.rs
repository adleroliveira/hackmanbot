#[derive(Debug)]
pub enum Bug {
    Chase,
    Predict,
    Lever,
    FarChase,
    Unknown
}

impl Bug {
    pub fn new(bug_type: &str) -> Bug {
        match bug_type {
            "0" => Bug::Chase,
            "1" => Bug::Predict,
            "2" => Bug::Lever,
            "3" => Bug::FarChase,
            _ => Bug::Unknown,
        }
    }
}