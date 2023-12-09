use nom::character::complete;
use nom::{IResult, Parser};
use nom::multi::separated_list1;
use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    sequences: Vec<Vec<i32>>,
}

impl Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            separated_list1(
                complete::space1,
                complete::i32,
            ),
        )
            .map(|sequences| Self { sequences })
            .parse(input)
    }
}

fn get_next(sequence: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut sequence = sequence.to_vec();
    while let Some(ending) = sequence.last() {
        result += ending;
        sequence = sequence.iter()
            .zip(sequence.iter().skip(1))
            .map(|(a,b)| b-a)
            .collect();
    }
    result
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle::parse(data).unwrap().1
    }

    fn part_1(&self) -> String {
        self.sequences.iter()
            .map(|sequence| get_next(sequence))
            .sum::<i32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.sequences.iter()
            .map(|sequence| {
                let mut sequence = sequence.to_vec();
                sequence.reverse();
                get_next(&sequence)
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_09.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            sequences: vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ]
        });
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "114");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "2");
    }
}