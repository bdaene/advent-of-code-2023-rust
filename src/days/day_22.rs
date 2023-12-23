use std::collections::{HashMap, HashSet};

use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    bricks: Vec<Brick>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Brick {
    start: Position,
    end: Position,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            Brick::parse,
        )
            .map(|mut bricks| {
                bricks.sort_by_key(|brick| brick.start.z);
                Self { bricks }
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let supports = self.get_supports();
        let mut actual_supports: HashSet<Brick> = HashSet::new();

        for brick in supports.keys() {
            if supports[brick].len() == 1 {
                actual_supports.insert(*supports[brick].iter().next().unwrap());
            }
        }

        (self.bricks.len() - actual_supports.len() + 1).to_string()
    }

    fn part_2(&self) -> String {
        let supports = self.get_supports();

        (0..self.bricks.len())
            .map(|i| count_falling(&self.bricks[i..], &supports))
            .sum::<usize>()
            .to_string()
    }
}

impl Puzzle {
    fn get_supports(&self) -> HashMap<Brick, HashSet<Brick>> {
        let max_x = self.bricks.iter().map(|brick| brick.end.x).max().expect("At least one brick");
        let max_y = self.bricks.iter().map(|brick| brick.end.y).max().expect("At least one brick");

        let ground_brick = Brick{start: Position{x:0,y:0,z:0}, end: Position{x:max_x, y:max_y, z:0}};

        let mut ground: Vec<Vec<(usize, Brick)>> = vec![vec![(0, ground_brick); max_y + 1]; max_x + 1];
        let mut supports = HashMap::new();

        for brick in self.bricks.iter().copied() {
            let below_bricks: HashSet<(usize, Brick)> = brick.iter_horizontal()
                .map(|(x, y)| ground[x][y])
                .collect();

            let max_height = below_bricks.iter().map(|(height, _)| height).copied().max().unwrap_or(0);
            let supports_: HashSet<Brick> = below_bricks.into_iter()
                .filter(|&(height, _)| { height == max_height })
                .map(|(_, brick)| brick)
                .collect();
            supports.insert(brick, supports_);

            let height = max_height + brick.end.z - brick.start.z + 1;
            brick.iter_horizontal()
                .for_each(|(x, y)| ground[x][y] = (height, brick));
        }

        supports
    }
}

impl Brick {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            Position::parse,
            tag("~"),
            Position::parse,
        )
            .map(|(start, end)| Self { start, end })
            .parse(input)
    }

    fn iter_horizontal(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        (self.start.x..=self.end.x).flat_map(|x| (self.start.y..=self.end.y).map(move |y| (x, y)))
    }
}

impl Position {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((
            complete::u16,
            tag(","),
            complete::u16,
            tag(","),
            complete::u16,
        ))
            .map(|(x, _, y, _, z)| Self { x: x as usize, y: y as usize, z: z as usize })
            .parse(input)
    }
}

fn count_falling(bricks: &[Brick], supports: &HashMap<Brick, HashSet<Brick>>) -> usize {
    let mut fallen = HashSet::from([bricks[0]]);

    for brick in bricks[1..].iter() {
        if let Some(brick_supports) = supports.get(brick) {
            if brick_supports.is_subset(&fallen) {
                fallen.insert(*brick);
            }
        }
    }

    fallen.len() - 1
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_22.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            bricks: vec![
                Brick { start: Position { x: 1, y: 0, z: 1 }, end: Position { x: 1, y: 2, z: 1 } },
                Brick { start: Position { x: 0, y: 0, z: 2 }, end: Position { x: 2, y: 0, z: 2 } },
                Brick { start: Position { x: 0, y: 2, z: 3 }, end: Position { x: 2, y: 2, z: 3 } },
                Brick { start: Position { x: 0, y: 0, z: 4 }, end: Position { x: 0, y: 2, z: 4 } },
                Brick { start: Position { x: 2, y: 0, z: 5 }, end: Position { x: 2, y: 2, z: 5 } },
                Brick { start: Position { x: 0, y: 1, z: 6 }, end: Position { x: 2, y: 1, z: 6 } },
                Brick { start: Position { x: 1, y: 1, z: 8 }, end: Position { x: 1, y: 1, z: 9 } },
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "5");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "7");
    }
}