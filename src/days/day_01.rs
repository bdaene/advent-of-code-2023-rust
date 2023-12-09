use nom::{IResult, Parser};
use nom::bytes::complete::take_till1;
use nom::character::complete;
use nom::multi::separated_list1;

use crate::PuzzleBase;

#[derive(PartialEq, Debug)]
pub struct Puzzle {
    lines: Vec<String>,
}

const DIGITS_NAME: [&str; 18] = [
    "1", "one",
    "2", "two",
    "3", "three",
    "4", "four",
    "5", "five",
    "6", "six",
    "7", "seven",
    "8", "eight",
    "9", "nine",
];

fn get_first_digit(line: &str) -> Option<u32> {
    let digit_index = DIGITS_NAME.iter()
        .enumerate()
        .filter_map(|(i, &digit)| {
            if let Some(position) = line.find(digit) {
                Some((position, i))
            } else {
                None
            }
        })
        .min()?
        .1;
    Some(1 + ((digit_index as u32) >> 1))
}

fn get_last_digit(line: &str) -> Option<u32> {
    let digit_index = DIGITS_NAME.iter()
        .enumerate()
        .filter_map(|(i, &digit)| {
            if let Some(position) = line.rfind(digit) {
                Some((position, i))
            } else {
                None
            }
        })
        .max()?
        .1;
    Some(1 + ((digit_index as u32) >> 1))
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            take_till1(|c| "\r\n".contains(c)),
        )
            .map(|lines| Self {
                lines: lines.into_iter()
                    .map(|line: &str| String::from(line))
                    .collect()
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.lines.iter()
            .map(|line| {
                let digits: Vec<u32> = line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect();
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum::<u32>().to_string()
    }

    fn part_2(&self) -> String {
        self.lines.iter()
            .map(|line| {
                get_first_digit(line).unwrap() * 10 + get_last_digit(line).unwrap()
            })
            .sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_example() -> Puzzle {
        let data: String = fs::read_to_string("data/examples/day_01.txt").unwrap();

        Puzzle::new(&data)
    }

    fn get_example_2() -> Puzzle {
        let data: String = fs::read_to_string("data/examples/day_01_2.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_example();

        assert_eq!(
            solution,
            Puzzle {
                lines: vec![
                    String::from("1abc2"),
                    String::from("pqr3stu8vwx"),
                    String::from("a1b2c3d4e5f"),
                    String::from("treb7uchet"),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_example();

        assert_eq!(solution.part_1(), "142");
    }

    #[test]
    fn part_2() {
        let solution = get_example_2();

        assert_eq!(solution.part_2(), "358");
    }

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("three"), Some(3));
        assert_eq!(get_first_digit("4three5"), Some(4));
        assert_eq!(get_first_digit("25twoabds"), Some(2));
        assert_eq!(get_first_digit("___oooneee___"), Some(1));
        assert_eq!(get_first_digit("8"), Some(8));
        assert_eq!(get_first_digit("fquhqz"), None);
        assert_eq!(get_first_digit("454"), Some(4));
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("three"), Some(3));
        assert_eq!(get_last_digit("4three5"), Some(5));
        assert_eq!(get_last_digit("25twoabds"), Some(2));
        assert_eq!(get_last_digit("___oooneee___"), Some(1));
        assert_eq!(get_last_digit("8"), Some(8));
        assert_eq!(get_last_digit("fquhqz"), None);
        assert_eq!(get_last_digit("454"), Some(4));
    }
}