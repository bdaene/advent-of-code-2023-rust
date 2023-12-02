use crate::PuzzleBase;

#[derive(PartialEq, Debug)]
pub struct Puzzle {}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle {}
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_solution() -> Puzzle {
        let data = fs::read_to_string("data/example.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(solution, Puzzle {})
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "");
    }
}