use rust_base::aoc::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RecordType {
    Operational,
    Damaged,
    Unknown,
}

type Row = (Vec<RecordType>, Vec<i32>);

impl Into<RecordType> for char {
    fn into(self) -> RecordType {
        match self {
            '.' => RecordType::Operational,
            '#' => RecordType::Damaged,
            '?' => RecordType::Unknown,
            ch => panic!("unknown type {}", ch),
        }
    }
}

fn count_arrangements(row: &Row) -> i32 {
    let (springs, lengths) = row;

    let mut i = 0;
    let mut spring_size = 0;

    let mut res = 0;
    for (s_i, spring) in springs.iter().enumerate() {
        if *spring == RecordType::Operational && spring_size > 0 {
            if i >= lengths.len() || lengths[i] != spring_size {
                return 0;
            }
            i += 1;
            spring_size = 0;
        } else if *spring == RecordType::Damaged {
            spring_size += 1;
        } else if *spring == RecordType::Unknown {
            let mut new_springs = springs.clone();
            new_springs[s_i] = RecordType::Damaged;
            res += count_arrangements(&(new_springs.clone(), lengths.clone()));
            new_springs[s_i] = RecordType::Operational;
            res += count_arrangements(&(new_springs, lengths.clone()));
            return res;
        }
    }

    if i == lengths.len() {
        return 1;
    }

    0
}

fn part1(data: &Vec<Row>) -> i32 {
    data.iter().map(count_arrangements).sum()
}

fn main() {
    let parser = Parser::new();
    let parsed: Vec<Row> = parser.parse_by_lines(|l| {
        let splitted: Vec<&str> = l.split(' ').collect();
        let mut springs: Vec<RecordType> = splitted[0].chars().map(|ch| ch.into()).collect();
        springs.push(RecordType::Operational);
        let lengths: Vec<i32> = splitted[1]
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();
        (springs, lengths)
    });

    println!("Part 1: {}", part1(&parsed));
}
