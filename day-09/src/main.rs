// Advent of Code 2025: Day 9
// https://adventofcode.com/2025/day/9
// Usage: `cargo run <input-file>

use std::ops::RangeInclusive;
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

    let mut lines = vec![];

    for i in 0..points.len() - 2 {
        lines.push(Line::new(&points[i], &points[i + 1]));
    }
    lines.push(Line::new(&points[points.len() - 1], &points[0]));

    let mut part_1: usize = 0;
    let mut part_2: usize = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = Rect::new(&points[i], &points[j]);
            let area = rect.area();
            if area > part_1 {
                part_1 = area;
            }
            if area > part_2 && !lines.iter().any(|line| rect.is_broken_by(&line)) {
                part_2 = area;
            }
        }
    }

    println!("The largest area of a rectangle between any two red tiles is {part_1}");
    println!("The largest area of a rectangle enclosed by the loop is {part_2}");
}

struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').ok_or(ParsePointError)?;

        let x = x_str.parse::<isize>().map_err(|_| ParsePointError)?;
        let y = y_str.parse::<isize>().map_err(|_| ParsePointError)?;

        Ok(Self { x, y })
    }
}

struct Rect {
    range_x: RangeInclusive<isize>,
    range_y: RangeInclusive<isize>,
    interior_range_x: RangeInclusive<isize>,
    interior_range_y: RangeInclusive<isize>,
}

impl Rect {
    fn new(corner_a: &Point, corner_b: &Point) -> Self {
        let range_x = corner_a.x.min(corner_b.x)..=corner_a.x.max(corner_b.x);
        let range_y = corner_a.y.min(corner_b.y)..=corner_a.y.max(corner_a.y);
        let interior_range_x = (range_x.start() + 1)..=(range_x.end() - 1);
        let interior_range_y = (range_y.start() + 1)..=(range_y.end() - 1);
        Self {
            range_x,
            range_y,
            interior_range_x,
            interior_range_y,
        }
    }

    fn area(&self) -> usize {
        ((self.range_x.end() - self.range_x.start() + 1).abs()
            * (self.range_y.end() - self.range_y.start() + 1).abs()) as usize
    }

    // does the given line go into the interior of the rect at all?
    fn is_broken_by(&self, line: &Line) -> bool {
        if line.point_a.x == line.point_b.x {
            let min_y = line.point_a.y.min(line.point_b.y);
            let max_y = line.point_a.y.max(line.point_b.y);

            self.interior_range_x.contains(&line.point_a.x)
                && *self.interior_range_y.start() <= max_y
                && min_y <= *self.interior_range_y.end()
        } else {
            let min_x = line.point_a.x.min(line.point_b.x);
            let max_x = line.point_a.x.max(line.point_b.x);

            self.interior_range_y.contains(&line.point_a.y)
                && *self.interior_range_x.start() <= max_x
                && min_x <= *self.interior_range_x.end()
        }
    }
}

struct Line<'a> {
    point_a: &'a Point,
    point_b: &'a Point,
}

impl<'a> Line<'a> {
    fn new(point_a: &'a Point, point_b: &'a Point) -> Self {
        Self { point_a, point_b }
    }
}
