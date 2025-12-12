// Advent of Code 2025: Day 4
// https://adventofcode.com/2025/day/4
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let grid = Grid::new(&input);

    let mut movable: usize = 0;

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            if grid.val(row as isize, col as isize) == Some('@' as u8)
                && grid.count_adjacent(row as isize, col as isize, '@' as u8) < 4
            {
                movable += 1;
            }
        }
    }

    println!(
        "The total number of forklift-accessible paper rolls is {}",
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

    fn val(&self, row: isize, col: isize) -> Option<u8> {
        if row < 0 || row > self.rows as isize - 1 || col < 0 || col > self.cols as isize - 1 {
            return None;
        }
        Some(self.data[row as usize * self.cols + col as usize])
    }

    fn count_adjacent(&self, row: isize, col: isize, test_val: u8) -> usize {
        [
            self.val(row - 1, col),     // N
            self.val(row - 1, col + 1), // NE
            self.val(row, col + 1),     // E
            self.val(row + 1, col + 1), // SE
            self.val(row + 1, col),     // S
            self.val(row + 1, col - 1), // SW
            self.val(row, col - 1),     // W
            self.val(row - 1, col - 1), // NW
        ]
        .iter()
        .filter_map(|opt| match opt {
            Some(val) => Some((*val == test_val) as usize),
            None => None,
        })
        .sum()
    }
}
