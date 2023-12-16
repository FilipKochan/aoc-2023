use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Lines},
};

use rust_base::aoc::Parser;

struct Board {
    board_: Vec<Vec<Block>>,
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Void,
    HorizontalSplit,
    VerticalSplit,
    ClockwiseTurn,
    CounterClockwiseTurn,
}

impl From<char> for Block {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Void,
            '-' => Self::HorizontalSplit,
            '|' => Self::VerticalSplit,
            '\\' => Self::CounterClockwiseTurn,
            '/' => Self::ClockwiseTurn,
            ch => panic!("unknown char {}", ch),
        }
    }
}

impl Board {
    fn new(board_: Lines<BufReader<File>>) -> Self {
        let mut b = Vec::new();
        for l in board_.flatten() {
            b.push(l.chars().map(|ch| ch.into()).collect());
        }

        Board { board_: b }
    }

    fn get(&self, pos: Pos) -> Option<Block> {
        let (row, col) = pos;
        if row < 0
            || col < 0
            || row as usize >= self.board_.len()
            || col as usize >= self.board_[0].len()
        {
            return None;
        }

        Some(self.board_[row as usize][col as usize])
    }
}

type Pos = (i32, i32);

fn traverse_from(mut pos: Pos, mut dir: Pos, board: &Board, visited: &mut HashSet<(Pos, Pos)>) {
    while let Some(block) = board.get(pos) {
        if !visited.insert((pos, dir)) {
            return;
        };

        match block {
            Block::Void => {}
            Block::HorizontalSplit => {
                if dir.0 != 0 {
                    traverse_from((pos.0, pos.1 - 1), (0, -1), board, visited);
                    traverse_from((pos.0, pos.1 + 1), (0, 1), board, visited);
                    return;
                }
            }
            Block::VerticalSplit => {
                if dir.1 != 0 {
                    traverse_from((pos.0 - 1, pos.1), (-1, 0), board, visited);
                    traverse_from((pos.0 + 1, pos.1), (1, 0), board, visited);
                    return;
                }
            }
            Block::ClockwiseTurn => {
                // /
                dir = (-dir.1, -dir.0);
            }
            Block::CounterClockwiseTurn => {
                // \
                dir = (dir.1, dir.0);
            }
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
}

fn part1(board: &Board) -> usize {
    let mut visited = HashSet::new();
    traverse_from((0, 0), (0, 1), board, &mut visited);
    different_positions(visited)
}

fn different_positions(path: HashSet<(Pos, Pos)>) -> usize {
    path.iter()
        .map(|(pos, _dir)| *pos)
        .collect::<HashSet<Pos>>()
        .len()
}

fn part2(board: &Board) -> usize {
    let mut max = 0;
    for row in 0..board.board_.len() {
        let mut visited = HashSet::new();
        traverse_from((row as i32, 0), (0, 1), board, &mut visited);
        let mut visited_ = HashSet::new();
        traverse_from(
            (row as i32, (board.board_[0].len() - 1) as i32),
            (0, -1),
            board,
            &mut visited_,
        );
        max = max
            .max(different_positions(visited_))
            .max(different_positions(visited));
    }

    for col in 0..board.board_[0].len() {
        let mut visited = HashSet::new();
        traverse_from((0, col as i32), (1, 0), board, &mut visited);
        let mut visited_ = HashSet::new();
        traverse_from(
            ((board.board_.len() - 1) as i32, col as i32),
            (-1, 0),
            board,
            &mut visited_,
        );
        max = max
            .max(different_positions(visited_))
            .max(different_positions(visited));
    }
    max
}

fn main() {
    let parser = Parser::new();
    let board = parser.parse_all(Board::new);
    println!("Part 1: {}", part1(&board));
    println!("Part 2: {}", part2(&board));
}
