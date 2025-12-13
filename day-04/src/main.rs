// Advent of Code 2025: Day 4
// https://adventofcode.com/2025/day/4
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let mut grid = Grid::new(&input);

    let mut total_moved = 0;
    let mut moved = move_rolls(&mut grid);

    println!("The number of rolls moved in the first round were: {moved}");

    while moved > 0 {
        total_moved += moved;
        moved = move_rolls(&mut grid);
    }

    println!("The total number of paper rolls that were moved were: {total_moved}");
}

fn move_rolls(grid: &mut Grid) -> usize {
    let movable_indices: Vec<usize> = (0..grid.data.len())
        .filter(|i| {
            let adjacent_rolls = grid
                .get_adjacent_indices_matching(*i, b'@')
                .iter()
                .flatten()
                .count();

            grid.data[*i] == b'@' && adjacent_rolls < 4
        })
        .collect();
    let moved = movable_indices.len();
    for i in movable_indices {
        grid.data[i] = b'x';
    }
    moved
}

struct Grid {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cols = input.as_bytes().iter().position(|c| *c == b'\n').unwrap();
        let data = input.replace("\n", "").into_bytes();
        let rows = data.len() / cols;
        Self { data, rows, cols }
    }

    fn get_adjacent_indices_matching(&self, index: usize, test_val: u8) -> [Option<usize>; 8] {
        let (row, col) = (index / self.cols, index % self.cols);

        self.adjacent_indices(row as isize, col as isize)
            .map(|opt| match opt {
                Some(index) if self.data[index] == test_val => Some(index),
                _ => None,
            })
    }

    fn coord_to_index(&self, row: isize, col: isize) -> Option<usize> {
        if row < 0 || row > self.rows as isize - 1 || col < 0 || col > self.cols as isize - 1 {
            return None;
        }
        Some(row as usize * self.cols + col as usize)
    }

    fn adjacent_indices(&self, row: isize, col: isize) -> [Option<usize>; 8] {
        [
            self.coord_to_index(row - 1, col),     // N
            self.coord_to_index(row - 1, col + 1), // NE
            self.coord_to_index(row, col + 1),     // E
            self.coord_to_index(row + 1, col + 1), // SE
            self.coord_to_index(row + 1, col),     // S
            self.coord_to_index(row + 1, col - 1), // SW
            self.coord_to_index(row, col - 1),     // W
            self.coord_to_index(row - 1, col - 1), // NW
        ]
    }
}
