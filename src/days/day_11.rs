use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    image: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            line_ending,
            many1(alt((
                tag(".").value('.'),
                tag("#").value('#'),
            ))),
        )
            .map(|image| Self { image })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.get_total_galaxies_distance(2).to_string()
    }

    fn part_2(&self) -> String {
        self.get_total_galaxies_distance(1_000_000).to_string()
    }
}

impl Puzzle {
    fn get_galaxies(&self) -> Vec<Position> {
        self.image.iter().enumerate()
            .flat_map(|(row, line)| {
                line.iter().enumerate()
                    .filter(|(_col, cell)| **cell == '#')
                    .map(move |(col, _cell)| Position { row, col })
            })
            .collect()
    }

    fn get_expanded_coordinates(&self, galaxies: &[Position], factor: usize) -> (Vec<usize>, Vec<usize>) {
        let mut empty_rows = vec![1usize; self.image.len()];
        let mut empty_cols = vec![1usize; self.image[0].len()];
        galaxies.iter().for_each(|galaxy| {
            empty_rows[galaxy.row] = 0;
            empty_cols[galaxy.col] = 0;
        });
        let expanded_rows = expand(&empty_rows, factor);
        let expanded_cols = expand(&empty_cols, factor);

        (expanded_rows, expanded_cols)
    }

    fn get_total_galaxies_distance(&self, factor: usize) -> usize {
        let galaxies = self.get_galaxies();
        let (expanded_rows, expanded_cols) = self.get_expanded_coordinates(&galaxies, factor);

        let mut expanded_galaxies_row: Vec<usize> = galaxies.iter().map(|galaxy| expanded_rows[galaxy.row]).collect();
        let total_row_distance = get_total_distance(&mut expanded_galaxies_row);

        let mut expanded_galaxies_col: Vec<usize> = galaxies.iter().map(|galaxy| expanded_cols[galaxy.col]).collect();
        let total_col_distance = get_total_distance(&mut expanded_galaxies_col);

        total_row_distance + total_col_distance
    }
}

fn expand(empty: &[usize], factor: usize) -> Vec<usize> {
    let mut offset = 0;
    let mut expanded = Vec::with_capacity(empty.len());

    empty.iter().for_each(|x| {
        expanded.push(offset);
        offset += 1 + x * (factor - 1);
    });
    expanded
}

fn get_total_distance(coordinates: &mut [usize]) -> usize {
    coordinates.sort_unstable();

    let mut total_distance = 0;
    let mut current_sum = 0;
    coordinates.iter().enumerate().for_each(|(i, coordinate)| {
        total_distance += i * coordinate - current_sum;
        current_sum += coordinate;
    });

    total_distance
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_11.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            image: vec![
                vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
                vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "374");
    }

    #[test]
    fn get_total_galaxies_distance() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_total_galaxies_distance(2), 374);
        assert_eq!(puzzle.get_total_galaxies_distance(10), 1030);
        assert_eq!(puzzle.get_total_galaxies_distance(100), 8410);
    }
}