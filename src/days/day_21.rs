use std::collections::HashSet;

use nom::{IResult, Parser};
use nom::bytes::complete::take_till1;
use nom::character::complete;
use nom::multi::separated_list1;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    grid: Vec<Vec<Ground>>,
    start: Position,
}

#[derive(Debug, PartialEq, Eq)]
enum Ground {
    Garden,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    row: isize,
    col: isize,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            take_till1(|c| "\r\n".contains(c)),
        )
            .map(|grid: Vec<&str>| {
                let mut start = None;
                let grid = grid.iter().enumerate()
                    .map(|(row, line)| line.chars().enumerate()
                        .map(|(col, cell)| match cell {
                            'S' => {
                                start = Some(Position { row: row as isize, col: col as isize });
                                Ground::Garden
                            }
                            '.' => Ground::Garden,
                            '#' => Ground::Rock,
                            c => panic!("Unknown ground {}!", c)
                        })
                        .collect()
                    )
                    .collect();
                let start = start.expect("Should be a start.");
                Self { grid, start }
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.get_positions_in(64).len().to_string()
    }

    fn part_2(&self) -> String {
        for i in 1..10 {
            println!("{i} {}", self.get_positions_in(65 + 131 * i).len())
        }
        "".to_string()
    }
}

impl Puzzle {

    fn update_positions(&self, positions: &HashSet<Position>) -> HashSet<Position> {
        HashSet::from_iter(positions.iter().flat_map(|&position| self.get_next_positions(position)))
    }

    fn get_positions_in(&self, moves: usize) -> HashSet<Position> {
        let mut positions = HashSet::from([Position { row: 0, col: 0 }]);
        for _ in 0..moves {
            positions = self.update_positions(&positions);
        }
        positions
    }

    fn get_next_positions(&self, position: Position) -> Vec<Position> {
        let (height, width) = (self.grid.len() as isize, self.grid[0].len() as isize);
        [
            (position.row + 1, position.col),
            (position.row - 1, position.col),
            (position.row, position.col + 1),
            (position.row, position.col - 1),
        ]
            .into_iter()
            .filter(|&(row, col)| self.grid[(row + self.start.row).rem_euclid(height) as usize][(col + self.start.col).rem_euclid(width) as usize] == Ground::Garden)
            .map(|(row, col)| Position { row, col })
            .collect()
    }
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_21.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            start: Position { row: 5, col: 5 },
            grid: vec![
                vec![Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden],
                vec![Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Garden],
                vec![Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden],
                vec![Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Garden],
                vec![Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden, Ground::Rock, Ground::Rock, Ground::Garden],
                vec![Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden, Ground::Garden],
            ],
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_positions_in(6).len(), 16);
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_positions_in(6).len(), 16);
        assert_eq!(puzzle.get_positions_in(10).len(), 50);
        assert_eq!(puzzle.get_positions_in(50).len(), 1594);
        assert_eq!(puzzle.get_positions_in(100).len(), 6536);
        // assert_eq!(puzzle.get_positions_in(500).len(), 167004);
        // assert_eq!(puzzle.get_positions_in(1000).len(), 668697);
        // assert_eq!(puzzle.get_positions_in(5000).len(), 16733044);

        assert_eq!(puzzle.get_positions_in(5).len(), 13);
        assert_eq!(puzzle.get_positions_in(16).len(), 129);
        assert_eq!(puzzle.get_positions_in(27).len(), 427);
        assert_eq!(puzzle.get_positions_in(38).len(), 894);
        assert_eq!(puzzle.get_positions_in(49).len(), 1528);
        assert_eq!(puzzle.get_positions_in(5+11*5).len(), 2324);
        assert_eq!(puzzle.get_positions_in(5+11*6).len(), 3282);
        assert_eq!(puzzle.get_positions_in(5+11*7).len(), 4402);
        assert_eq!(puzzle.get_positions_in(5+11*8).len(), 5684);
        assert_eq!(puzzle.get_positions_in(5+11*9).len(), 7128);
        assert_eq!(puzzle.get_positions_in(5+11*10).len(), (81*10+67)*10-36);
        assert_eq!(puzzle.get_positions_in(5+11*11).len(), (81*11+67)*11-36);

    }
}