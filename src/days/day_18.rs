use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{delimited, separated_pair};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    plan: Vec<(Instruction, Instruction)>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Instruction {
    direction: Direction,
    length: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            separated_pair(
                Instruction::parse,
                complete::space1,
                Instruction::parse_hex,
            ),
        )
            .map(|plan| Self { plan })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let instructions = self.plan.iter().copied().map(|(instruction, _)| instruction).collect();
        compute_coverage(&instructions).to_string()
    }

    fn part_2(&self) -> String {
        let instructions = self.plan.iter().copied().map(|(_, instruction)| instruction).collect();
        compute_coverage(&instructions).to_string()
    }
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            alt((
                tag("U").value(Direction::Up),
                tag("D").value(Direction::Down),
                tag("L").value(Direction::Left),
                tag("R").value(Direction::Right),
            )),
            complete::space1,
            complete::u32,
        )
            .map(|(direction, length)| Self { direction, length })
            .parse(input)
    }

    fn parse_hex(input: &str) -> IResult<&str, Self> {
        delimited(
            tag("(#"),
            complete::alphanumeric1,
            tag(")"),
        )
            .map(|s: &str| {
                let mut chars = s.chars();
                let direction = match chars.next_back().unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!()
                };
                let length = u32::from_str_radix(chars.as_str(), 16).unwrap();
                Self { direction, length }
            })
            .parse(input)
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Vertex {
    row: isize,
    col: isize,
    from: Direction,
    to: Direction,
}


fn compute_coverage(instructions: &Vec<Instruction>) -> u64 {
    let perimeter: u32 = instructions.iter()
        .map(|instruction| instruction.length)
        .sum();

    let mut last_position = (0, 0);
    let area: i64 = instructions.iter().map(
        |instruction| {
            match instruction.direction {
                Direction::Up => {
                    last_position.0 -= instruction.length as i64;
                    last_position.1 * instruction.length as i64
                }
                Direction::Down => {
                    last_position.0 += instruction.length as i64;
                    -last_position.1 * instruction.length as i64
                }
                Direction::Left => {
                    last_position.1 -= instruction.length as i64;
                    -last_position.0 * instruction.length as i64
                }
                Direction::Right => {
                    last_position.1 += instruction.length as i64;
                    last_position.0 * instruction.length as i64
                }
            }
        }
    )
        .sum();

    (area.abs() as u64 + perimeter as u64) / 2 + 1
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_18.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            plan: vec![]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "62");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "952408144115");
    }
}