pub mod days;

pub trait PuzzleBase {
    fn new(data: &str) -> Self
        where
            Self: Sized;

    fn part_1(&self) -> String {
        String::from("Not implemented yet.")
    }

    fn part_2(&self) -> String {
        String::from("Not implemented yet.")
    }
}

pub fn get_puzzle(day: u8, data: &str) -> Box<dyn PuzzleBase> {
    match day {
        01 => Box::new(days::day_01::Puzzle::new(data)),
        02 => Box::new(days::day_02::Puzzle::new(data)),
        03 => Box::new(days::day_03::Puzzle::new(data)),
        04 => Box::new(days::day_04::Puzzle::new(data)),
        05 => Box::new(days::day_05::Puzzle::new(data)),

        _ => panic!("Invalid day"),
    }
}