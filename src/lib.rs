#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use rand::seq::index;
pub struct Board {
    tiles: Vec<Tile>,
    turn: usize,
    width: usize,
    height: usize,
    mines: usize,
    active_mines: usize,
}

impl Board {
    #[must_use]
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        let length = width * height;

        let mut tiles = vec![Tile::new(TileValue::Neighbored(0)); length];

        let mut rng = rand::thread_rng();

        for index in index::sample(&mut rng, length, mines) {
            tiles[index] = Tile::new(TileValue::Mine);
        }

        Self {
            tiles,
            turn: 0,
            width,
            height,
            mines,
            active_mines: mines,
        }
    }

    #[must_use]
    const fn get_coord(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(x + y * self.width)
        }
    }

    // TODO: convert to result
    #[must_use]
    pub fn tile(&self, x: usize, y: usize) -> Option<Tile> {
        let coord = self.get_coord(x, y)?;

        Some(self.tiles[coord])
    }

    // TODO: convert to result
    pub fn flag(&mut self, x: usize, y: usize) -> Option<()> {
        let coord = self.get_coord(x, y)?;

        let tile = &mut self.tiles[coord];

        if tile.status == TileStatus::Flagged {
            None
        } else {
            tile.status = TileStatus::Flagged;
            self.turn += 1;
            self.active_mines -= 1;
            Some(())
        }
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub const fn turn(&self) -> usize {
        self.turn
    }

    #[must_use]
    pub const fn mines(&self) -> usize {
        self.mines
    }

    #[must_use]
    pub const fn active_mines(&self) -> usize {
        self.active_mines
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    status: TileStatus,
    value: TileValue,
}

impl Tile {
    #[must_use]
    pub const fn new(value: TileValue) -> Self {
        Self {
            status: TileStatus::Blank,
            value,
        }
    }

    #[must_use]
    pub const fn status(&self) -> TileStatus {
        self.status
    }

    #[must_use]
    pub const fn value(&self) -> Option<TileValue> {
        match self.status {
            TileStatus::Open => Some(self.value),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileStatus {
    Blank,
    Flagged,
    Open,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileValue {
    Mine,
    Neighbored(usize),
}
