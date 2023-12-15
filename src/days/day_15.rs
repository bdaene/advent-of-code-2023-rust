use nom::{IResult, Parser};
use nom::bytes::complete::{tag, take_till1};
use nom::multi::separated_list1;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    steps: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Box {
    lenses: Vec<(String, usize)>,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            tag(","),
            take_till1(|c| ",\r\n".contains(c)),
        )
            .map(|steps| Self {
                steps: steps.into_iter()
                    .map(|step: &str| String::from(step))
                    .collect()
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.steps.iter()
            .map(|step| hash(step.as_bytes()) as usize)
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut boxes = Vec::new();
        for _ in 0..256 {
            boxes.push(Box::new())
        }

        self.steps.iter().for_each(|step| {
            if let Some(label) = step.strip_suffix("-") {
                boxes[hash(label.as_bytes()) as usize].remove_lens(label)
            } else if let Some((label, focal_length)) = step.split_once("=") {
                boxes[hash(label.as_bytes()) as usize].add_lens(label, focal_length.parse().unwrap())
            }
        });

        boxes.iter().enumerate()
            .map(|(i, box_)| (i + 1) * box_.get_total_focusing_power())
            .sum::<usize>()
            .to_string()
    }
}

impl Box {
    fn new() -> Box {
        Box { lenses: Vec::new() }
    }
    fn remove_lens(&mut self, label: &str) {
        if let Some(index) = self.lenses.iter().position(|(label_, _)| label_ == label) {
            self.lenses.remove(index);
        }
    }

    fn add_lens(&mut self, label: &str, focal_length: usize) {
        let lens = (String::from(label), focal_length);
        if let Some(index) = self.lenses.iter().position(|(label_, _)| label_ == label) {
            self.lenses[index] = lens;
        } else {
            self.lenses.push(lens)
        }
    }

    fn get_total_focusing_power(&self) -> usize {
        self.lenses.iter().enumerate()
            .map(|(i, (_label, focal_length))| (i + 1) * focal_length)
            .sum()
    }
}


fn hash(string: &[u8]) -> u8 {
    string.iter().fold(0, |acc, c| acc.wrapping_add(*c).wrapping_mul(17))
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_15.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            steps: vec![
                String::from("rn=1"),
                String::from("cm-"),
                String::from("qp=3"),
                String::from("cm=2"),
                String::from("qp-"),
                String::from("pc=4"),
                String::from("ot=9"),
                String::from("ab=5"),
                String::from("pc-"),
                String::from("pc=6"),
                String::from("ot=7"),
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "1320");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "145");
    }
}