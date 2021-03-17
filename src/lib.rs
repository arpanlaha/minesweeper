#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::collections::HashSet;

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

        let mut rng = rand::thread_rng();

        let mut tiles: Vec<Tile> = Vec::with_capacity(length);

        let mine_indices: HashSet<usize> = index::sample(&mut rng, length, mines).iter().collect();

        for index in 0..length {
            if mine_indices.contains(&index) {
                tiles.push(Tile::new(TileValue::Mine));
            } else {
                tiles.push(Tile::new(TileValue::Neighbored(num_mine_neighbors(
                    index,
                    width,
                    height,
                    &mine_indices,
                ))));
            }
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

fn num_mine_neighbors(
    index: usize,
    width: usize,
    height: usize,
    mine_indices: &HashSet<usize>,
) -> usize {
    let x = index % width;
    let y = index / width;

    let has_left = x == 0;
    let has_upper = y == 0;
    let has_right = x == width - 1;
    let has_lower = y == height - 1;

    let mut mine_neighbors = 0;

    if has_left && mine_indices.contains(&(index - 1)) {
        mine_neighbors += 1;
    }

    if has_right && mine_indices.contains(&(index + 1)) {
        mine_neighbors += 1;
    }

    if has_upper && mine_indices.contains(&(index - width)) {
        mine_neighbors += 1;
    }

    if has_lower && mine_indices.contains(&(index + width)) {
        mine_neighbors += 1;
    }

    if has_left && has_upper && mine_indices.contains(&(index - 1 - width)) {
        mine_neighbors += 1;
    }

    if has_left && has_lower && mine_indices.contains(&(index - 1 + width)) {
        mine_neighbors += 1;
    }

    if has_right && has_upper && mine_indices.contains(&(index + 1 - width)) {
        mine_neighbors += 1;
    }

    if has_right && has_lower && mine_indices.contains(&(index + 1 + width)) {
        mine_neighbors += 1;
    }

    mine_neighbors
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
