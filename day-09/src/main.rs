// Advent of Code 2025: Day 9
// https://adventofcode.com/2025/day/9
// Usage: `cargo run <input-file>

use std::str::FromStr;
use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let points: Vec<Point> = input
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect();

    let mut largest_area: usize = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = points[i].area_of_rect_with(&points[j]);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("The largest area was {largest_area}");
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn area_of_rect_with(&self, other: &Point) -> usize {
        ((self.x as isize - other.x as isize + 1).abs()
            * (self.y as isize - other.y as isize + 1).abs()) as usize
    }
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').ok_or(ParsePointError)?;

        let x = x_str.parse::<usize>().map_err(|_| ParsePointError)?;
        let y = y_str.parse::<usize>().map_err(|_| ParsePointError)?;

        Ok(Self { x, y })
    }
}
