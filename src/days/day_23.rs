use std::collections::{HashMap, HashSet};

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
        let graph = self.extract_graph();

        let mut distance_to_end = vec![0usize; graph.nodes.len()];
        graph.get_topological_sort().into_iter().rev()
            .for_each(|index| {
                let distance = graph.nodes[index].successors.iter().copied()
                    .map(|(successor, distance)| distance_to_end[successor] + distance)
                    .max()
                    .unwrap_or(0);
                distance_to_end[index] = distance;
            });

        distance_to_end[graph.start].to_string()
    }

    fn part_2(&self) -> String {
        let graph = self.extract_graph().extended();

        let mut stack = vec![(1u64 << graph.start, 0, graph.start)];
        let mut best = 0;

        while let Some((seen, distance, index)) = stack.pop() {
            if index == graph.end {
                best = best.max(distance);
                continue;
            }

            stack.extend(graph.nodes[index].successors.iter()
                .filter(|&(successor, _dist)| (seen & 1 << successor) == 0)
                .map(|&(successor, dist)| (seen | 1 << successor, distance + dist, successor))
            );
        }

        best.to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, Ord, PartialOrd)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    nodes: Vec<Node>,
    start: NodeIndex,
    end: NodeIndex,
}

type NodeIndex = usize;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: Position,
    successors: Vec<(NodeIndex, usize)>,
}

impl Puzzle {
    fn extract_graph(&self) -> Graph {
        let start = self.grid[0].iter().copied().enumerate()
            .filter(|&(_, cell)| cell == Cell::Path).next().expect("Should be a start!").0;
        let end = self.grid[self.grid.len() - 1].iter().copied().enumerate()
            .filter(|&(_, cell)| cell == Cell::Path).next().expect("Should be a start!").0;
        let start = Position { row: 0, col: start };
        let end = Position { row: self.grid.len() - 1, col: end };

        let mut nodes: Vec<Node> = Vec::from([Node::new(start)]);
        let mut indexes: HashMap<Position, usize> = HashMap::from([(start, 0)]);
        let mut visited: HashSet<(Position, Direction)> = HashSet::new();

        let mut stack = vec![(start, Direction::Down)];
        while let Some((position, direction)) = stack.pop() {
            if position == end {
                continue;
            }
            if !visited.insert((position, direction)) {
                continue;
            }
            let (distance, destination) = self.get_path(position, direction);
            if !indexes.contains_key(&destination) {
                indexes.insert(destination, nodes.len());
                nodes.push(Node::new(destination));
            }
            nodes[indexes[&position]].successors.push((indexes[&destination], distance));

            if destination != end {
                stack.extend(DIRECTIONS.iter().copied()
                    .map(|direction| (destination.follow(direction), direction))
                    .filter(|&(position, direction)| self.grid[position.row][position.col] == Cell::Slope(direction))
                    .map(|(_, direction)| (destination, direction))
                );
            }
        }

        Graph { nodes, start: 0, end: indexes[&end] }
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

impl Node {
    fn new(position: Position) -> Self {
        Self { position, successors: Vec::new() }
    }
}

impl Graph {
    fn get_topological_sort(&self) -> Vec<NodeIndex> {
        let mut parent_count = vec![0usize; self.nodes.len()];
        self.nodes.iter()
            .for_each(|node| node.successors.iter().copied()
                .for_each(|(successor, _distance)| {
                    parent_count[successor] += 1;
                })
            );

        let mut stack: Vec<usize> = Vec::from_iter(parent_count.iter().copied().enumerate()
            .filter(|&(_index, count)| count == 0)
            .map(|(index, _count)| index));
        let mut ordered_indexes = vec![];
        while let Some(index) = stack.pop() {
            ordered_indexes.push(index);
            self.nodes[index].successors.iter().copied()
                .for_each(|(successor, _distance)| {
                    parent_count[successor] -= 1;
                    if parent_count[successor] == 0 {
                        stack.push(successor)
                    }
                });
        }
        ordered_indexes
    }

    fn extended(&self) -> Graph {
        let mut nodes: Vec<Node> = self.nodes.iter()
            .map(|node| Node { position: node.position, successors: node.successors.to_vec() })
            .collect();

        self.nodes.iter().enumerate()
            .for_each(|(index, node)| node.successors.iter().copied()
                .for_each(|(successor, distance)| {
                    nodes[successor].successors.push((index, distance))
                }));

        Graph { nodes, start: self.start, end: self.end }
    }
}


// fn extend(graph: &mut Graph) {
//     let new_keys = Vec::from_iter(graph.keys().copied()
//         .flat_map(|position_a| {
//             let destinations = graph.get(&position_a).unwrap();
//             destinations.keys().copied()
//                 .map(move |position_b: Position| (position_b, (destinations[&position_b], position_a)))
//         })
//     );
//
//     new_keys.into_iter()
//         .for_each(|(position_b, (distance, position_a))| {
//             if !graph.contains_key(&position_b) {
//                 graph.insert(position_b, HashMap::from([(position_a, distance)]));
//             } else {
//                 graph.get_mut(&position_b).unwrap().insert(position_a, distance);
//             }
//         })
// }

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