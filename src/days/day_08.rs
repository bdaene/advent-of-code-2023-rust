use std::collections::HashMap;
use std::mem::swap;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, tuple};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    instructions: Vec<Instruction>,
    network: HashMap<String, (String, String)>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    LEFT,
    RIGHT,
}

impl Puzzle {
    fn get_cycle_length(&self, start: &str) -> usize {
        let mut node = start;
        let mut instructions = self.instructions.iter().cycle();
        let mut counter = 0;

        while !node.ends_with("Z") {
            node = match instructions.next().unwrap() {
                Instruction::LEFT => &self.network.get(node).unwrap().0,
                Instruction::RIGHT => &self.network.get(node).unwrap().1,
            };
            counter += 1;
        }
        counter
    }
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("L").value(Instruction::LEFT),
            tag("R").value(Instruction::RIGHT),
        ))
            .parse(input)
    }
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            many1(Instruction::parse),
            tuple((complete::line_ending, complete::line_ending)),
            separated_list1(
                complete::line_ending,
                separated_pair(
                    complete::alphanumeric1,
                    tag(" = "),
                    tuple((
                        tag("("), complete::alphanumeric1, tag(", "), complete::alphanumeric1, tag(")"))),
                ),
            ),
        )
            .map(|(instructions, nodes)| {
                let mut network = HashMap::new();
                nodes.iter().for_each(|(node, (_, left, _, right, _))| {
                    network.insert(node.to_string(), (left.to_string(), right.to_string()));
                });
                Self {
                    instructions,
                    network,
                }
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.get_cycle_length("AAA").to_string()
    }

    fn part_2(&self) -> String {
        let cycles_length: Vec<usize> = self.network.keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| self.get_cycle_length(node))
            .collect();

        cycles_length.iter().fold(
            1,
            |ppcm, &n| ppcm * n / gcd(n, ppcm),
        ).to_string()
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a != 0 {
        b = b % a;
        swap(&mut a, &mut b);
    }
    b
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle(i: usize) -> Puzzle {
        let data = fs::read_to_string(format!("data/examples/day_08_{i}.txt")).unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle(1);
        let mut network = HashMap::new();
        network.insert("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()));
        network.insert("BBB".to_string(), ("DDD".to_string(), "EEE".to_string()));
        network.insert("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string()));
        network.insert("DDD".to_string(), ("DDD".to_string(), "DDD".to_string()));
        network.insert("EEE".to_string(), ("EEE".to_string(), "EEE".to_string()));
        network.insert("GGG".to_string(), ("GGG".to_string(), "GGG".to_string()));
        network.insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));


        assert_eq!(puzzle, Puzzle {
            instructions: vec![Instruction::RIGHT, Instruction::LEFT],
            network,
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle(2);

        assert_eq!(puzzle.part_1(), "6");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle(3);

        assert_eq!(puzzle.part_2(), "6");
    }
}