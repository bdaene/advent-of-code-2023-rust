use std::collections::HashMap;
use std::hash::Hash;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

type Platform = Vec<Vec<Rock>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    platform: Platform,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            line_ending,
            many1(alt((
                tag("O").value(Rock::Round),
                tag("#").value(Rock::Cube),
                tag(".").value(Rock::Empty),
            ))),
        )
            .map(|platform| Self { platform })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let mut offsets = vec![0; self.platform[0].len()];

        let mut total = 0;
        for (row, line) in self.platform.iter().enumerate() {
            for (col, &rock) in line.iter().enumerate() {
                if rock == Rock::Round {
                    total += self.platform.len() - offsets[col];
                    offsets[col] += 1;
                } else if rock == Rock::Cube {
                    offsets[col] = row + 1;
                }
            }
        }

        total.to_string()
    }

    fn part_2(&self) -> String {
        let mut platform = self.platform.to_vec();

        let mut known_platforms = HashMap::new();
        for cycle in 0..1_000_000_000 {
            if let Some(previous_cycle) = known_platforms.insert(compute_hash(&platform), cycle) {
                let cycle_length = cycle - previous_cycle;
                for _ in 0..((1_000_000_000 - cycle) % cycle_length) {
                    platform = cycle_tilts(&platform)
                }
                break;
            }
            platform = cycle_tilts(&platform)
        }

        get_north_load(&platform).to_string()
    }
}

fn compute_hash(platform: &Platform) -> usize {
    platform.iter().enumerate()
        .flat_map(|(row, line)| line.iter().enumerate()
            .map(move |(col, rock)| match rock {
                Rock::Round => (row << 24) + col,
                _ => 0
            })
        )
        .sum()
}

fn tilt_north(platform: &Platform) -> Platform {
    let (height, width) = (platform.len(), platform[0].len());
    let mut tilted_platform = vec![vec![Rock::Empty; width]; height];

    let mut offsets = vec![0; width];
    for row in 0..width {
        for col in 0..height {
            match platform[row][col] {
                Rock::Empty => (),
                Rock::Cube => {
                    tilted_platform[row][col] = Rock::Cube;
                    offsets[col] = row + 1;
                }
                Rock::Round => {
                    tilted_platform[offsets[col]][col] = Rock::Round;
                    offsets[col] += 1;
                }
            }
        }
    }

    tilted_platform
}

fn tilt_west(platform: &Platform) -> Platform {
    let (height, width) = (platform.len(), platform[0].len());
    let mut tilted_platform = vec![vec![Rock::Empty; width]; height];

    let mut offsets = vec![0; height];
    for col in 0..width {
        for row in 0..height {
            match platform[row][col] {
                Rock::Empty => (),
                Rock::Cube => {
                    tilted_platform[row][col] = Rock::Cube;
                    offsets[row] = col + 1;
                }
                Rock::Round => {
                    tilted_platform[row][offsets[row]] = Rock::Round;
                    offsets[row] += 1;
                }
            }
        }
    }

    tilted_platform
}

fn tilt_south(platform: &Platform) -> Platform {
    let (height, width) = (platform.len(), platform[0].len());
    let mut tilted_platform = vec![vec![Rock::Empty; width]; height];

    let mut offsets = vec![height; width];
    for row in (0..width).rev() {
        for col in 0..height {
            match platform[row][col] {
                Rock::Empty => (),
                Rock::Cube => {
                    tilted_platform[row][col] = Rock::Cube;
                    offsets[col] = row;
                }
                Rock::Round => {
                    offsets[col] -= 1;
                    tilted_platform[offsets[col]][col] = Rock::Round;
                }
            }
        }
    }

    tilted_platform
}

fn tilt_east(platform: &Platform) -> Platform {
    let (height, width) = (platform.len(), platform[0].len());
    let mut tilted_platform = vec![vec![Rock::Empty; width]; height];

    let mut offsets = vec![width; height];
    for col in (0..height).rev() {
        for row in 0..width {
            match platform[row][col] {
                Rock::Empty => (),
                Rock::Cube => {
                    tilted_platform[row][col] = Rock::Cube;
                    offsets[row] = col;
                }
                Rock::Round => {
                    offsets[row] -= 1;
                    tilted_platform[row][offsets[row]] = Rock::Round;
                }
            }
        }
    }

    tilted_platform
}

fn cycle_tilts(platform: &Platform) -> Platform {
    let platform = tilt_north(&platform);
    let platform = tilt_west(&platform);
    let platform = tilt_south(&platform);
    let platform = tilt_east(&platform);
    platform
}


fn get_north_load(platform: &Platform) -> usize {
    let height = platform.len();
    platform.iter().enumerate()
        .flat_map(|(row, line)| line.iter()
            .map(move |rock| match rock {
                Rock::Round => height - row,
                _ => 0,
            })
        )
        .sum()
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_14.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            platform: vec![
                vec![Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty],
                vec![Rock::Round, Rock::Empty, Rock::Round, Rock::Round, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube],
                vec![Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty],
                vec![Rock::Round, Rock::Round, Rock::Empty, Rock::Cube, Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round],
                vec![Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round, Rock::Cube, Rock::Empty],
                vec![Rock::Round, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Cube],
                vec![Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Round, Rock::Empty, Rock::Empty, Rock::Round],
                vec![Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Round, Rock::Empty, Rock::Empty],
                vec![Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Cube, Rock::Cube, Rock::Empty, Rock::Empty],
                vec![Rock::Cube, Rock::Round, Rock::Round, Rock::Empty, Rock::Empty, Rock::Cube, Rock::Empty, Rock::Empty, Rock::Empty, Rock::Empty]]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "136");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "64");
    }
}