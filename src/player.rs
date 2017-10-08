#[derive(Debug)]
pub struct Player {
  id: Option<usize>,
  pub name: String,
  pub bombs: usize,
  pub snippets: usize,
}

impl Player {
  pub fn new(name: String) -> Player {
    Player {
      id: None,
      name,
      bombs: 0,
      snippets: 0
    }
  }

  pub fn set_id(&mut self, id: usize) {
    self.id = Some(id);
  }

  pub fn is(&self, name :&str) -> bool {
    self.name == name
  }
}