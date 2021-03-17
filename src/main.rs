#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use minesweeper::Board;

fn main() {
    let board = Board::new(30, 16, 99);
    println!("Hello, world!");
}
