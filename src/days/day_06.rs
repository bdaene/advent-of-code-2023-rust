use nom::character::complete;
use nom::IResult;
use nom::multi::separated_list1;
use nom::Parser;
use nom::sequence::separated_pair;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    times: Vec<u32>,
    distances: Vec<u32>,
}

impl Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            tag("Time:").precedes(complete::multispace1).precedes(separated_list1(complete::multispace1, complete::u32)),
            complete::line_ending,
            tag("Distance:").precedes(complete::multispace1).precedes(separated_list1(complete::multispace1, complete::u32)),
        )
            .map(|(times, distances)| Self { times, distances })
            .parse(input)
    }
}

fn get_number_of_ways(time: u32, distance: u64) -> u32 {
    let t2 = (time /2) as f64;
    let d = distance as f64;
    let delta = (t2.powf(2.) - d).powf(0.5);
    let left = (t2 - delta + 1.).floor() as u32;
    let right = (t2 + delta - 1.).ceil() as u32;
    right - left + 1
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle::parse(data).unwrap().1
    }

    fn part_1(&self) -> String {
        self.times.iter().zip(self.distances.iter())
            .map(|(time, distance)| get_number_of_ways(*time, *distance as u64))
            .product::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let time: u32 = self.times.iter().map(u32::to_string).collect::<Vec<String>>().join("").parse().unwrap();
        let distance: u64 = self.distances.iter().map(u32::to_string).collect::<Vec<String>>().join("").parse().unwrap();
        get_number_of_ways(time, distance).to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_06.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle { times: vec![7, 15, 30], distances: vec![9, 40, 200] })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "288");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "Not implemented yet.");
    }
}