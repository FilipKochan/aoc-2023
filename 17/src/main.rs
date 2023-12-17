use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use rust_base::aoc::Parser;

fn traverse(map: &Map, shortest_straight: i32, longetst_straight: i32) -> i32 {
    let mut costs = HashMap::new();
    let mut q = BinaryHeap::new();
    let mut prev: HashMap<PointWithStrait, PointWithStrait> = HashMap::new();

    let start = PointWithStrait {
        point: (0, 0),
        remainig_straight: longetst_straight,
        dir: (0, 0),
    };
    q.push(Reverse((0, start.clone())));
    costs.insert(start, 0);

    while let Some(point_) = q.pop() {
        let (_, cur) = point_.0;
        let mut next: Vec<(i32, i32)> = Vec::new();

        if cur.point.0 > 0 {
            next.push((-1, 0));
        }
        if cur.point.1 > 0 {
            next.push((0, -1));
        }
        if cur.point.0 + 1 < map.len() as i32 {
            next.push((1, 0));
        }
        if cur.point.1 + 1 < map[0].len() as i32 {
            next.push((0, 1))
        }

        let dir_straight = prev
            .get(&cur)
            .map(|p| (cur.point.0 - p.point.0, cur.point.1 - p.point.1));

        if cur.remainig_straight == 0 {
            next.retain(|dir| Some(*dir) != dir_straight);
        }

        if cur.remainig_straight > longetst_straight - shortest_straight && dir_straight.is_some() {
            next.retain(|dir| Some(*dir) == dir_straight);
        }

        next.retain(|dir| Some(*dir) != dir_straight.map(|v| (-v.0, -v.1)));

        for (row_off, col_off) in next {
            let rem: i32 = if Some((-col_off, row_off)) == dir_straight
                || Some((col_off, -row_off)) == dir_straight
            {
                longetst_straight - 1
            } else {
                cur.remainig_straight - 1
            };

            let next = PointWithStrait {
                point: (cur.point.0 + row_off, cur.point.1 + col_off),
                remainig_straight: rem,
                dir: (row_off, col_off),
            };

            if Some(next.point) != prev.get(&cur).map(|p| p.point) {
                let d = costs[&cur] + map[next.point.0 as usize][next.point.1 as usize];

                if costs.get(&next).map(|&cost| d < cost).unwrap_or(true) {
                    costs.insert(next.clone(), d);
                    prev.insert(next.clone(), cur.clone());
                    q.push(Reverse((d, next)));
                }
            }
        }
    }

    let end = ((map.len() - 1) as i32, (map[0].len() - 1) as i32);

    let mut best = None;
    for (p, cost) in costs {
        if p.point == end && p.remainig_straight <= longetst_straight - shortest_straight {
            if best.is_none() {
                best = Some(cost);
            } else if let Some(best_) = best {
                if cost < best_ {
                    best = Some(cost);
                }
            }
        }
    }

    best.unwrap()
}

fn part1(map: &Map) -> i32 {
    traverse(map, 0, 3)
}

fn part2(map: &Map) -> i32 {
    traverse(map, 4, 10)
}

type Map = Vec<Vec<i32>>;
type Point = (i32, i32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct PointWithStrait {
    point: Point,
    remainig_straight: i32,
    dir: Point,
}

fn main() {
    let parser = Parser::new();
    let map: Map = parser.parse_by_lines(|l| {
        l.chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>()
    });
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
