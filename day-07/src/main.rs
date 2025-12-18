// Advent of Code 2025: Day 7
// https://adventofcode.com/2025/day/7
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut grid = Grid::of_u8_with_count(&input);
    let mut splits = 0;

    for col in 0isize..grid.cols as isize {
        if grid.get(0, col).unwrap().val == b'S' {
            grid.set(1, col, ValWithCount::new(b'|', 1));
            break;
        }
    }

    for row in 1isize..grid.rows as isize - 2 {
        for col in 0isize..grid.cols as isize {
            let cell = grid.get(row, col).unwrap();
            if cell.val == b'|' {
                match grid.get(row + 1, col).unwrap() {
                    ValWithCount {
                        val: b'^',
                        count: _,
                    } => {
                        grid.set(
                            row + 2,
                            col - 1,
                            ValWithCount::new(
                                b'|',
                                grid.get(row + 2, col - 1).unwrap().count + cell.count,
                            ),
                        );
                        grid.set(
                            row + 2,
                            col + 1,
                            ValWithCount::new(
                                b'|',
                                grid.get(row + 2, col + 1).unwrap().count + cell.count,
                            ),
                        );
                        splits += 1
                    }
                    _ => {
                        grid.set(
                            row + 2,
                            col,
                            ValWithCount::new(
                                b'|',
                                grid.get(row + 2, col).unwrap().count + cell.count,
                            ),
                        );
                    }
                }
            }
        }
    }

    let timelines: usize = (0..grid.cols as isize)
        .map(|col| grid.get(grid.rows as isize - 1, col).unwrap().count)
        .sum();

    println!("The number of splits was {splits}");
    println!("The number of timelines were {timelines}");
}

struct Grid<T>
where
    T: Copy,
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ValWithCount {
    val: u8,
    count: usize,
}

impl ValWithCount {
    fn new(val: u8, count: usize) -> Self {
        Self { val, count }
    }
}

impl Grid<ValWithCount> {
    fn of_u8_with_count(input: &str) -> Self {
        let cols = input.as_bytes().iter().position(|c| *c == b'\n').unwrap();
        let data: Vec<ValWithCount> = input
            .replace("\n", "")
            .as_bytes()
            .iter()
            .map(|&b| ValWithCount::new(b, 0))
            .collect();
        let rows = data.len() / cols;
        Self { data, rows, cols }
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    fn get(&self, row: isize, col: isize) -> Option<T> {
        if let Some(index) = self.coord_to_index(row, col) {
            Some(self.data[index])
        } else {
            None
        }
    }

    fn set(&mut self, row: isize, col: isize, val: T) {
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
