use std::collections::HashMap;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    grid: Vec<Vec<Cell>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Cell {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            many1(alt((
                tag(".").value(Cell::Path),
                tag("#").value(Cell::Forest),
                tag("^").value(Cell::Slope(Direction::Up)),
                tag(">").value(Cell::Slope(Direction::Right)),
                tag("v").value(Cell::Slope(Direction::Down)),
                tag("<").value(Cell::Slope(Direction::Left)),
            ))),
        )
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let (start, _end, graph) = self.extract_graph();
        let ordered_positions = get_topological_sort(&graph);

        let mut distance_to_end = HashMap::new();
        ordered_positions.iter().rev()
            .for_each(|position| {
                let distance = if let Some(destinations) = graph.get(position) {
                    destinations.keys()
                        .map(|position| distance_to_end[position] + destinations[position])
                        .max().unwrap_or(0)
                } else { 0 };
                distance_to_end.insert(position, distance);
            });

        distance_to_end[&start].to_string()
    }

    fn part_2(&self) -> String {
        let (start, end, mut graph) = self.extract_graph();

        extend(&mut graph);

        let mut stack = vec![(0, start, vec![])];
        let mut best = 0;

        while let Some((distance, position, seen_positions)) = stack.pop() {
            if position == end {
                best = best.max(distance);
                continue;
            }
            let mut seen_positions_ = seen_positions.to_vec();
            seen_positions_.push(position);
            if let Some(destinations) = graph.get(&position) {
                stack.extend(
                    destinations.keys().copied()
                        .filter(|position| !seen_positions_.contains(position))
                        .map(|position| (distance + destinations[&position], position, seen_positions_.to_vec()))
                )
            }
        }
        best.to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

type Graph = HashMap<Position, HashMap<Position, usize>>;

impl Puzzle {
    fn extract_graph(&self) -> (Position, Position, Graph) {
        let start = self.grid[0].iter().copied().enumerate()
            .filter(|&(_, cell)| cell == Cell::Path).next().expect("Should be a start!").0;
        let end = self.grid[self.grid.len() - 1].iter().copied().enumerate()
            .filter(|&(_, cell)| cell == Cell::Path).next().expect("Should be a start!").0;
        let start = Position { row: 0, col: start };
        let end = Position { row: self.grid.len() - 1, col: end };

        let mut graph = HashMap::new();
        let mut stack = vec![(start, Direction::Down)];
        while let Some((position, direction)) = stack.pop() {
            if position == end {
                continue;
            }
            let (distance, destination) = self.get_path(position, direction);
            if !graph.contains_key(&position) {
                graph.insert(position, HashMap::from([(destination, distance)]));
            } else {
                graph.get_mut(&position).unwrap().insert(destination, distance);
            }

            if destination != end {
                stack.extend(DIRECTIONS.iter().copied()
                    .map(|direction| (destination.follow(direction), direction))
                    .filter(|&(position, direction)| self.grid[position.row][position.col] == Cell::Slope(direction))
                    .map(|(_, direction)| (destination, direction))
                );
            }
        }

        (start, end, graph)
    }

    fn get_path(&self, from: Position, direction: Direction) -> (usize, Position) {
        let mut count = 1;
        let mut position = from.follow(direction);
        let mut direction = direction;
        if self.grid[position.row][position.col] == Cell::Slope(direction) {
            count += 1;
            position = position.follow(direction)
        }
        while position.row + 1 < self.grid.len() && self.grid[position.row][position.col] != Cell::Slope(direction) {
            count += 1;
            (direction, position) = direction.get_next_directions().into_iter()
                .map(|direction| (direction, position.follow(direction)))
                .filter(|(_, position)| self.grid[position.row][position.col] != Cell::Forest)
                .next()
                .expect("No dead end.");
        }
        if position.row != self.grid.len() - 1 {
            count += 1;
            position = position.follow(direction);
        }
        (count, position)
    }
}

impl Position {
    fn follow(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Position { row: self.row - 1, col: self.col },
            Direction::Down => Position { row: self.row + 1, col: self.col },
            Direction::Left => Position { row: self.row, col: self.col - 1 },
            Direction::Right => Position { row: self.row, col: self.col + 1 },
        }
    }
}

impl Direction {
    fn get_next_directions(&self) -> [Self; 3] {
        match self {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Left, Direction::Up, Direction::Down],
            Direction::Right => [Direction::Right, Direction::Up, Direction::Down],
        }
    }
}

fn get_topological_sort(graph: &Graph) -> Vec<Position> {
    let mut parent_count = HashMap::new();
    graph.values()
        .for_each(|destinations| destinations.keys()
            .for_each(|position| {
                if let Some(count) = parent_count.get_mut(position) {
                    *count += 1;
                } else {
                    parent_count.insert(position, 1);
                }
            })
        );

    let mut stack: Vec<Position> = Vec::from_iter(graph.keys().copied().filter(|position| !parent_count.contains_key(position)));
    let mut ordered_positions = vec![];
    while let Some(position) = stack.pop() {
        ordered_positions.push(position);
        if let Some(destinations) = graph.get(&position) {
            destinations.keys()
                .for_each(|position| {
                    if let Some(count) = parent_count.get_mut(position) {
                        *count -= 1;
                        if *count == 0 {
                            stack.push(*position);
                        }
                    }
                })
        }
    }
    ordered_positions
}

fn extend(graph: &mut Graph) {
    let new_keys = Vec::from_iter(graph.keys().copied()
        .flat_map(|position_a| {
            let destinations = graph.get(&position_a).unwrap();
            destinations.keys().copied()
                .map(move |position_b: Position| (position_b, (destinations[&position_b], position_a)))
        })
    );

    new_keys.into_iter()
        .for_each(|(position_b, (distance, position_a))| {
            if !graph.contains_key(&position_b) {
                graph.insert(position_b, HashMap::from([(position_a, distance)]));
            } else {
                graph.get_mut(&position_b).unwrap().insert(position_a, distance);
            }
        })
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_23.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            grid: vec![
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest, Cell::Path, Cell::Slope(Direction::Right), Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest, Cell::Path, Cell::Forest, Cell::Slope(Direction::Down), Cell::Forest, Cell::Forest, Cell::Forest],
                vec![Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest, Cell::Path, Cell::Path, Cell::Path, Cell::Forest],
                vec![Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Forest, Cell::Path, Cell::Forest]]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "94");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "154");
    }
}