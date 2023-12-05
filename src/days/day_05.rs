use nom::{IResult, Parser};
use nom::bytes::complete::take_until;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Slice {
    start: u32,
    length: u32,
}

impl Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            tag("seeds: ").precedes(
                separated_list1(
                    complete::space1,
                    complete::u32,
                )
            ),
            tuple((complete::line_ending, complete::line_ending)),
            separated_list1(
                tuple((complete::line_ending, complete::line_ending)),
                Map::parse,
            ),
        )
            .map(|(seeds, maps)| Self { seeds, maps })
            .parse(input)
    }
}


impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            take_until(" map:"),
            tag(" map:").precedes(complete::line_ending),
            separated_list1(
                complete::line_ending,
                Range::parse,
            ),
        )
            .map(|(name, mut ranges)| {
                ranges.sort_by_key(|range| range.source_start);
                Self { name: String::from(name), ranges }
            })
            .parse(input)
    }

    fn map(&self, source: u32) -> u32 {
        for range in self.ranges.iter() {
            if range.contains(source) {
                return range.map(source);
            }
        }
        source
    }


    fn map_slices(&self, source_slices: &Vec<Slice>) -> Vec<Slice> {
        let mut source_slices: Vec<Slice> = source_slices.to_vec();
        let mut destination_slices: Vec<Slice> = vec![];

        source_slices.sort_by_key(|source_range| source_range.start);
        source_slices.reverse();
        let mut map_ranges = self.ranges.iter();
        let mut current_range = map_ranges.next();
        while let Some(slice) = source_slices.pop() {
            while let Some(range) = current_range {
                if slice.start < range.source_start || range.contains(slice.start) { break; }
                current_range = map_ranges.next()
            }
            if let Some(range) = current_range {
                let (left_slice, right_slice) = if slice.start < range.source_start {
                    slice.split_at(range.source_start - slice.start)
                } else {
                    let (left_slice, right_slice) = slice.split_at(range.length - (slice.start - range.source_start));
                    let left_slice = Slice {
                        start: range.map(left_slice.start),
                        length: left_slice.length,
                    };
                    (left_slice, right_slice)
                };
                destination_slices.push(left_slice);
                if let Some(right_slice) = right_slice {
                    source_slices.push(right_slice)
                }
            } else {
                destination_slices.push(slice)
            }
        }
        destination_slices
    }
}

impl Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((
            complete::u32,
            complete::space1,
            complete::u32,
            complete::space1,
            complete::u32
        ))
            .map(|(destination_start, _, source_start, _, length)| Self { destination_start, source_start, length })
            .parse(input)
    }

    fn contains(&self, source: u32) -> bool {
        self.source_start <= source && source - self.source_start < self.length
    }

    fn map(&self, source: u32) -> u32 {
        debug_assert!(self.contains(source));
        self.destination_start + (source - self.source_start)
    }
}

impl Slice {
    fn split_at(&self, length: u32) -> (Slice, Option<Slice>) {
        if length >= self.length {
            (*self, None)
        } else {
            (
                Slice { start: self.start, length },
                Some(Slice { start: self.start + length, length: self.length - length })
            )
        }
    }
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle::parse(data).unwrap().1
    }

    fn part_1(&self) -> String {
        self.seeds.iter()
            .map(|&seed| {
                self.maps.iter()
                    .fold(
                        seed,
                        |source, map| map.map(source),
                    )
            })
            .min()
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        let seed_slices = self.seeds.chunks_exact(2)
            .map(|seeds| Slice { start: seeds[0], length: seeds[1] })
            .collect();
        let location_slices = self.maps.iter()
            .fold(seed_slices, |seeds, map| map.map_slices(&seeds));
        location_slices.iter()
            .map(|location_slice| location_slice.start)
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_05.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map {
                    name: String::from("seed-to-soil"),
                    ranges: vec![
                        Range { destination_start: 52, source_start: 50, length: 48 },
                        Range { destination_start: 50, source_start: 98, length: 2 },
                    ],
                },
                Map {
                    name: String::from("soil-to-fertilizer"),
                    ranges: vec![
                        Range { destination_start: 39, source_start: 0, length: 15 },
                        Range { destination_start: 0, source_start: 15, length: 37 },
                        Range { destination_start: 37, source_start: 52, length: 2 },
                    ],
                },
                Map {
                    name: String::from("fertilizer-to-water"),
                    ranges: vec![
                        Range { destination_start: 42, source_start: 0, length: 7 },
                        Range { destination_start: 57, source_start: 7, length: 4 },
                        Range { destination_start: 0, source_start: 11, length: 42 },
                        Range { destination_start: 49, source_start: 53, length: 8 },
                    ],
                },
                Map {
                    name: String::from("water-to-light"),
                    ranges: vec![
                        Range { destination_start: 88, source_start: 18, length: 7 },
                        Range { destination_start: 18, source_start: 25, length: 70 },
                    ],
                },
                Map {
                    name: String::from("light-to-temperature"),
                    ranges: vec![
                        Range { destination_start: 81, source_start: 45, length: 19 },
                        Range { destination_start: 68, source_start: 64, length: 13 },
                        Range { destination_start: 45, source_start: 77, length: 23 },
                    ],
                },
                Map {
                    name: String::from("temperature-to-humidity"),
                    ranges: vec![
                        Range { destination_start: 1, source_start: 0, length: 69 },
                        Range { destination_start: 0, source_start: 69, length: 1 },
                    ],
                },
                Map {
                    name: String::from("humidity-to-location"),
                    ranges: vec![
                        Range { destination_start: 60, source_start: 56, length: 37 },
                        Range { destination_start: 56, source_start: 93, length: 4 },
                    ],
                },
            ],
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "35");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "46");
    }

    #[test]
    fn map_slices() {
        let map = Map {
            name: String::from("a-to-b"),
            ranges: vec![
                Range { destination_start: 113, source_start: 13, length: 17 },
                Range { destination_start: 254, source_start: 54, length: 7 },
            ],
        };

        assert_eq!(map.map_slices(&vec![Slice { start: 3, length: 5 }]),
                   vec![Slice { start: 3, length: 5 }]);
        assert_eq!(map.map_slices(&vec![Slice { start: 3, length: 15 }]),
                   vec![Slice { start: 3, length: 10 }, Slice { start: 113, length: 5 }]);
        assert_eq!(map.map_slices(&vec![Slice { start: 23, length: 5 }]),
                   vec![Slice { start: 123, length: 5 }]);
        assert_eq!(map.map_slices(&vec![Slice { start: 23, length: 15 }]),
                   vec![Slice { start: 123, length: 7 }, Slice { start: 30, length: 8 }]);
        assert_eq!(map.map_slices(&vec![Slice { start: 48, length: 61 }]),
                   vec![Slice { start: 48, length: 6 }, Slice { start: 254, length: 7 }, Slice { start: 61, length: 48 }]);

        assert_eq!(map.map_slices(&vec![
            Slice { start: 23, length: 15 },
            Slice { start: 3, length: 15 },
            Slice { start: 48, length: 61 },
        ]),
                   vec![
                       Slice { start: 3, length: 10 }, Slice { start: 113, length: 5 },
                       Slice { start: 123, length: 7 }, Slice { start: 30, length: 8 },
                       Slice { start: 48, length: 6 }, Slice { start: 254, length: 7 }, Slice { start: 61, length: 48 },
                   ]);
    }
}