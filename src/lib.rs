use nom::character::complete;
use nom::{IResult, Parser};
use nom::sequence::terminated;

pub mod days;

pub trait PuzzleBase {
    fn new(data: &str) -> Self
        where
            Self: Sized {
        terminated(Self::parse, complete::line_ending).parse(data).unwrap().1
    }

    fn parse(input: &str) -> IResult<&str, Self>
        where
            Self: Sized;

    fn part_1(&self) -> String {
        String::from("Not implemented yet.")
    }

    fn part_2(&self) -> String {
        String::from("Not implemented yet.")
    }
}

pub fn get_puzzle(day: u8, data: &str) -> Box<dyn PuzzleBase> {
    match day {
        01 => Box::new(days::day_01::Puzzle::new(data)),
        02 => Box::new(days::day_02::Puzzle::new(data)),
        03 => Box::new(days::day_03::Puzzle::new(data)),
        04 => Box::new(days::day_04::Puzzle::new(data)),
        05 => Box::new(days::day_05::Puzzle::new(data)),
        06 => Box::new(days::day_06::Puzzle::new(data)),
        07 => Box::new(days::day_07::Puzzle::new(data)),
        08 => Box::new(days::day_08::Puzzle::new(data)),
        09 => Box::new(days::day_09::Puzzle::new(data)),
        10 => Box::new(days::day_10::Puzzle::new(data)),

        _ => panic!("Invalid day"),
    }
}

pub fn solve_all_puzzles(data: &Vec<String>) -> Vec<(String, String)> {
    data.iter().enumerate()
        .map(|(day, day_data)| {
            let puzzle = get_puzzle((day + 1) as u8, day_data);
            (puzzle.part_1(), puzzle.part_2())
        })
        .collect()
}