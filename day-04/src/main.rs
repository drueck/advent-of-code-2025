// Advent of Code 2025: Day 4
// https://adventofcode.com/2025/day/4
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let grid = Grid::new(&input);

    let movable = (0..grid.data.len())
        .filter(|i| {
            let adjacent_rolls = grid
                .get_adjacent_indices_matching(*i, '@' as u8)
                .iter()
                .flatten()
                .count();

            grid.data[*i] == '@' as u8 && adjacent_rolls < 4
        })
        .count();

    println!(
        "The total number of forklift-accessible paper rolls initially is {}",
        movable
    );
}

struct Grid {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cols = input
            .as_bytes()
            .iter()
            .position(|c| *c == '\n' as u8)
            .unwrap();

        let data = input.replace("\n", "").into_bytes();

        let rows = data.len() / cols;

        assert_eq!(data.len() % rows, 0);

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
