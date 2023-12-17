use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::ops::RangeInclusive;

use nom::{IResult, Parser};
use nom::character::complete;
use nom::multi::separated_list1;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    grid: Vec<Vec<u32>>,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            complete::digit1,
        )
            .map(|lines| Self {
                grid: lines.into_iter()
                    .map(|line: &str| line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect()
                    )
                    .collect()
            })
            .parse(input)
    }

    fn part_1(&self) -> String {
        get_minimal_heat_loss(&self.grid, &(1..=3)).to_string()
    }

    fn part_2(&self) -> String {
        get_minimal_heat_loss(&self.grid, &(4..=10)).to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    position: Position,
    to_horizontal: bool,
    heat_loss: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn gen_positions(&self, direction: Direction, grid: &Vec<Vec<u32>>, wobbly: &RangeInclusive<usize>) -> Vec<(Position, u32)> {
        let (height, width) = (grid.len(), grid[0].len());

        let mut heat_loss = (1..*wobbly.start())
            .filter_map(|distance| self.position.get_at(distance, direction, height, width))
            .fold(
                self.heat_loss,
                |heat_loss, position| heat_loss + grid[position.row][position.col],
            );

        wobbly.clone()
            .filter_map(|distance| self.position.get_at(distance, direction, height, width))
            .map(|position| {
                heat_loss += grid[position.row][position.col];
                (position, heat_loss)
            })
            .collect()
    }

    fn get_next_states(&self, grid: &Vec<Vec<u32>>, wobbly: &RangeInclusive<usize>) -> Vec<Self> {
        let directions = if self.to_horizontal {
            [Direction::Left, Direction::Right]
        } else {
            [Direction::Up, Direction::Down]
        };
        directions.into_iter()
            .flat_map(|direction| self.gen_positions(direction, grid, wobbly))
            .map(|(position, heat_loss)| Self { position, to_horizontal: !self.to_horizontal, heat_loss })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_at(&self, distance: usize, direction: Direction, height: usize, width: usize) -> Option<Self> {
        match direction {
            Direction::Up => Some(Self { row: self.row.checked_sub(distance)?, col: self.col }),
            Direction::Left => Some(Self { row: self.row, col: self.col.checked_sub(distance)? }),
            Direction::Down => (self.row + distance < height).then(|| Self { row: self.row + distance, col: self.col }),
            Direction::Right => (self.col + distance < width).then(|| Self { row: self.row, col: self.col + distance }),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


fn get_minimal_heat_loss(grid: &Vec<Vec<u32>>, wobbly: &RangeInclusive<usize>) -> u32 {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.extend([true, false].into_iter()
        .map(|to_horizontal| State { position: Position { row: 0, col: 0 }, heat_loss: 0, to_horizontal })
    );

    let target = Position { row: grid.len() - 1, col: grid[0].len() - 1 };
    let mut seen: HashSet<(Position, bool)> = HashSet::new();
    while let Some(state) = heap.pop() {
        if !seen.insert((state.position, state.to_horizontal)) {
            continue;
        }
        if state.position == target {
            return state.heat_loss;
        };
        heap.extend(state.get_next_states(grid, &wobbly).into_iter());
    }
    u32::MAX
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_17.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            grid: vec![
                vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
                vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
                vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
                vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
                vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
                vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
                vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
                vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
                vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
                vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
                vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
                vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
                vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "102");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "94");
    }
}
