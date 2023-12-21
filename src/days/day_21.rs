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
    row: usize,
    col: usize,
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
                                start = Some(Position { row, col });
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
        self.count_positions(64).to_string()
    }

    fn part_2(&self) -> String {
        self.count_positions_large(26501365).to_string()
    }
}


impl Puzzle {
    fn get_distances(&self, from: Position, steps: usize) -> Vec<Vec<Option<usize>>> {
        let size = self.grid.len();
        let mut distances = vec![vec![None; 2 * steps + 1]; 2 * steps + 1];
        if self.grid[from.row % size][from.col % size] == Ground::Rock {
            return distances;
        }

        distances[steps][steps] = Some(0);
        let offset = size - steps % size;
        let mut border = vec![(steps, steps)];
        for step in 1..=steps {
            let mut border_ = vec![];
            for (row, col) in border {
                for (row_, col_) in [(row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)] {
                    if distances[row_][col_].is_none() && self.grid[(from.row + row_ + offset) % size][(from.col + col_ + offset) % size] == Ground::Garden {
                        distances[row_][col_] = Some(step);
                        border_.push((row_, col_))
                    }
                }
            }
            border = border_;
        }
        distances
    }

    fn count_positions(&self, steps: usize) -> usize {
        let distances = self.get_distances(self.start, steps);
        distances.iter()
            .flat_map(|line| line.iter())
            .filter(|distance| distance.is_some_and(|d| d <= steps && d % 2 == steps % 2))
            .count()
    }

    fn count_positions_large(&self, steps: usize) -> usize {
        let size = self.grid.len();
        assert_eq!(steps % size, size / 2);

        get_nth_term(|i| self.count_positions(size / 2 + i * size), steps / size)
    }
}

fn combinations(r: usize, n: usize) -> usize {
    let mut c = 1;
    for i in 0..r {
        c = c * (n - i) / (i + 1)
    }
    c
}

fn get_nth_term(f: impl Fn(usize) -> usize, n: usize) -> usize {
    let mut diffs = Vec::<isize>::new();
    let mut offset = 0;
    for i in 0.. {
        diffs.push(f(i) as isize);
        for j in (1..diffs.len()).rev() {
            diffs[j - 1] = diffs[j] - diffs[j - 1]
        }
        // println!("{diffs:?}");
        if let Some(i) = diffs.iter().position(|&v| v == 0) {
            diffs = diffs[i + 1..].to_vec();
            offset = i + 1;
            break;
        }
    }

    for i in 1..diffs.len() {
        for j in (i..diffs.len()).rev() {
            diffs[j] = diffs[j] - diffs[j - 1]
        }
    }

    // println!("{diffs:?}");
    diffs.into_iter().rev().enumerate()
        .map(|(r, d)| d * combinations(r, n - offset) as isize)
        .sum::<isize>() as usize
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

        assert_eq!(puzzle.count_positions(6), 16);
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.count_positions(6), 16);
        assert_eq!(puzzle.count_positions(10), 50);
        assert_eq!(puzzle.count_positions(50), 1594);
        assert_eq!(puzzle.count_positions(100), 6536);
        assert_eq!(puzzle.count_positions(500), 167004);
        assert_eq!(puzzle.count_positions(1000), 668697);
        // assert_eq!(puzzle.count_positions(5000), 16733044);

        assert_eq!(puzzle.count_positions(5 + 5 * 11), (81 * 5 + 67) * 5 - 36);
        assert_eq!(puzzle.count_positions_large(5 + 5 * 11), (81 * 5 + 67) * 5 - 36);
        assert_eq!(puzzle.count_positions_large(5 + 7 * 11), (81 * 7 + 67) * 7 - 36);
    }
}