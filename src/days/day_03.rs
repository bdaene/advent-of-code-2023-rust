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
        let values: Vec<u32> = self.get_numbers().into_iter()
            .filter(|number| self.is_adjacent_symbol(number))
            .map(|number| number.value)
            .collect();
        values.iter()
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let numbers = self.get_numbers();
        self.get_symbols().iter()
            .filter_map(|symbol| {
                if symbol.char != '*' { return None}
                let adjacent_numbers: Vec<&Number> = numbers.iter()
                    .filter(|&number| number.is_adjacent_symbol(symbol))
                    .collect();
                if adjacent_numbers.len() != 2 {
                    return None
                }
                Some(adjacent_numbers.iter().map(|number| number.value).product::<u32>())
            })
            .sum::<u32>()
            .to_string()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Number {
    value: u32,
    row: usize,
    col: usize,
    length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Symbol {
    char: char,
    row: usize,
    col: usize,
}


impl Puzzle {
    fn get_numbers(&self) -> Vec<Number> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                let mut numbers = Vec::new();
                let mut value = 0;
                let mut length: usize = 0;
                for (col, char) in line.iter().enumerate() {
                    if let Some(digit) = char.to_digit(10) {
                        value = value * 10 + digit;
                        length += 1;
                    } else {
                        if length > 0 {
                            numbers.push(Number { value, row, col: col - length, length });
                            value = 0;
                            length = 0;
                        }
                    }
                }
                if length > 0 {
                    numbers.push(Number { value, row, col: line.len() - length, length });
                }
                numbers.into_iter()
            })
            .collect()
    }

    fn get_symbols(&self) -> Vec<Symbol> {
        self.lines
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(col, &char)| Symbol { char, row, col })
            })
            .filter(|symbol| symbol.char != '.' && ! symbol.char.is_digit(10))
            .collect()
    }

    fn is_adjacent_symbol(&self, number: &Number) -> bool {
        (number.row.saturating_sub(1)..=(number.row + 1))
            .flat_map(|row| (number.col.saturating_sub(1)..=(number.col + number.length)).map(move |col| (row, col)))
            .filter(|&(row, col)| row != number.row || col < number.col || (col >= number.col + number.length))
            .filter(|&(row, col)| row < self.lines.len() && col < self.lines[row].len())
            .any(|(row, col)| self.lines[row][col] != '.')
    }
}

impl Number {
    fn is_adjacent(&self, row: usize, col: usize) -> bool {
        self.row.abs_diff(row) <= 1 && col + 1 >= self.col && col <= self.col + self.length
    }

    fn is_adjacent_symbol(&self, symbol: &Symbol) -> bool {
        self.is_adjacent(symbol.row, symbol.col)
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
    fn get_numbers() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_numbers(), vec![
            Number { value: 467, row: 0, col: 0, length: 3 },
            Number { value: 114, row: 0, col: 5, length: 3 },
            Number { value: 35, row: 2, col: 2, length: 2 },
            Number { value: 633, row: 2, col: 6, length: 3 },
            Number { value: 617, row: 4, col: 0, length: 3 },
            Number { value: 58, row: 5, col: 7, length: 2 },
            Number { value: 592, row: 6, col: 2, length: 3 },
            Number { value: 755, row: 7, col: 6, length: 3 },
            Number { value: 664, row: 9, col: 1, length: 3 },
            Number { value: 598, row: 9, col: 5, length: 3 },
        ])
    }

    #[test]
    fn is_adjacent_symbol() {
        let puzzle = Puzzle::new("....\n.12.\n....\n");
        assert!(!puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));

        let puzzle = Puzzle::new("*...\n.12.\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new(".@..\n.12.\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new(".../\n.12.\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n#12.\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n#12.\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n.12%\n....\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n.12.\n=...\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n.12.\n.5..\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n.12.\n..$.\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("....\n.12.\n...u\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));

        let puzzle = Puzzle::new("45..\n*...\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
        let puzzle = Puzzle::new("..45\n.*..\n");
        assert!(puzzle.is_adjacent_symbol(&puzzle.get_numbers()[0]));
    }

    #[test]
    fn get_symbols(){
        let puzzle = get_puzzle();

        assert_eq!(puzzle.get_symbols(), vec![
            Symbol { char: '*', row: 1, col: 3 },
            Symbol { char: '#', row: 3, col: 6 },
            Symbol { char: '*', row: 4, col: 3 },
            Symbol { char: '+', row: 5, col: 5 },
            Symbol { char: '$', row: 8, col: 3 },
            Symbol { char: '*', row: 8, col: 5 }
        ])
    }

    #[test]
    fn is_adjacent(){
        let puzzle = get_puzzle();
        let numbers = puzzle.get_numbers();
        let symbols =  puzzle.get_symbols();

        assert!(numbers[0].is_adjacent_symbol(&symbols[0]));
        assert!(numbers[2].is_adjacent_symbol(&symbols[0]));
        assert!(numbers[4].is_adjacent_symbol(&symbols[2]));
        assert!(numbers[7].is_adjacent_symbol(&symbols[5]));
        assert!(numbers[9].is_adjacent_symbol(&symbols[5]));

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