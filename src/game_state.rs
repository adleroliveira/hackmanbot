use bug::Bug;

#[derive(Debug)]
pub struct GameState {
    map: Vec<Cell>
}

#[derive(Debug)]
enum Entity {
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
            value if value.contains("P") => value.get(1..1).map(|pid| Entity::Player(pid.parse().unwrap())),
            value if value.contains("S") => value.get(1..1).map(|due| Entity::Spawn(due.parse().unwrap())),
            value if value.contains("G") => value.get(1..1).map(|dir| Entity::Gate(dir.to_string())),
            value if value.contains("E") => value.get(1..1).map(|typ| Entity::Bug(Bug::new(typ))),
            value if value.contains("B") => value.get(1..1).map(|due| Entity::Mine(due.parse().unwrap())),
            value if value.contains("C") => Some(Entity::Snippet),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum CellType {
    Bloqued,
    Free
}

#[derive(Debug)]
struct Cell {
    position: (i32, i32),
    cell_type: CellType,
    entities: Option<Vec<Entity>>,
}

impl Cell {
    fn new(cell: &str, x: i32, y: i32) -> Cell {
        if cell.contains("x") {
            Cell {
                position: (x, y),
                cell_type: CellType::Bloqued,
                entities: None,
            }
        } else {
            let entities = if cell.contains(";") {
                Some(cell.split(";").flat_map(|s| Entity::new(s)).collect())
            } else {
                Entity::new(cell).map(|e| vec![e])
            };

            Cell {
                position: (x, y),
                cell_type: CellType::Free,
                entities,
            }
        }
    }
}

impl GameState {
    pub fn new(state: &str, width: i32, height: i32) -> GameState {
        let mut state_iterator = state.split(',');
        let mut map = Vec::new();

        for x in 0..width {
            for y in 0..height {
                let cell_str = state_iterator.next().expect("Invalid matrix representation");
                map.push(Cell::new(cell_str, x, y));
            }
        }

        GameState { map }
    }
}