// Advent of Code 2025: Day 1
// https://adventofcode.com/2025/day/1
// Usage: `cargo run <input-file>
//
use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let mut val: isize = 50;
    let mut zeros: usize = 0;

    input.trim().split('\n').for_each(|line| {
        let (direction, magnitude_str) = line.split_at(1);
        let magnitude: isize = magnitude_str.parse().expect("input is a valid int");
        if direction == "L" {
            val -= magnitude;
        } else {
            val += magnitude;
        }
        val %= 100;
        if val < 0 {
            val += 100;
        }
        if val == 0 {
            zeros += 1;
        }
    });

    println!("The number of zeros was: {}", zeros);
}
