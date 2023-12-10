use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    map: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    NS,
    WE,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
enum Direction { E, N, W, S }

const DIRECTIONS: [Direction; 4] = [Direction::E, Direction::N, Direction::W, Direction::S];

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
struct Position(usize, usize);

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            many1(Tile::parse),
        )
            .map(|map| Self { map })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let path = self.get_start_loop();

        (path.len() / 2).to_string()
    }

    fn part_2(&self) -> String {
        let path = self.get_start_loop();
        let map = self.replace_start();
        let mut loop_by_row = vec![vec![]; map.len()];
        for position in path {
            loop_by_row[position.0].push((position, map[position.0][position.1]));
        }
        let mut total = 0;
        for mut loop_row in loop_by_row {
            loop_row.sort_by_key(|(position, _)| position.1);
            let mut left = 0;
            let mut inside = false;
            while left < loop_row.len() {
                let (cut, right) = get_cut(&loop_row, left);
                inside ^= cut;

                if let Some((position, _)) = loop_row.get(right + 1) {
                    if inside {
                        total += position.1 - loop_row[right].0.1 - 1;
                    }
                }
                left = right+1;
            }
        }


        total.to_string()
    }
}

impl Puzzle {
    fn get_tile(&self, position: Position) -> Option<Tile> {
        let Position(row, col) = position;
        Some(*(self.map.get(row)?.get(col)?))
    }

    fn get_start(&self) -> Position {
        self.map.iter()
            .enumerate()
            .flat_map(|(row, line)| line.iter()
                .enumerate()
                .filter(|(_col, &tile)| tile == Tile::Start)
                .map(move |(col, _tile)| Position(row, col))
            )
            .next()
            .expect("Should be a start")
    }

    fn get_valid_directions(&self, position: Position) -> Vec<Direction> {
        DIRECTIONS.iter().copied()
            .filter(|direction| {
                if let Some(position) = direction.next_position(position) {
                    if let Some(tile) = self.get_tile(position) {
                        return tile != Tile::Ground && tile.next_direction(*direction).is_some();
                    }
                }
                false
            })
            .collect()
    }

    fn get_start_direction(&self, start: Position) -> Direction {
        *self.get_valid_directions(start)
            .first()
            .expect("There should be a valid direction from the start.")
    }

    fn get_start_loop(&self) -> Vec<Position> {
        let start = self.get_start();
        let start_direction = self.get_start_direction(start);

        let mut path = Vec::new();
        path.push(start);
        let mut position = start_direction.next_position(start).expect("Hit the border!");
        let mut direction = start_direction;
        while position != start {
            path.push(position);
            direction = self.get_tile(position).expect("In the void!").next_direction(direction).expect("Should be a loop!");
            position = direction.next_position(position).expect("Hit the border!")
        }
        path
    }

    fn replace_start(&self) -> Vec<Vec<Tile>> {
        let start = self.get_start();
        let mut map = self.map.to_vec();
        let mut start_directions = self.get_valid_directions(start);
        start_directions.sort();

        map[start.0][start.1] = match (start_directions[0], start_directions[1]) {
            (Direction::E, Direction::N) => Tile::NE,
            (Direction::E, Direction::W) => Tile::WE,
            (Direction::E, Direction::S) => Tile::SE,
            (Direction::N, Direction::W) => Tile::NW,
            (Direction::N, Direction::S) => Tile::NS,
            (Direction::W, Direction::S) => Tile::SW,
            _ => panic!("Invalid directions at start {:?}", start_directions)
        };

        map
    }
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        {
            alt((
                tag("|").value(Tile::NS),
                tag("-").value(Tile::WE),
                tag("L").value(Tile::NE),
                tag("J").value(Tile::NW),
                tag("7").value(Tile::SW),
                tag("F").value(Tile::SE),
                tag(".").value(Tile::Ground),
                tag("S").value(Tile::Start),
            ))
                .parse(input)
        }
    }

    fn next_direction(&self, direction: Direction) -> Option<Direction> {
        match self {
            Tile::Start => None,
            Tile::Ground => Some(direction),
            Tile::NE => match direction {
                Direction::S => Some(Direction::E),
                Direction::W => Some(Direction::N),
                _ => None
            }
            Tile::NW => match direction {
                Direction::S => Some(Direction::W),
                Direction::E => Some(Direction::N),
                _ => None
            }
            Tile::SW => match direction {
                Direction::N => Some(Direction::W),
                Direction::E => Some(Direction::S),
                _ => None
            }
            Tile::SE => match direction {
                Direction::N => Some(Direction::E),
                Direction::W => Some(Direction::S),
                _ => None
            }
            Tile::WE => match direction {
                Direction::E => Some(Direction::E),
                Direction::W => Some(Direction::W),
                _ => None
            }
            Tile::NS => match direction {
                Direction::S => Some(Direction::S),
                Direction::N => Some(Direction::N),
                _ => None
            }
        }
    }
}

impl Direction {
    fn next_position(&self, position: Position) -> Option<Position> {
        let Position(row, col) = position;
        if (*self == Direction::N && row == 0) || (*self == Direction::W && col == 0) {
            return None;
        }
        Some(match self {
            Direction::N => Position(row - 1, col),
            Direction::S => Position(row + 1, col),
            Direction::W => Position(row, col - 1),
            Direction::E => Position(row, col + 1),
        })
    }
}

fn get_cut(loop_row: &[(Position, Tile)], start: usize) -> (bool, usize) {
    if loop_row[start].1 == Tile::NS {
        return (true, start);
    }

    let length = loop_row[start..].iter().enumerate().skip(1)
        .filter(|(_, (_, tile))| *tile != Tile::WE)
        .next()
        .expect("The cut should end.")
        .0;

    let end = start + length;
    let cut = (loop_row[start].1 == Tile::NE && loop_row[end].1 == Tile::SW)
        || (loop_row[start].1 == Tile::SE && loop_row[end].1 == Tile::NW);

    (cut, end)
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle(i: usize) -> Puzzle {
        let data = fs::read_to_string(format!("data/examples/day_10_{i}.txt")).unwrap();

        Puzzle::new(&data)
    }


    #[test]
    fn new() {
        let puzzle = get_puzzle(1);

        assert_eq!(puzzle, Puzzle {
            map: vec![
                vec![Tile::SW, Tile::WE, Tile::SE, Tile::SW, Tile::WE],
                vec![Tile::Ground, Tile::SE, Tile::NW, Tile::NS, Tile::SW],
                vec![Tile::Start, Tile::NW, Tile::NE, Tile::NE, Tile::SW],
                vec![Tile::NS, Tile::SE, Tile::WE, Tile::WE, Tile::NW],
                vec![Tile::NE, Tile::NW, Tile::Ground, Tile::NE, Tile::NW],
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle(1);

        assert_eq!(puzzle.part_1(), "8");
    }

    #[test]
    fn part_2() {
        assert_eq!(get_puzzle(2).part_2(), "4");
        assert_eq!(get_puzzle(3).part_2(), "10");
    }

    #[test]
    fn test_get_cut() {
        assert_eq!(
            get_cut(&vec![
                (Position(0, 0), Tile::NS)
            ], 0),
            (true, 0)
        );

        assert_eq!(
            get_cut(&vec![
                (Position(0, 0), Tile::NE),
                (Position(0, 1), Tile::WE),
                (Position(0, 2), Tile::SW),
            ], 0),
            (true, 2)
        );

        assert_eq!(
            get_cut(&vec![
                (Position(0, 0), Tile::SE),
                (Position(0, 1), Tile::WE),
                (Position(0, 2), Tile::WE),
                (Position(0, 3), Tile::SW),
            ], 0),
            (false, 3

            )
        );

        assert_eq!(
            get_cut(&vec![
                (Position(0, 0), Tile::SE),
                (Position(0, 1), Tile::SW),
            ], 0),
            (false, 1)
        );

        assert_eq!(
            get_cut(&vec![
                (Position(4, 3), Tile::NS),
                (Position(4, 5), Tile::NE),
                (Position(4, 6), Tile::WE),
                (Position(4, 7), Tile::SW),
                (Position(4, 6), Tile::NS),
            ], 1),
            (true, 3)
        );
    }
}