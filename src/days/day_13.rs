use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom::sequence::pair;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    patterns: Vec<Pattern>,
}

#[derive(Debug, PartialEq, Eq)]
struct Pattern {
    ground: Vec<Vec<GroundType>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum GroundType {
    Ash,
    Rocks,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            pair(complete::line_ending, complete::line_ending),
            Pattern::parse,
        )
            .map(|patterns| Self { patterns })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.patterns.iter()
            .map(|pattern| pattern.get_symmetry_value().unwrap())
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.patterns.iter()
            .map(|pattern| pattern.get_smudged_symmetry_value().unwrap())
            .sum::<usize>()
            .to_string()
    }
}

impl Pattern {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            many1(alt((
                tag(".").value(GroundType::Ash),
                tag("#").value(GroundType::Rocks),
            ))),
        )
            .map(|ground| Self { ground })
            .parse(input)
    }

    fn get_symmetry_value(&self) -> Option<usize> {
        if let Some(symmetry_value) = get_all_symmetry_values(&self.ground).first() {
            return Some(*symmetry_value);
        }
        None
    }

    fn get_smudged_symmetry_value(&self) -> Option<usize> {
        let mut ground = self.ground.to_vec();
        let known_symmetries = get_all_symmetry_values(&ground);

        for row in 0..ground.len() {
            for col in 0..ground[row].len() {
                ground[row][col] = ground[row][col].opposite();
                let symmetries = get_all_symmetry_values(&ground);
                if let Some(symmetry_value) = symmetries.iter()
                    .filter(|symmetry| !known_symmetries.contains(symmetry))
                    .next() {
                    return Some(*symmetry_value);
                }
                ground[row][col] = ground[row][col].opposite();
            }
        }
        None
    }
}

fn get_all_symmetry_values(ground: &Vec<Vec<GroundType>>) -> Vec<usize> {
    let mut symmetries: Vec<usize> = Vec::new();

    symmetries.extend((0..ground.len() - 1)
        .filter(|&symmetry_row| is_vertically_symmetric(ground, symmetry_row))
        .map(|row| (row + 1) * 100));

    symmetries.extend((0..ground[0].len() - 1)
        .filter(|&symmetry_col| is_horizontally_symmetric(ground, symmetry_col))
        .map(|col| col + 1));

    symmetries
}

fn is_vertically_symmetric(ground: &Vec<Vec<GroundType>>, symmetry_row: usize) -> bool {
    (0..(symmetry_row + 1).min(ground.len() - symmetry_row - 1)).all(
        |offset| ground[symmetry_row - offset].iter()
            .zip(ground[symmetry_row + offset + 1].iter())
            .all(|(&ground_above, &ground_below)| ground_above == ground_below)
    )
}

fn is_horizontally_symmetric(ground: &Vec<Vec<GroundType>>, symmetry_col: usize) -> bool {
    (0..(symmetry_col + 1).min(ground[0].len() - symmetry_col - 1)).all(
        |offset| (0..ground.len())
            .all(|row| ground[row][symmetry_col - offset] == ground[row][symmetry_col + offset + 1])
    )
}

impl GroundType {
    fn opposite(&self) -> GroundType {
        match self {
            GroundType::Ash => GroundType::Rocks,
            GroundType::Rocks => GroundType::Ash,
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_13.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            patterns: vec![
                Pattern {
                    ground: vec![
                        vec![GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Rocks],
                        vec![GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Rocks],
                        vec![GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Ash],
                    ]
                },
                Pattern {
                    ground: vec![
                        vec![GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks],
                        vec![GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks],
                        vec![GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks],
                        vec![GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash],
                        vec![GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Rocks, GroundType::Rocks],
                        vec![GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Ash, GroundType::Rocks, GroundType::Ash, GroundType::Ash, GroundType::Rocks],
                    ]
                },
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "405");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "400");
    }
}