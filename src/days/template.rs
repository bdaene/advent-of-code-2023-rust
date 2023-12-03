use crate::PuzzleBase;

#[derive(PartialEq, Debug)]
pub struct Puzzle {
    data: String,
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle { data: String::from(data) }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/example.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle { data: String::from("Hello World!") })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "Not implemented yet.");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "Not implemented yet.");
    }
}