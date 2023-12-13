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
            .map(|pattern| get_symmetry_value(&pattern.ground, 0))
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.patterns.iter()
            .map(|pattern| get_symmetry_value(&pattern.ground, 1))
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
}


fn get_symmetry_value(ground: &Vec<Vec<GroundType>>, smudge: usize) -> usize {
    if let Some(vertical_axis) = (0..ground[0].len() - 1)
        .filter(|&axis| count_horizontal_differences(&ground, axis) == smudge)
        .next() {
        return vertical_axis + 1;
    }

    let ground = transpose(ground);

    if let Some(horizontal_axis) = (0..ground[0].len() - 1)
        .filter(|&axis| count_horizontal_differences(&ground, axis) == smudge)
        .next() {
        return 100 * (horizontal_axis + 1);
    }

    panic!("No symmetries found for {ground:?}");
}


fn transpose<T>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>>
    where T: Copy
{
    (0..grid[0].len())
        .map(|col| grid.iter().map(|line| line[col]).collect())
        .collect()
}

fn count_horizontal_differences<T>(grid: &Vec<Vec<T>>, axis: usize) -> usize
    where T: PartialEq
{
    grid.iter()
        .map(|row| count_differences(row, axis))
        .sum::<usize>()
}

fn count_differences<T>(vector: &[T], axis: usize) -> usize
    where T: PartialEq
{
    (0..(axis + 1).min(vector.len() - axis - 1))
        .filter(|offset| vector[axis - offset] != vector[axis + offset + 1])
        .count()
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