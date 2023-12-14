use crate::Direction::{East, North, South, West};
use crate::MapElem::{CubeRock, RoundedRock, Void};
use rust_base::aoc::Parser;
use std::fs::File;
use std::io::{BufReader, Lines};
use std::ops::Neg;

#[derive(Copy, Clone)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MapElem {
    RoundedRock,
    CubeRock,
    Void,
}

type Map = Vec<Vec<MapElem>>;

impl From<char> for MapElem {
    fn from(value: char) -> Self {
        match value {
            'O' => RoundedRock,
            '#' => CubeRock,
            '.' => Void,
            ch => panic!("cannot convert {} to element", ch),
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            North => North,
            West => East,
            South => South,
            East => West,
        }
    }
}

fn parse(lines: Lines<BufReader<File>>) -> Map {
    lines
        .flatten()
        .map(|l| l.chars().map(|c| c.into()).collect::<Vec<MapElem>>())
        .collect()
}

fn rotate(map: &Map, direction: Direction) -> Map {
    match direction {
        North => map.clone(),
        South => {
            let mut res: Map = Vec::new();

            for elems in map {
                res.push(elems.clone().into_iter().rev().collect());
            }
            res.reverse();
            res
        }
        East => {
            let mut res: Map = vec![Vec::new(); map[0].len()];
            for elems in map.iter().rev() {
                for (col, elem) in elems.iter().enumerate() {
                    res[col].push(*elem);
                }
            }
            res
        }
        West => {
            let mut res: Map = vec![Vec::new(); map[0].len()];
            for elems in map {
                for (col, elem) in elems.iter().enumerate() {
                    res[col].push(*elem);
                }
            }
            res.reverse();
            res
        }
    }
}

fn find_cost(map: &Map) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.iter().filter(|e| e == &&RoundedRock).count())
        .sum()
}

fn tilt(map: &Map) -> Map {
    let bound = map.len();
    let mut last_bounds = vec![bound; map[0].len()];

    let mut res: Map = vec![vec![Void; map[0].len()]; map.len()];

    for (row, elems) in map.iter().enumerate() {
        let row = bound - row;
        for (col, elem) in elems.iter().enumerate() {
            if *elem == CubeRock {
                last_bounds[col] = row - 1;
                res[row - 1][col] = CubeRock;
            }

            if *elem == RoundedRock {
                let moves_to = last_bounds[col];
                res[moves_to - 1][col] = RoundedRock;
                last_bounds[col] -= 1;
            }
        }
    }

    res.reverse();
    res
}

fn part1(map: &Map) -> usize {
    find_cost(&tilt(map))
}

fn part2(map: &Map) -> usize {
    const ITERS: usize = 1000000000;

    let mut iterations = Vec::new();
    let mut res: Map = map.clone();
    let mut costs = Vec::new();
    for i in 0..ITERS {
        for dir in [North, East, South, West] {
            res = rotate(&tilt(&rotate(&res, dir)), -dir);
        }

        costs.push(find_cost(&res));

        if let Some(first_occurrence) = iterations
            .iter()
            .enumerate()
            .find(|(_, state)| state == &&res)
            .map(|e| e.0)
        {
            let d = i - first_occurrence;
            let remaining_to_cycle = (ITERS - first_occurrence) % d;
            return costs[first_occurrence + remaining_to_cycle - 1];
        }

        iterations.push(res.clone());
    }

    panic!("unreachable")
}

fn main() {
    let parser = Parser::new();
    let map = parser.parse_all(parse);
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
