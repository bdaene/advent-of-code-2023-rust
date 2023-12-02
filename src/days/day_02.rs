use std::ops;

use nom::{branch, bytes::complete::tag, character::complete, IResult, multi, sequence};

use crate::PuzzleBase;

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(PartialEq, Debug)]
pub struct Game {
    id: u32,
    colors: Vec<Color>,
}


#[derive(PartialEq, Debug)]
pub struct Puzzle {
    games: Vec<Game>,
}


impl ops::Add for Color {
    type Output = Color;

    fn add(self, color: Self) -> Self::Output {
        Color {
            red: self.red + color.red,
            green: self.green + color.green,
            blue: self.blue + color.blue,
        }
    }
}

impl Color {
    fn from_color_str(input: &str) -> IResult<&str, Color> {
        let (input, (n, color)) = branch::alt((
            sequence::separated_pair(complete::u32, complete::multispace1, tag("red")),
            sequence::separated_pair(complete::u32, complete::multispace1, tag("green")),
            sequence::separated_pair(complete::u32, complete::multispace1, tag("blue")),
        ))(input)?;
        Ok((input,
            match color {
                "red" => Color { red: n, ..Color::default() },
                "green" => Color { green: n, ..Color::default() },
                "blue" => Color { blue: n, ..Color::default() },
                _ => panic!("Invalid color {:?}.", color)
            }
        ))
    }

    fn from_str(input: &str) -> IResult<&str, Color> {
        let (input, colors) = multi::separated_list1(
            tag(", "),
            Color::from_color_str,
        )(input)?;
        Ok((input,
            colors.into_iter().fold(Color::default(), |a, b| a + b)
        ))
    }

    fn max(colors: &[Color]) -> Option<Color> {
        let mut iterator = colors.iter();
        let max_color = iterator.next()?;
        Some(iterator.fold(
            *max_color,
            |a, b| Color {
                red: a.red.max(b.red),
                green: a.green.max(b.green),
                blue: a.blue.max(b.blue),
            },
        ))
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    fn from_str(input: &str) -> IResult<&str, Game> {
        let (input, (_, id, _, colors)) = sequence::tuple((
            tag("Game "),
            complete::u32,
            tag(": "),
            multi::separated_list1(
                tag("; "),
                Color::from_str,
            )
        ))(input)?;
        Ok((input,
            Game { id, colors }
        ))
    }
}

impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        let games = data.lines()
            .map(|line| {
                let (_, game) = Game::from_str(line).unwrap();
                game
            })
            .collect();
        Puzzle { games }
    }

    fn part_1(&self) -> String {
        self.games.iter()
            .filter(|&game| {
                game.colors.iter()
                    .all(|colors| colors.red <= 12 && colors.green <= 13 && colors.blue <= 14)
            })
            .map(|game| game.id)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.games.iter()
            .map(|game| Color::max(&game.colors[..]).unwrap())
            .map(|color| color.power())
            .sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_solution() -> Puzzle {
        let data = fs::read_to_string("data/day_02_example.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(solution, Puzzle {
            games: vec![
                Game {
                    id: 1,
                    colors: vec![
                        Color { red: 4, blue: 3, green: 0 },
                        Color { red: 1, blue: 6, green: 2 },
                        Color { red: 0, blue: 0, green: 2 },
                    ],
                },
                Game {
                    id: 2,
                    colors: vec![
                        Color { red: 0, blue: 1, green: 2 },
                        Color { red: 1, blue: 4, green: 3 },
                        Color { red: 0, blue: 1, green: 1 },
                    ],
                },
                Game {
                    id: 3,
                    colors: vec![
                        Color { red: 20, blue: 6, green: 8 },
                        Color { red: 4, blue: 5, green: 13 },
                        Color { red: 1, blue: 0, green: 5 },
                    ],
                },
                Game {
                    id: 4,
                    colors: vec![
                        Color { red: 3, blue: 6, green: 1 },
                        Color { red: 6, blue: 0, green: 3 },
                        Color { red: 14, blue: 15, green: 3 },
                    ],
                },
                Game {
                    id: 5,
                    colors: vec![
                        Color { red: 6, blue: 1, green: 3 },
                        Color { red: 1, blue: 2, green: 2 },
                    ],
                },
            ]
        })
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "8");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "2286");
    }
}