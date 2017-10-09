use bug::Bug;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: HashMap<(i32, i32), Cell>,
    pub snippets: Vec<(i32, i32)>,
    pub player_position: (i32, i32),
}

impl GameState {
    pub fn new(state: &str, width: i32, height: i32, bot_id: usize) -> GameState {
        let mut state_iterator = state.split(',');
        let mut board: HashMap<(i32, i32), Cell> = HashMap::new();
        let mut snippets = Vec::new();
        let mut player_position = (0, 0);
        let bot_id = format!("P{}", bot_id);

        for y in 0..height {
            for x in 0..width {

                let cell_str = state_iterator.next().expect("Invalid matrix representation");
                let cell = Cell::new(cell_str, y, x, width, height);

                if cell_str.contains("C") {
                    snippets.push((y, x));
                }

                if cell_str.contains(&bot_id) {
                    player_position = (y, x);
                }

                board.insert((cell.position.0, cell.position.1), cell);
            }
        }

        GameState { board, snippets, player_position }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Entity {
    Snippet,
    Player(usize),
    Spawn(usize),
    Gate(String),
    Bug(Bug),
    Mine(usize),
}

impl Entity {
    fn new(entity: &str) -> Option<Entity> {
        match entity {
            value if value.contains("P") => value.chars().nth(1).map(|pid| Entity::Player(pid.to_string().parse().unwrap())),
            value if value.contains("S") => value.chars().nth(1).map(|due| Entity::Spawn(due.to_string().parse().unwrap())),
            value if value.contains("G") => value.chars().nth(1).map(|dir| Entity::Gate(dir.to_string().to_string())),
            value if value.contains("E") => value.chars().nth(1).map(|typ| Entity::Bug(Bug::new(&typ.to_string()).unwrap())),
            value if value.contains("B") => value.chars().nth(1).map(|due| Entity::Mine(due.to_string().parse().unwrap())),
            value if value.contains("C") => Some(Entity::Snippet),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Move{
    pub position: (i32, i32),
    pub action: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    Bloqued,
    Free
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub position: (i32, i32),
    pub cell_type: CellType,
    pub entities: Option<Vec<Entity>>,
    pub neighbours: HashMap<(i32, i32), &'static str>,
}

impl Cell {
    fn new(cell: &str, y: i32, x: i32, w: i32, h: i32) -> Cell {
        let mut neighbours = HashMap::new();

        if y-1 >= 0 {
            neighbours.insert((y-1, x), "up");
        }

        if y + 1 < h {
            neighbours.insert((y+1, x), "down");
        }

        if x-1 >= 0 {
            neighbours.insert((y, x-1), "left");
        }

        if x+1 < w {
            neighbours.insert((y, x+1), "right");
        }

        if cell.contains("Gl") {
            neighbours.insert((y, w-1), "left");
        }

        if cell.contains("Gr") {
            neighbours.insert((y, 0), "right");
        }


        if cell.contains("x") {
            Cell {
                position: (y, x),
                cell_type: CellType::Bloqued,
                entities: None,
                neighbours,
            }
        } else {
            let entities = if cell.contains(";") {
                Some(cell.split(";").flat_map(|s| Entity::new(s)).collect())
            } else {
                Entity::new(cell).map(|e| vec![e])
            };

            Cell {
                position: (y, x),
                cell_type: CellType::Free,
                entities,
                neighbours,
            }
        }
    }

    pub fn moves(&self, state: &GameState) -> Vec<Move>  {
        let mut moves = Vec::new();
        for (pos, action) in &self.neighbours {
            let cell = state.board.get(&pos);
            if let Some(cell) = cell {
                if cell.is_free() {
                    moves.push(Move{
                        position: cell.position,
                        action
                    });
                }
            }
        }
        moves
    }

    pub fn is_free(&self) -> bool {
        match self.cell_type {
            CellType::Bloqued => false,
            CellType::Free => true,
        }
    }
}