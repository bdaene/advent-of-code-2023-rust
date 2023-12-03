use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    lines: Vec<Vec<char>>,
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        let lines = data
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Puzzle { lines }
    }

    fn part_1(&self) -> String {
        self.get_symbols().iter()
            .flat_map(|symbol| self.get_values_adjacent_to(symbol))
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.get_symbols().iter()
            .filter(|&symbol| symbol.char == '*')
            .filter_map(|symbol| {
                let values = self.get_values_adjacent_to(symbol);
                if values.len() == 2 {
                    Some(values[0] * values[1])
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string()
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    char: char,
    row: usize,
    col: usize,
}

fn get_value(chars: &[char]) -> u32 {
    chars.iter().fold(0, |acc, char| acc * 10 + char.to_digit(10).unwrap())
}

impl Puzzle {
    fn get_symbols(&self) -> Vec<Symbol> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col, &char)| Symbol { char, row, col })
            })
            .filter(|symbol| symbol.char != '.' && !symbol.char.is_digit(10))
            .collect()
    }

    fn get_values_adjacent_to(&self, symbol: &Symbol) -> Vec<u32> {
        let mut values = Vec::new();
        for row in symbol.row.saturating_sub(1)..self.lines.len().min(symbol.row + 2) {
            let line = &self.lines[row];

            let mut left: usize = symbol.col;
            while left > 0 && line[left - 1].is_digit(10) { left -= 1 };

            let mut right: usize = symbol.col;
            while right < line.len() - 1 && line[right + 1].is_digit(10) { right += 1 };

            if line[symbol.col].is_digit(10) {
                values.push(get_value(&line[left..=right]));
            } else {
                if left < symbol.col {
                    values.push(get_value(&line[left..symbol.col]));
                }
                if right > symbol.col {
                    values.push(get_value(&line[(symbol.col + 1)..=right]));
                }
            }
        }
        values
    }
}


#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/day_03_example.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            lines: vec![
                vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
                vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
                vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
                vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
                vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
                vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
                vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
            ]
        })
    }


    #[test]
    fn get_symbols() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_symbols(), vec![
            Symbol { char: '*', row: 1, col: 3 },
            Symbol { char: '#', row: 3, col: 6 },
            Symbol { char: '*', row: 4, col: 3 },
            Symbol { char: '+', row: 5, col: 5 },
            Symbol { char: '$', row: 8, col: 3 },
            Symbol { char: '*', row: 8, col: 5 },
        ])
    }

    #[test]
    fn get_values_adjacent_to() {
        let puzzle = get_puzzle();
        let symbols = puzzle.get_symbols();

        assert_eq!(puzzle.get_values_adjacent_to(&symbols[0]), vec![467, 35]);
        assert_eq!(puzzle.get_values_adjacent_to(&symbols[3]), vec![592]);
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "4361");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "467835");
    }
}