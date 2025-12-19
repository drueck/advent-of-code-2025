// Advent of Code 2025: Day 8
// https://adventofcode.com/2025/day/8
// Usage: `cargo run <input-file>

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::{env, fs};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let max_pairs: usize = env::args()
        .nth(2)
        .expect("please specify the number of pairs to connet")
        .parse()
        .expect("please specify the number of pairs as an integer");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let boxes: Vec<Point> = input
        .trim()
        .split('\n')
        .map(|line| {
            let mut coords = line.split(',').map(|coord| coord.parse::<isize>().unwrap());
            Point::new(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect();

    let mut combinations: BinaryHeap<PointPair> = BinaryHeap::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            combinations.push(PointPair::new(
                &boxes[i],
                &boxes[j],
                boxes[i].distance_to(&boxes[j]),
            ));
            if combinations.len() > max_pairs {
                combinations.pop();
            }
        }
    }

    let combos = combinations.into_sorted_vec();
    let mut circuits: Vec<HashSet<&Point>> = vec![];

    for pair in &combos[..] {
        let mut new_circuits: Vec<HashSet<&Point>> = vec![];
        let mut new_circuit: HashSet<&Point> = HashSet::from([pair.a, pair.b]);
        for i in 0..circuits.len() {
            if circuits[i].is_disjoint(&new_circuit) {
                new_circuits.push(circuits[i].clone());
            } else {
                for point in &circuits[i] {
                    new_circuit.insert(point.clone());
                }
            }
        }
        new_circuits.push(new_circuit);
        circuits = new_circuits
    }

    circuits.sort_unstable_by_key(|circuit| Reverse(circuit.len()));

    let part_1: usize = circuits[..3].iter().map(|circuit| circuit.len()).product();

    println!("The sum of the three largest circuits was {part_1}");
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Point) -> usize {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt() as usize
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PointPair<'a> {
    a: &'a Point,
    b: &'a Point,
    dist: usize,
}

impl<'a> PointPair<'a> {
    fn new(a: &'a Point, b: &'a Point, dist: usize) -> Self {
        Self { a, b, dist }
    }
}

impl<'a> Ord for PointPair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl<'a> PartialOrd for PointPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
