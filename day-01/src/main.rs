// Advent of Code 2025: Day 1
// https://adventofcode.com/2025/day/1
// Usage: `cargo run <input-file>

use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");
    let mut dial = Dial::new();

    input.trim().split('\n').for_each(|line| {
        let (direction, magnitude_str) = line.split_at(1);
        let magnitude: isize = magnitude_str.parse().expect("input is a valid int");
        let amount = if direction == "L" {
            -magnitude
        } else {
            magnitude
        };

        dial.turn(amount);
    });

    println!(
        "The number of zeros landed upon was: {}",
        dial.zeros_landed_upon
    );
    println!("The number of zeros seen was: {}", dial.zeros_seen);
}

pub struct Dial {
    val: isize,
    zeros_seen: isize,
    zeros_landed_upon: isize,
}

impl Dial {
    fn new() -> Self {
        Dial {
            val: 50,
            zeros_seen: 0,
            zeros_landed_upon: 0,
        }
    }

    pub fn turn(&mut self, amount: isize) {
        // start by counting how many full rotations we would do, which would
        // generally pass by 0, unless you start and end on 0 which we will handle later
        let full_rotations = amount.abs() / 100;
        self.zeros_seen += full_rotations;

        // this is what is left over after any full rotations
        let extra = amount % 100;

        // handle the case where you're starting and ending on 0 because the final
        // zero will be counted at the end and we don't want to duplicate it
        if self.val == 0 && extra == 0 && full_rotations > 0 {
            self.zeros_seen -= 1;
        }

        let sum = self.val + extra;

        // if we started at 0 then anything less than 100 would not pass 0 again
        // but from any other starting value we could pass 0 again if the sum goes
        // negative, or over 100
        if self.val != 0 {
            if sum > 100 || sum < 0 {
                self.zeros_seen += 1;
            }
        }

        // this is the amount we actually want to rotate the dial ignoring full rotations
        let remainder = sum % 100;

        self.val = if remainder < 0 {
            remainder + 100
        } else {
            remainder
        };

        if self.val == 0 {
            self.zeros_landed_upon += 1;
            self.zeros_seen += 1;
        }
    }

    pub fn reset(&mut self, value: isize) {
        self.val = value;
        self.zeros_seen = 0;
        self.zeros_landed_upon = 0;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_turn() {
        let mut dial = Dial::new();
        let tests: [(isize, isize, isize, isize, isize); 10] = [
            (50, 50, 0, 1, 1),   // starting non-zero, landing on zero, no extra rotations
            (50, 150, 0, 2, 1),  // starting non-zero. landing on zero, one extra rotation
            (0, 100, 0, 1, 1),   // starting on zero, landing on zero, exactly one rotation
            (0, -100, 0, 1, 1),  // negative the same scenario
            (99, 2, 1, 1, 0),    // crossed zero so one zero seen
            (1, -2, 99, 1, 0),   // crossed zero so one zero seen
            (99, 102, 1, 2, 0),  // one full rotation plus crossed zero
            (1, -102, 99, 2, 0), // one full rotation plus crossed zero
            (0, 500, 0, 5, 1),   // five full rotations but starting and ending on zero
            (0, -501, 99, 5, 0), // five full rotations plus one more
        ];

        for (start, amount, end, zeros_seen, zeros_landed_upon) in tests {
            dial.reset(start);
            dial.turn(amount);
            assert_eq!(dial.val, end);
            assert_eq!(dial.zeros_seen, zeros_seen);
            assert_eq!(dial.zeros_landed_upon, zeros_landed_upon);
        }
    }
}
