// Advent of Code 2025: Day 7
// https://adventofcode.com/2025/day/7
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let mut grid = Grid::new(&input);

    for col in 0isize..grid.cols as isize {
        if grid.get(0, col).unwrap() == b'S' {
            grid.set(1, col, b'|');
        }
    }

    let mut splits = 0;

    for row in 1isize..grid.rows as isize - 2 {
        for col in 0isize..grid.cols as isize {
            if grid.get(row, col).unwrap() == b'|' {
                if grid.get(row + 1, col).unwrap() == b'^' {
                    grid.set(row + 2, col - 1, b'|');
                    grid.set(row + 2, col + 1, b'|');
                    splits += 1;
                } else {
                    grid.set(row + 2, col, b'|');
                }
            }
        }
    }

    println!("The number of splits was {splits}");
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

    fn get(&self, row: isize, col: isize) -> Option<u8> {
        if let Some(index) = self.coord_to_index(row, col) {
            Some(self.data[index])
        } else {
            None
        }
    }

    fn set(&mut self, row: isize, col: isize, val: u8) {
        if let Some(index) = self.coord_to_index(row, col) {
            self.data[index] = val;
        }
    }

    fn coord_to_index(&self, row: isize, col: isize) -> Option<usize> {
        if row < 0 || row > self.rows as isize - 1 || col < 0 || col > self.cols as isize - 1 {
            return None;
        }
        Some(row as usize * self.cols + col as usize)
    }
}
