use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom_supreme::ParserExt;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct CubeSubset {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    id: u32,
    cube_subsets: Vec<CubeSubset>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    games: Vec<Game>,
}

impl Color {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("red").value(Color::Red),
            tag("green").value(Color::Green),
            tag("blue").value(Color::Blue),
        ))
            .parse(input)
    }
}

impl CubeSubset {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            tag(", "),
            separated_pair(complete::u32, complete::space1, Color::parse),
        )
            .map(|cube_subsets| {
                cube_subsets
                    .iter()
                    .fold(CubeSubset::default(), |acc, (n, color)| match color {
                        Color::Red => CubeSubset { red: acc.red + n, ..acc },
                        Color::Green => CubeSubset { green: acc.green + n, ..acc },
                        Color::Blue => CubeSubset { blue: acc.blue + n, ..acc },
                    })
            })
            .parse(input)
    }

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            tag("Game ").precedes(complete::u32),
            tag(": "),
            separated_list1(tag("; "), CubeSubset::parse),
        )
            .map(|(id, cube_subsets)| Self { id, cube_subsets })
            .parse(input)
    }

    fn is_possible(&self) -> bool {
        self.cube_subsets
            .iter()
            .all(|cube_subset| {
                cube_subset.red <= 12 && cube_subset.green <= 13 && cube_subset.blue <= 14
            })
    }

    fn get_min_cube_subset(&self) -> CubeSubset {
        self.cube_subsets
            .iter()
            .fold(CubeSubset::default(), |acc, cube_subset| CubeSubset {
                red: acc.red.max(cube_subset.red),
                green: acc.green.max(cube_subset.green),
                blue: acc.blue.max(cube_subset.blue),
            })
    }
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(complete::line_ending, Game::parse)
            .map(|games| Self { games })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.games
            .iter()
            .filter(|game| game.is_possible())
            .map(|game| game.id)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.games
            .iter()
            .map(|game| game.get_min_cube_subset().get_power())
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_solution() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_02.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(solution, Puzzle {
            games: vec![
                Game {
                    id: 1,
                    cube_subsets: vec![
                        CubeSubset { red: 4, blue: 3, green: 0 },
                        CubeSubset { red: 1, blue: 6, green: 2 },
                        CubeSubset { red: 0, blue: 0, green: 2 },
                    ],
                },
                Game {
                    id: 2,
                    cube_subsets: vec![
                        CubeSubset { red: 0, blue: 1, green: 2 },
                        CubeSubset { red: 1, blue: 4, green: 3 },
                        CubeSubset { red: 0, blue: 1, green: 1 },
                    ],
                },
                Game {
                    id: 3,
                    cube_subsets: vec![
                        CubeSubset { red: 20, blue: 6, green: 8 },
                        CubeSubset { red: 4, blue: 5, green: 13 },
                        CubeSubset { red: 1, blue: 0, green: 5 },
                    ],
                },
                Game {
                    id: 4,
                    cube_subsets: vec![
                        CubeSubset { red: 3, blue: 6, green: 1 },
                        CubeSubset { red: 6, blue: 0, green: 3 },
                        CubeSubset { red: 14, blue: 15, green: 3 },
                    ],
                },
                Game {
                    id: 5,
                    cube_subsets: vec![
                        CubeSubset { red: 6, blue: 1, green: 3 },
                        CubeSubset { red: 1, blue: 2, green: 2 },
                    ],
                },
            ]
        })
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "8");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "2286");
    }

    #[test]
    fn cube_subset_parse() {
        assert_eq!(CubeSubset::parse("1 red, 2 green, 3 blue"), Ok(("", CubeSubset { red: 1, green: 2, blue: 3 })));
        assert_eq!(CubeSubset::parse("4 blue"), Ok(("", CubeSubset { blue: 4, ..CubeSubset::default() })));
    }
}
