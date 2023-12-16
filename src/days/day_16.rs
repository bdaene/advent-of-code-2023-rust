use nom::{IResult, Parser};
use nom::bytes::complete::take_till1;
use nom::character::complete;
use nom::multi::separated_list1;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    grid: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            take_till1(|c| "\r\n".contains(c)),
        )
            .map(|lines| Self {
                grid: lines.into_iter()
                    .map(|line: &str| line.chars().collect())
                    .collect()
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        energize(&self.grid, Position { row: 0, col: 0 }, Direction::Right)
            .to_string()
    }

    fn part_2(&self) -> String {
        let (height, width) = (self.grid.len(), self.grid[0].len());

        let max_right = (0..height)
            .map(|row| energize(&self.grid, Position{row, col:0}, Direction::Right))
            .max().unwrap();
        let max_down = (0..width)
            .map(|col| energize(&self.grid, Position{row:0, col}, Direction::Down))
            .max().unwrap();
        let max_left = (0..height)
            .map(|row| energize(&self.grid, Position{row, col:width-1}, Direction::Left))
            .max().unwrap();
        let max_up = (0..width)
            .map(|col| energize(&self.grid, Position{row:height-1, col}, Direction::Up))
            .max().unwrap();

        max_right.max(max_down).max(max_left).max(max_up).to_string()
    }
}

impl Direction {
    fn bounce(&self, cell: char) -> Vec<Direction> {
        match cell {
            '.' => vec![*self],
            '/' => match self {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            '\\' => match self {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            '-' => match self {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Right, Direction::Left],
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
            },
            '|' => match self {
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Down, Direction::Up],
            },
            _ => panic!("Unknown cell {cell}")
        }
    }
}

impl Position {
    fn update(&self, direction: &Direction, height: usize, width: usize) -> Option<Position> {
        let (row, col) = (self.row, self.col);
        match direction {
            Direction::Up => if self.row > 0 { Some(Position { row: row - 1, col }) } else { None },
            Direction::Left => if self.col > 0 { Some(Position { row, col: col - 1 }) } else { None },
            Direction::Right => if self.col + 1 < width { Some(Position { row, col: col + 1 }) } else { None },
            Direction::Down => if self.row + 1 < height { Some(Position { row: row + 1, col }) } else { None },
        }
    }
}

fn energize(grid: &Vec<Vec<char>>, position: Position, direction: Direction) -> usize {
    let (height, width) = (grid.len(), grid[0].len());
    let mut states = vec![(position, direction)];

    let mut seen = vec![vec![vec![false; width]; height]; 4];

    while let Some((position, direction)) = states.pop() {
        if seen[direction as usize][position.row][position.col] {
            continue;
        }
        seen[direction as usize][position.row][position.col] = true;
        for direction_ in direction.bounce(grid[position.row][position.col]) {
            if let Some(position_) = position.update(&direction_, height, width) {
                states.push((position_, direction_))
            }
        }
    }

    (0..height)
        .flat_map(|row| (0..width)
            .map(move |col| (row, col))
            .filter(|&(row, col)| seen[Direction::Up as usize][row][col]
                | seen[Direction::Down as usize][row][col]
                | seen[Direction::Left as usize][row][col]
                | seen[Direction::Right as usize][row][col]
            ))
        .count()
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_16.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            grid: vec![
                vec!['.', '|', '.', '.', '.', '\\', '.', '.', '.', '.'],
                vec!['|', '.', '-', '.', '\\', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '|', '-', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '|', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '\\'],
                vec!['.', '.', '.', '.', '/', '.', '\\', '\\', '.', '.'],
                vec!['.', '-', '.', '-', '/', '.', '.', '|', '.', '.'],
                vec!['.', '|', '.', '.', '.', '.', '-', '|', '.', '\\'],
                vec!['.', '.', '/', '/', '.', '|', '.', '.', '.', '.'],
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "46");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "51");
    }
}