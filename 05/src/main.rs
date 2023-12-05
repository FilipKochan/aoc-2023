use std::{
    collections::HashSet,
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
struct Boundary {
    src: i64,
    dst: i64,
    range: i64,
}

impl From<Vec<i64>> for Boundary {
    fn from(value: Vec<i64>) -> Self {
        Boundary { src: value[1], dst: value[0], range: value[2] }
    }
}

struct Data {
    seeds: Vec<i64>,
    maps: Vec<HashSet<Boundary>>
}

fn parse_boundary(line: String) -> Boundary {
    let nums: Vec<i64> = line.trim().split(' ').map(|v|v.parse::<i64>().unwrap()).collect();
    nums.into()
}

fn parse_lines(mut lines: impl Iterator<Item = String>) -> Data {
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect();


    let mut maps = Vec::new();

    let mut last = HashSet::new();

    lines.next().unwrap();

    for line in lines {
        if line.is_empty() {
            maps.push(last.clone());
            last.clear();
        } else if line.chars().next().unwrap().is_digit(10) {
            let b = parse_boundary(line);
            last.insert(b);
        }
    }

    if !last.is_empty() {
        maps.push(last);
    }

    Data { seeds, maps }
}


fn find_location(seed: i64, maps: &Vec<HashSet<Boundary>>) -> (i64, Option<i64>) {
    let mut id = seed;

    let mut same_count = None;

    for map in maps {
        for boundary in map {
            let offset = id - boundary.src;

            if offset < 0 {
                let count_to_start = -offset;
                if let Some(b) = same_count {
                    if count_to_start < b {
                        same_count = Some(count_to_start);
                    }
                }
            }

            if offset >= 0 && offset < boundary.range {
                id = boundary.dst + offset;

                let count_till_end = boundary.range - offset;

                if let Some(b) = same_count {
                    if count_till_end < b {
                        same_count = Some(count_till_end)
                    }
                } else {
                    same_count = Some(count_till_end);
                }
                break;
            }
        }
    }

    (id, same_count)
}

fn part1(data: &Data) -> i64 {
    data.seeds.iter().map(|seed| find_location(*seed, &data.maps)).min().unwrap().0
}

fn part2(data: &Data) -> i64 {
    data.seeds.chunks(2).map(|chunk| {
        let start = chunk[0];
        let len = chunk[1];
        let mut lowest = None;
        let mut i = start;
        while i < start + len {
            let (location, can_skip) = find_location(i, &data.maps);

            if let Some(lowest_) = lowest {
                if location < lowest_ {
                    lowest = Some(location);
                }
            } else {
                lowest = Some(location);
            }

            i += can_skip.unwrap_or(1);
        }
        lowest.unwrap()
    })
    .min()
    .unwrap()
}

fn main() -> io::Result<()> {
    let name = args().nth(1);

    if let Some(name) = name {
        let file = BufReader::new(File::open(name)?);
        let parsed_lines = parse_lines(file.lines().flatten());

        
        println!("Part 1: {}", part1(&parsed_lines));
        println!("Part 2: {}", part2(&parsed_lines));
    } else {
        return Err(Error::new(
            io::ErrorKind::InvalidInput,
            "input file not provided",
        ));
    }

    Ok(())
}
