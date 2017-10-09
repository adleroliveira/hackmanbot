use game_state::GameState;

#[derive(Debug)]
pub enum Strategy {
    BFS
}

impl Strategy {
    pub fn run(&self, state: Option<GameState>) {
        let state = state.expect("Ivalid game state");
        match self {
            &Strategy::BFS => bfs::run(state),
        }
    }
}

// While the competition is still going on I will add all strategies to
// the .gitignore file but as soon as the competition ends I will open
// Source all the strategies I made regardles of my final position.

// pub mod bfs;