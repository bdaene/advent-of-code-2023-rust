use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    grid: Vec<Vec<Option<Object>>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Object {
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Mirror {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Splitter {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct LightBeam {
    row: usize,
    col: usize,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            many1(alt((
                tag(".").value(None),
                tag("/").value(Some(Object::Mirror(Mirror::Ascending))),
                tag("\\").value(Some(Object::Mirror(Mirror::Descending))),
                tag("|").value(Some(Object::Splitter(Splitter::Vertical))),
                tag("-").value(Some(Object::Splitter(Splitter::Horizontal))),
            ))),
        )
            .map(|grid| Self { grid })
            .parse(input)
    }

    fn part_1(&self) -> String {
        energize(&self.grid, LightBeam { row: 0, col: 0, direction: Direction::Right })
            .to_string()
    }

    fn part_2(&self) -> String {
        let (height, width) = (self.grid.len(), self.grid[0].len());

        0
            .max(
                (0..height)
                    .map(|row| energize(&self.grid, LightBeam { row, col: 0, direction: Direction::Right }))
                    .max().unwrap()
            )
            .max(
                (0..width)
                    .map(|col| energize(&self.grid, LightBeam { row: 0, col, direction: Direction::Down }))
                    .max().unwrap()
            )
            .max(
                (0..height)
                    .map(|row| energize(&self.grid, LightBeam { row, col: width - 1, direction: Direction::Left }))
                    .max().unwrap()
            )
            .max(
                (0..width)
                    .map(|col| energize(&self.grid, LightBeam { row: height - 1, col, direction: Direction::Up }))
                    .max().unwrap()
            )
            .to_string()
    }
}

impl Direction {
    fn bounce(&self, object: Object) -> [Option<Direction>; 2] {
        match object {
            Object::Mirror(mirror) => match (mirror, self) {
                (Mirror::Ascending, Direction::Right) | (Mirror::Descending, Direction::Left) => [Some(Direction::Up), None],
                (Mirror::Ascending, Direction::Left) | (Mirror::Descending, Direction::Right) => [Some(Direction::Down), None],
                (Mirror::Ascending, Direction::Up) | (Mirror::Descending, Direction::Down) => [Some(Direction::Right), None],
                (Mirror::Ascending, Direction::Down) | (Mirror::Descending, Direction::Up) => [Some(Direction::Left), None],
            },
            Object::Splitter(splitter) => match (splitter, self) {
                (Splitter::Horizontal, Direction::Up) | (Splitter::Horizontal, Direction::Down) => [Some(Direction::Left), Some(Direction::Right)],
                (Splitter::Vertical, Direction::Left) | (Splitter::Vertical, Direction::Right) => [Some(Direction::Up), Some(Direction::Down)],
                _ => [Some(*self), None]
            }
        }
    }
}

impl LightBeam {
    fn update(&self, direction: Direction, limits: (usize, usize)) -> Option<Self> {
        let (height, width) = limits;
        Some(match direction {
            Direction::Up => LightBeam { row: self.row.checked_sub(1)?, col: self.col, direction },
            Direction::Left => LightBeam { row: self.row, col: self.col.checked_sub(1)?, direction },
            Direction::Down => (self.row + 1 < height).then_some(LightBeam { row: self.row + 1, col: self.col, direction })?,
            Direction::Right => (self.col + 1 < width).then_some(LightBeam { row: self.row, col: self.col + 1, direction })?,
        })
    }

    fn bounce(&self, object: Object, limits: (usize, usize)) -> [Option<LightBeam>; 2] {
        self.direction.bounce(object)
            .map(|direction| direction.and_then(|direction| self.update(direction, limits)))
    }
}


fn energize(grid: &Vec<Vec<Option<Object>>>, light_beam: LightBeam) -> usize {
    let limits = (grid.len(), grid[0].len());
    let (height, width) = limits;
    let mut light_beams = vec![light_beam];

    let mut energized = vec![vec![0u8; width]; height];

    while let Some(light_beam) = light_beams.pop() {
        let flag = 1u8 << light_beam.direction as u8;
        if energized[light_beam.row][light_beam.col] & flag == 0 {
            energized[light_beam.row][light_beam.col] |= flag;
            match grid[light_beam.row][light_beam.col] {
                None => light_beam.update(light_beam.direction, limits).into_iter().for_each(|light_beam| light_beams.push(light_beam)),
                Some(object) => light_beams.extend(light_beam.bounce(object, limits).into_iter().flatten())
            }
        }
    }

    energized.into_iter().flat_map(|line| line.into_iter().filter(|&cell| cell != 0)).count()
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_16.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            grid: vec![
                vec![None, Some(Object::Splitter(Splitter::Vertical)), None, None, None, Some(Object::Mirror(Mirror::Descending)), None, None, None, None],
                vec![Some(Object::Splitter(Splitter::Vertical)), None, Some(Object::Splitter(Splitter::Horizontal)), None, Some(Object::Mirror(Mirror::Descending)), None, None, None, None, None],
                vec![None, None, None, None, None, Some(Object::Splitter(Splitter::Vertical)), Some(Object::Splitter(Splitter::Horizontal)), None, None, None],
                vec![None, None, None, None, None, None, None, None, Some(Object::Splitter(Splitter::Vertical)), None],
                vec![None, None, None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None, None, Some(Object::Mirror(Mirror::Descending))],
                vec![None, None, None, None, Some(Object::Mirror(Mirror::Ascending)), None, Some(Object::Mirror(Mirror::Descending)), Some(Object::Mirror(Mirror::Descending)), None, None],
                vec![None, Some(Object::Splitter(Splitter::Horizontal)), None, Some(Object::Splitter(Splitter::Horizontal)), Some(Object::Mirror(Mirror::Ascending)), None, None, Some(Object::Splitter(Splitter::Vertical)), None, None],
                vec![None, Some(Object::Splitter(Splitter::Vertical)), None, None, None, None, Some(Object::Splitter(Splitter::Horizontal)), Some(Object::Splitter(Splitter::Vertical)), None, Some(Object::Mirror(Mirror::Descending))],
                vec![None, None, Some(Object::Mirror(Mirror::Ascending)), Some(Object::Mirror(Mirror::Ascending)), None, Some(Object::Splitter(Splitter::Vertical)), None, None, None, None],
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "46");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "51");
    }
}