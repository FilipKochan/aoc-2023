use rust_base::aoc::Parser;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Lines};

type Coord = (i64, i64);

type Planets = (Vec<Coord>, HashSet<i64>, HashSet<i64>);

fn parse_lines(lines: Lines<BufReader<File>>) -> Planets {
    let mut planets = Vec::new();
    let mut occupied_cols = HashSet::new();
    let mut occupied_rows = HashSet::new();

    for (row, l) in lines.enumerate() {
        let row = row as i64;
        for (col, ch) in l.unwrap().char_indices() {
            let col = col as i64;
            if ch == '#' {
                occupied_rows.insert(row);
                occupied_cols.insert(col);
                planets.push((row, col));
            }
        }
    }

    (planets, occupied_rows, occupied_cols)
}

fn d(left: Coord, right: Coord) -> i64 {
    let (row1, col1) = left;
    let (row2, col2) = right;

    (row1 - row2).abs() + (col1 - col2).abs()
}

fn real_planet_coord(
    planet: Coord,
    occupied_rows: &HashSet<i64>,
    occupied_cols: &HashSet<i64>,
    scale: i64,
) -> Coord {
    let (prow, pcol) = planet;

    let mut offs = Vec::new();
    for (coord, set) in [prow, pcol].iter().zip([occupied_rows, occupied_cols]) {
        offs.push((scale - 1) * (0..=*coord).filter(|v| !set.contains(v)).count() as i64)
    }

    (prow + offs[0], pcol + offs[1])
}

fn part(planets: &Planets, scale: i64) -> i64 {
    let (planets, occupied_rows, occupied_cols) = planets;

    let mut coords_cache = HashMap::new();

    let mut res = 0;
    let max = planets.len();
    for i in 0..max {
        for j in i..max {
            let mut coords = Vec::new();

            for coord in [planets[i], planets[j]] {
                if let Some(coord) = coords_cache.get(&coord) {
                    coords.push(*coord);
                } else {
                    let computed = real_planet_coord(coord, occupied_rows, occupied_cols, scale);
                    // verify that nothing is computed twice
                    let prev = coords_cache.insert(coord, computed);
                    assert_eq!(prev, None);
                    coords.push(computed);
                }
            }

            let dist = d(coords[0], coords[1]);
            res += dist;
        }
    }

    res
}

fn main() {
    let parser = Parser::new();
    let parsed = parser.parse_all(parse_lines);
    println!("Part 1: {}", part(&parsed, 2));
    println!("Part 2: {}", part(&parsed, 1_000_000));
}
