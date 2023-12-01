use crate::SolutionBase;


#[derive(PartialEq, Debug)]
pub struct Solution {
    document: Vec<String>,
}

const DIGITS_STRING: [&str; 18] = [
    "1", "one",
    "2", "two",
    "3", "three",
    "4", "four",
    "5", "five",
    "6", "six",
    "7", "seven",
    "8", "eight",
    "9", "nine",
];
const DIGITS: [u32; 18] = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9];

fn get_first_digit(line: &str) -> u32 {
    let digit = DIGITS_STRING.iter()
        .enumerate()
        .map(|(i, &digit)| (line.find(digit).unwrap_or(line.len()), DIGITS[i]))
        .min();
    digit.unwrap_or((0, 0)).1
}

fn get_last_digit(line: &str) -> u32 {
    let digit = DIGITS_STRING.iter()
        .enumerate()
        .map(|(i, &digit)| (line.rfind(digit), DIGITS[i]))
        .filter(|(position, _)| position.is_some())
        .map(|(position, digit)| (position.unwrap(), digit))
        .max();
    digit.unwrap_or((0, 0)).1
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let lines = data
            .lines()
            .map(|s| String::from(s))
            .collect();

        Solution { document: lines }
    }

    fn part_1(&self) -> String {
        self.document.iter()
            .map(|s| {
                let digits: Vec<u32> = s.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect();
                digits.first().unwrap() * 10 + digits.last().unwrap()
            }).sum::<u32>().to_string()
    }

    fn part_2(&self) -> String {
        let numbers: Vec<u32> = self.document.iter()
            .map(|line| 10 * get_first_digit(line) + get_last_digit(line)).collect();

        numbers.iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data: String = fs::read_to_string("data/day_01_example.txt").unwrap();

        Solution::new(&data)
    }

    fn get_solution_2() -> Solution {
        let data: String = fs::read_to_string("data/day_01_example_2.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                document: vec![
                    String::from("1abc2"),
                    String::from("pqr3stu8vwx"),
                    String::from("a1b2c3d4e5f"),
                    String::from("treb7uchet"),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "142");
    }

    #[test]
    fn part_2() {
        let solution = get_solution_2();

        assert_eq!(solution.part_2(), "358");
    }

    #[test]
    fn test_get_last_digit() {
        let s = "three";

        assert_eq!(s.rfind("three"), Some(0));
        assert_eq!(get_last_digit(s), 3);
    }
}