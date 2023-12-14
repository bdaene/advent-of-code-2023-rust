use std::collections::HashMap;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    lines: Vec<Vec<Rock>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position {
    row: usize,
    col: usize,
    rock: Rock,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            line_ending,
            many1(alt((
                tag("O").value(Rock::Round),
                tag("#").value(Rock::Cube),
                tag(".").value(Rock::Empty),
            ))),
        )
            .map(|lines| Self { lines })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let mut offsets = vec![0usize; self.lines[0].len()];

        let mut total = 0;
        for (row, line) in self.lines.iter().enumerate() {
            for (col, &rock) in line.iter().enumerate() {
                if rock == Rock::Round {
                    total += self.lines.len() - offsets[col];
                    offsets[col] += 1;
                } else if rock == Rock::Cube {
                    offsets[col] = row + 1;
                }
            }
        }

        total.to_string()
    }

    fn part_2(&self) -> String {
        let mut positions: Vec<Position> = self.lines.iter().enumerate()
            .flat_map(|(row, line)| line.iter().enumerate()
                .map(move |(col, &rock)| Position { row, col, rock })
            )
            .filter(|position| position.rock != Rock::Empty)
            .collect();

        let (height, width) = (self.lines.len(), self.lines[0].len());
        let mut known_positions: HashMap<Vec<Position>, usize> = HashMap::new();
        for cycle in 0..1_000_000_000 {
            if let Some(previous_cycle) = known_positions.insert(positions.clone(), cycle) {
                let cycle_length = cycle - previous_cycle;
                for _ in 0..((1_000_000_000 - cycle) % cycle_length) {
                    positions = cycle_directions(positions.to_vec(), height, width)
                }
                break;
            }
            let mut new_positions = cycle_directions(positions.to_vec(), height, width);
            new_positions.sort_unstable_by_key(|position| (position.row, position.col));
            positions = new_positions
        }

        get_north_load(&positions, height).to_string()
    }
}

fn tilt_north(mut positions: Vec<Position>, height: usize, width: usize) -> Vec<Position> {
    positions.sort_unstable_by_key(|position| position.row);
    debug_assert!(positions.iter().all(|position| (0..height).contains(&position.row) && (0..width).contains(&position.col)));

    let mut offsets = vec![0; width];

    positions.into_iter().map(|position| match position.rock {
        Rock::Cube => {
            offsets[position.col] = position.row + 1;
            position
        }
        Rock::Round => {
            offsets[position.col] += 1;
            Position { row: offsets[position.col] - 1, ..position }
        }
        Rock::Empty => position
    })
        .collect()
}

fn tilt_south(mut positions: Vec<Position>, height: usize, width: usize) -> Vec<Position> {
    positions.sort_unstable_by_key(|position| position.row);
    positions.reverse();
    debug_assert!(positions.iter().all(|position| (0..height).contains(&position.row) && (0..width).contains(&position.col)));

    let mut offsets = vec![height; width];

    positions.into_iter().map(|position| match position.rock {
        Rock::Cube => {
            offsets[position.col] = position.row;
            position
        }
        Rock::Round => {
            offsets[position.col] -= 1;
            Position { row: offsets[position.col], ..position }
        }
        Rock::Empty => position
    })
        .collect()
}

fn tilt_west(mut positions: Vec<Position>, height: usize, width: usize) -> Vec<Position> {
    positions.sort_unstable_by_key(|position| position.col);
    debug_assert!(positions.iter().all(|position| (0..height).contains(&position.row) && (0..width).contains(&position.col)));

    let mut offsets = vec![0; height];

    positions.into_iter().map(|position| match position.rock {
        Rock::Cube => {
            offsets[position.row] = position.col + 1;
            position
        }
        Rock::Round => {
            offsets[position.row] += 1;
            Position { col: offsets[position.row] - 1, ..position }
        }
        Rock::Empty => position
    })
        .collect()
}

fn tilt_east(mut positions: Vec<Position>, height: usize, width: usize) -> Vec<Position> {
    positions.sort_unstable_by_key(|position| position.col);
    positions.reverse();
    debug_assert!(positions.iter().all(|position| (0..height).contains(&position.row) && (0..width).contains(&position.col)));

    let mut offsets = vec![width; height];

    positions.into_iter().map(|position| match position.rock {
        Rock::Cube => {
            offsets[position.row] = position.col;
            position
        }
        Rock::Round => {
            offsets[position.row] -= 1;
            Position { col: offsets[position.row], ..position }
        }
        Rock::Empty => position
    })
        .collect()
}

fn cycle_directions(mut positions: Vec<Position>, height: usize, width: usize) -> Vec<Position> {
    positions = tilt_north(positions, height, width);
    positions = tilt_west(positions, height, width);
    positions = tilt_south(positions, height, width);
    positions = tilt_east(positions, height, width);
    positions
}

fn get_north_load(positions: &Vec<Position>, height: usize) -> usize {
    positions.iter()
        .map(|position| match position.rock {
            Rock::Round => height - position.row,
            _ => 0,
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_14.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            lines: vec![
                vec![Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty],
                vec![Rock::Round, Rock::Empty, Rock::Round, Rock::Round, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube],
                vec![Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty],
                vec![Rock::Round, Rock::Round, Rock::Empty, Rock::Cube, Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round],
                vec![Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round, Rock::Cube, Rock::Empty],
                vec![Rock::Round, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Cube],
                vec![Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Round, Rock::Empty, Rock::Empty, Rock::Round],
                vec![Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty],
                vec![Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Cube, Rock::Cube, Rock::Empty, Rock::Empty],
                vec![Rock::Cube, Rock::Round, Rock::Round, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty]]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "136");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "64");
    }
}