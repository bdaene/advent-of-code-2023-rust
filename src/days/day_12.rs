use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    records: Vec<Record>,
}

#[derive(Debug, PartialEq, Eq)]
struct Record {
    springs: Vec<SpringState>,
    groups: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            Record::parse,
        )
            .map(|records| Self { records })
            .parse(input)
    }

    fn part_1(&self) -> String {
        self.records.iter()
            .map(|record| record.count_possible_arrangements())
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.records.iter()
            .map(|record| record.unfold().count_possible_arrangements())
            .sum::<usize>()
            .to_string()
    }
}

impl Record {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            many1(alt((
                tag(".").value(SpringState::Operational),
                tag("#").value(SpringState::Damaged),
                tag("?").value(SpringState::Unknown),
            ))),
            complete::space1,
            separated_list1(
                tag(","),
                complete::u8,
            ),
        )
            .map(|(springs, groups)| Self { springs, groups: groups.iter().map(|&size| size as usize).collect() })
            .parse(input)
    }

    fn count_possible_arrangements(&self) -> usize {
        count_possible_arrangements(&self.springs, &self.groups)
    }

    fn unfold(&self) -> Self {
        let mut springs = self.springs.to_vec();
        let mut groups = self.groups.to_vec();

        for _ in 1..5 {
            springs.push(SpringState::Unknown);
            springs.extend_from_slice(&self.springs);
            groups.extend_from_slice(&self.groups);
        }

        Self { springs, groups }
    }
}

fn count_possible_arrangements(springs: &[SpringState], groups: &[usize]) -> usize {
    let mut count = vec![vec![0; springs.len() + 1]; groups.len() + 1];

    count[0][0] = 1;

    for (j, &spring) in springs.iter().enumerate() {
        count[0][j + 1] = if spring != SpringState::Damaged { count[0][j] } else { 0 };
    }

    for (i, &group) in groups.iter().enumerate() {
        for (j, &spring) in springs.iter().enumerate() {
            if spring != SpringState::Damaged {
                count[i+1][j+1] += count[i+1][j]
            }

            if j+1 >= group && springs[j+1-group..=j].iter().all(|&spring_| spring_ != SpringState::Operational) {
                if j+1 == group {
                    count[i+1][j+1] += count[i][j+1-group];
                } else if springs[j-group] != SpringState::Damaged {
                    count[i+1][j+1] += count[i][j-group];
                }
            }
        }
    }

    count[groups.len()][springs.len()]
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_12.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            records: vec![
                Record { springs: vec![SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Operational, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged], groups: vec![1, 1, 3] },
                Record { springs: vec![SpringState::Operational, SpringState::Unknown, SpringState::Unknown, SpringState::Operational, SpringState::Operational, SpringState::Unknown, SpringState::Unknown, SpringState::Operational, SpringState::Operational, SpringState::Operational, SpringState::Unknown, SpringState::Damaged, SpringState::Damaged, SpringState::Operational], groups: vec![1, 1, 3] },
                Record { springs: vec![SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown, SpringState::Damaged, SpringState::Unknown], groups: vec![1, 3, 1, 6] },
                Record { springs: vec![SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Operational, SpringState::Damaged, SpringState::Operational, SpringState::Operational, SpringState::Operational, SpringState::Damaged, SpringState::Operational, SpringState::Operational, SpringState::Operational], groups: vec![4, 1, 1] },
                Record { springs: vec![SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Operational, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Operational, SpringState::Operational, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Operational], groups: vec![1, 6, 5] },
                Record { springs: vec![SpringState::Unknown, SpringState::Damaged, SpringState::Damaged, SpringState::Damaged, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown, SpringState::Unknown], groups: vec![3, 2, 1] },
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "21");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "525152");
    }

    #[test]
    fn test_count_possible_arrangements() {
        assert_eq!(Record::parse("???.### 1,1,3").unwrap().1.count_possible_arrangements(), 1);
        assert_eq!(Record::parse(".??..??...?##. 1,1,3").unwrap().1.count_possible_arrangements(), 4);
    }
}