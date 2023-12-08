use num::integer::lcm;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Lines},
};

use rust_base::aoc::Parser;

#[derive(Debug, Clone)]
struct Component {
    left: String,
    right: String,
}

fn parse(lines: Lines<BufReader<File>>) -> (String, HashMap<String, Component>) {
    let mut lines = lines.flatten();
    let directions = String::from(lines.next().unwrap().trim());

    lines.next();

    let mut components = HashMap::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    for l in lines {
        let res = re.captures(l.as_str()).unwrap();
        components.insert(
            String::from(&res[1]),
            Component {
                left: String::from(&res[2]),
                right: String::from(&res[3]),
            },
        );
    }

    (directions, components)
}

fn part1(
    directions: String,
    map: HashMap<String, Component>,
    start: String,
    mut end_predicate: Box<dyn FnMut(String, i32) -> bool>,
) -> i32 {
    let mut cur = start;
    let mut steps = 0;
    for direction in directions.chars().cycle() {
        if end_predicate(cur.clone(), steps) {
            break;
        }

        steps += 1;
        if direction == 'R' {
            cur = map[&cur].right.clone();
        } else if direction == 'L' {
            cur = map[&cur].left.clone();
        } else {
            panic!("unknown direction {}", direction);
        }
    }

    steps
}

fn ends_with(s: String, c: char) -> bool {
    s.chars().last().unwrap() == c
}

fn part2(directions: String, map: HashMap<String, Component>) -> i64 {
    let mut starts = Vec::new();
    for name in map.keys() {
        if ends_with(name.clone(), 'A') {
            starts.push(name);
        }
    }

    let mut final_ = 1i64;
    for start in starts {
        let len = directions.len();
        let res = part1(
            directions.clone(),
            map.clone(),
            start.clone(),
            Box::new(move |v, steps| steps % len as i32 == 0 && ends_with(v, 'Z')),
        ) as i64;
        final_ = lcm(res, final_);
    }

    final_
}

fn main() {
    let parser = Parser::new();
    let (directions, map) = parser.parse_all(parse);
    println!(
        "Part 1: {}",
        part1(
            directions.clone(),
            map.clone(),
            String::from("AAA"),
            Box::new(|v, _| v == *"ZZZ")
        )
    );
    println!("Part 2: {}", part2(directions, map));
}
