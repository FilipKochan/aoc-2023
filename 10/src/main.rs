use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufReader, Lines},
};

use rust_base::aoc::Parser;

type Coord = (i32, i32);

fn parse(lines: Lines<BufReader<File>>) -> (Coord, HashMap<Coord, char>) {
    let mut pipes = HashMap::new();
    let mut start = None;

    for (row, row_line) in lines.flatten().enumerate() {
        for (col, ch) in row_line.trim().char_indices() {
            let coord = (row as i32, col as i32);
            if ch == 'S' {
                start = Some(coord);
            }

            if ch != '.' {
                pipes.insert(coord, ch);
            }
        }
    }

    (start.unwrap(), pipes)
}

fn around(center: Coord) -> [Coord; 4] {
    let (row, col) = center;
    [
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ]
}

fn neighbors(from: Coord, pipes: &HashMap<Coord, char>) -> Vec<Coord> {
    let [up, down, left, right] = around(from);
    let pipe = pipes[&from];
    match pipe {
        '|' => vec![up, down],
        '-' => vec![left, right],
        'L' => vec![up, right],
        'J' => vec![up, left],
        '7' => vec![left, down],
        'F' => vec![right, down],
        'S' => {
            let mut ns = Vec::new();
            for n in [up, down, left, right] {
                if pipes.contains_key(&n) && neighbors(n, pipes).contains(&from) {
                    ns.push(n)
                }
            }
            ns
        }
        c => panic!("unknown pipe type {}", c),
    }
}

fn traverse(start: Coord, pipes: &HashMap<Coord, char>) -> (i32, HashSet<Coord>) {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut gen = 0;

    while let Some((current, generation)) = q.pop_front() {
        gen = generation.max(gen);
        for neighbor in neighbors(current, pipes) {
            if visited.insert(neighbor) {
                q.push_back((neighbor, generation + 1));
            }
        }
    }

    (gen, visited)
}

fn part1(start: Coord, pipes: &HashMap<Coord, char>) -> i32 {
    traverse(start, pipes).0
}

fn extend(route: &HashSet<Coord>, pipes: &HashMap<Coord, char>) -> HashSet<Coord> {
    let mut extended_route = HashSet::new();

    for coord in route {
        let (row, col) = *coord;
        let center = (row * 2, col * 2);
        let [up, down, left, right] = around(center);
        extended_route.insert(center);
        if pipes[coord] == '|' {
            extended_route.insert(up);
            extended_route.insert(down);
        }
        if pipes[coord] == '-' {
            extended_route.insert(left);
            extended_route.insert(right);
        }
        if pipes[coord] == 'J' {
            extended_route.insert(left);
            extended_route.insert(up);
        }
        if pipes[coord] == 'F' {
            extended_route.insert(down);
            extended_route.insert(right);
        }
        if pipes[coord] == '7' {
            extended_route.insert(down);
            extended_route.insert(left);
        }
        if pipes[coord] == 'L' {
            extended_route.insert(up);
            extended_route.insert(right);
        }
    }

    extended_route
}

fn part2(start: Coord, pipes: &HashMap<Coord, char>) -> usize {
    let boundary = traverse(start, pipes).1;
    let extended_route = extend(&boundary, pipes);

    let min_row = boundary.iter().map(|&(row, _)| row).min().unwrap_or(0) * 2;
    let max_row = boundary.iter().map(|&(row, _)| row).max().unwrap_or(0) * 2;
    let min_col = boundary.iter().map(|&(_, col)| col).min().unwrap_or(0) * 2;
    let max_col = boundary.iter().map(|&(_, col)| col).max().unwrap_or(0) * 2;

    for (row_offset, col_offset) in [(-1, -1), (1, 1), (1, -1), (-1, 1)] {
        let new_start = (start.0 * 2 + row_offset, start.1 * 2 + col_offset);
        let mut visited = HashSet::new();
        if flood_fill(
            new_start,
            &mut visited,
            ((min_row, min_col), (max_row, max_col)),
            &extended_route,
        ) {
            return visited
                .iter()
                .filter(|(r, c)| *r % 2 == 0 && *c % 2 == 0)
                .count();
        }
    }

    panic!("no solution found")
}

fn flood_fill(
    start: Coord,
    visited: &mut HashSet<Coord>,
    bounds: (Coord, Coord),
    route: &HashSet<Coord>,
) -> bool {
    if route.contains(&start) {
        return true;
    }

    let ((min_row, min_col), (max_row, max_col)) = bounds;

    let (r, c) = start;
    if r < min_row || r > max_row || c < min_col || c > max_col {
        return false;
    }

    if !visited.insert(start) {
        return true;
    };

    for neighbor in around(start) {
        if !flood_fill(neighbor, visited, bounds, route) {
            return false;
        }
    }

    true
}

fn main() {
    let parser = Parser::new();
    let (start, pipes) = parser.parse_all(parse);
    println!("Part 1: {}", part1(start, &pipes));
    println!("Part 2: {}", part2(start, &pipes));
}
