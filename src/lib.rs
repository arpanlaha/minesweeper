pub struct Board {
    tiles: Vec<Tile>,
    turn: usize,
    width: usize,
    height: usize,
    mines: usize,
    active_mines: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            tiles: vec![],
            turn: 0,
            width,
            height,
            mines,
            active_mines: mines,
        }
    }
}

pub struct Tile {
    status: TileStatus,
    value: TileValue,
}

impl Tile {
    pub fn status(&self) -> TileStatus {
        self.status
    }

    pub fn value(&self) -> Option<TileValue> {
        match self.status {
            TileStatus::Open => Some(self.value),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TileStatus {
    Blank,
    Flagged,
    Open,
}
#[derive(Clone, Copy, Debug)]
pub enum TileValue {
    Mine,
    Neighbored(usize),
}
