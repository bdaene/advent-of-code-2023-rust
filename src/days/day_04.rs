use nom::{IResult, Parser};
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            Card::parse,
        )
            .map(|cards| Self { cards })
            .parse(input)
    }
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            tag("Card ").precedes(complete::space0.precedes(complete::u32)),
            tag(": "),
            separated_pair(
                separated_list1(
                    complete::space1,
                    complete::space0.precedes(complete::u32),
                ),
                tag(" | "),
                separated_list1(
                    complete::space1,
                    complete::space0.precedes(complete::u32),
                ),
            ),
        )
            .map(|(id, (winning_numbers, numbers))| Self { id, winning_numbers, numbers })
            .parse(input)
    }

    fn get_won_numbers(&self) -> Vec<u32> {
        self.numbers.iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .map(|number| *number)
            .collect()
    }
}


impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle::parse(data).unwrap().1
    }

    fn part_1(&self) -> String {
        self.cards.iter()
            .map(|card| {
                let won_numbers = card.get_won_numbers();
                if won_numbers.len() > 0 {
                    1 << (won_numbers.len() - 1)
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut total_cards = vec![1; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            for k in 1..=card.get_won_numbers().len() {
                total_cards[i + k] += total_cards[i];
            }
        }
        total_cards.iter().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_04.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            cards: vec![
                Card { id: 1, winning_numbers: vec![41, 48, 83, 86, 17], numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] },
                Card { id: 2, winning_numbers: vec![13, 32, 20, 16, 61], numbers: vec![61, 30, 68, 82, 17, 32, 24, 19] },
                Card { id: 3, winning_numbers: vec![1, 21, 53, 59, 44], numbers: vec![69, 82, 63, 72, 16, 21, 14, 1] },
                Card { id: 4, winning_numbers: vec![41, 92, 73, 84, 69], numbers: vec![59, 84, 76, 51, 58, 5, 54, 83] },
                Card { id: 5, winning_numbers: vec![87, 83, 26, 28, 32], numbers: vec![88, 30, 70, 12, 93, 22, 82, 36] },
                Card { id: 6, winning_numbers: vec![31, 18, 13, 56, 72], numbers: vec![74, 77, 10, 23, 35, 67, 36, 11] }]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "13");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "30");
    }
}