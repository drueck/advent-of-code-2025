// Advent of Code 2025: Day 6
// https://adventofcode.com/2025/day/6
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let maths: Vec<Vec<&str>> = input
        .trim()
        .split('\n')
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let operator_index = maths.len() - 1;

    let part_1: usize = (0..maths[0].len())
        .map(|col| {
            let numbers = (0..operator_index).map(|row| maths[row][col].parse::<usize>().unwrap());
            match maths[operator_index][col] {
                "*" => numbers.product::<usize>(),
                _ => numbers.sum(),
            }
        })
        .sum();

    println!("The answer to part 1 is: {part_1}");
}
