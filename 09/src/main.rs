use rust_base::aoc::Parser;

fn reduce(sequence: &[i32]) -> Vec<i32> {
    let mut next = Vec::new();
    for (i, num) in sequence.iter().enumerate().skip(1) {
        next.push(*num - sequence[i - 1]);
    }
    next
}

fn compute_next(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|v| v == &0) {
        return 0;
    }
    sequence.last().unwrap() + compute_next(&reduce(sequence))
}

fn compute_prev(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|v| v == &0) {
        return 0;
    }
    sequence[0] - compute_prev(&reduce(sequence))
}

fn part1(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().map(|s| compute_next(s.as_slice())).sum()
}

fn part2(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().map(|s| compute_prev(s.as_slice())).sum()
}

fn main() {
    let parser = Parser::new();
    let sequences = parser.parse_by_lines(|l| {
        l.split(' ')
            .map(|part| part.trim().parse().unwrap())
            .collect::<Vec<i32>>()
    });
    println!("Part 1: {}", part1(&sequences));
    println!("Part 2: {}", part2(&sequences));
}
