use std::collections::HashMap;
use std::ops::Range;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::combinator::{opt, value};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    condition: Option<Condition>,
    destination: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Condition {
    category: Category,
    is_lower_limit: bool,
    limit: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Category {
    ExtremelyCoolLooking = 0,
    Musical = 1,
    Aerodynamic = 2,
    Shiny = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    extremely_cool_looking: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PartRange {
    extremely_cool_looking: Range<u32>,
    musical: Range<u32>,
    aerodynamic: Range<u32>,
    shiny: Range<u32>,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            separated_list1(
                complete::line_ending,
                Workflow::parse,
            ),
            pair(complete::line_ending, complete::line_ending),
            separated_list1(
                complete::line_ending,
                Part::parse,
            ),
        )
            .map(|(workflows, parts)| Self { workflows, parts })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let workflows: HashMap<&str, &Workflow> = HashMap::from_iter(self.workflows.iter()
            .map(|workflow| (workflow.name.as_str(), workflow)));

        self.parts.iter()
            .filter(|part| part.is_accepted(&workflows, "in"))
            .map(|part| part.get_rating())
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let workflows: HashMap<&str, &Workflow> = HashMap::from_iter(self.workflows.iter()
            .map(|workflow| (workflow.name.as_str(), workflow)));

        let start_range = PartRange {
            extremely_cool_looking: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        };
        let mut current_ranges = vec![("in", start_range)];
        let mut total = 0;
        while let Some((destination, part_range)) = current_ranges.pop() {
            match destination {
                "R" => (),
                "A" => total += part_range.count(),
                _ => current_ranges.extend(workflows[destination].send_range(&part_range).into_iter())
            }
        }

        total.to_string()
    }
}

impl Workflow {
    fn parse(input: &str) -> IResult<&str, Self> {
        pair(
            alpha1,
            delimited(
                tag("{"),
                separated_list1(
                    tag(","),
                    Rule::parse,
                ),
                tag("}"),
            ),
        )
            .map(|(name, rules)| Self { name: String::from(name), rules })
            .parse(input)
    }

    fn send(&self, part: &Part) -> &str {
        self.rules.iter()
            .filter(|rule| rule.matches(part))
            .next()
            .unwrap()
            .destination.as_str()
    }

    fn send_range(&self, start_range: &PartRange) -> Vec<(&str, PartRange)> {
        let mut current_range = start_range.clone();

        let mut send_ranges = Vec::new();
        for rule in &self.rules {
            if current_range.is_empty() {
                break;
            }
            let (matched_range, excluded_range) = rule.split(&current_range);
            send_ranges.push((rule.destination.as_str(), matched_range));
            current_range = excluded_range;
        }
        send_ranges
    }
}

impl Rule {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((
            opt(terminated(
                Condition::parse,
                tag(":"),
            )),
            alpha1
        ))
            .map(|(condition, destination)| Self { condition, destination: String::from(destination) })
            .parse(input)
    }

    fn matches(&self, part: &Part) -> bool {
        match &self.condition {
            None => true,
            Some(condition) => condition.matches(part),
        }
    }

    fn split(&self, part_range: &PartRange) -> (PartRange, PartRange) {
        match &self.condition {
            None => (part_range.clone(), PartRange::empty()),
            Some(condition) => condition.split(part_range)
        }
    }
}

impl Condition {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((
            Category::parse,
            alt((
                value(true, tag(">")),
                value(false, tag("<")),
            )),
            complete::u32
        ))
            .map(|(category, is_lower_limit, limit)| Self { category, is_lower_limit, limit })
            .parse(input)
    }

    fn matches(&self, part: &Part) -> bool {
        if self.is_lower_limit {
            part.get_category_rating(self.category) > self.limit
        } else {
            part.get_category_rating(self.category) < self.limit
        }
    }

    fn split(&self, part_range: &PartRange) -> (PartRange, PartRange) {
        match self.category {
            Category::ExtremelyCoolLooking => {
                let (matched_range, excluded_range) = split(&part_range.extremely_cool_looking, self.is_lower_limit, self.limit);
                (
                    PartRange {
                        extremely_cool_looking: matched_range,
                        musical: part_range.musical.clone(),
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: part_range.shiny.clone(),
                    },
                    PartRange {
                        extremely_cool_looking: excluded_range,
                        musical: part_range.musical.clone(),
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: part_range.shiny.clone(),
                    }
                )
            }
            Category::Musical => {
                let (matched_range, excluded_range) = split(&part_range.musical, self.is_lower_limit, self.limit);
                (
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: matched_range,
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: part_range.shiny.clone(),
                    },
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: excluded_range,
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: part_range.shiny.clone(),
                    }
                )
            }
            Category::Aerodynamic => {
                let (matched_range, excluded_range) = split(&part_range.aerodynamic, self.is_lower_limit, self.limit);
                (
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: part_range.musical.clone(),
                        aerodynamic: matched_range,
                        shiny: part_range.shiny.clone(),
                    },
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: part_range.musical.clone(),
                        aerodynamic: excluded_range,
                        shiny: part_range.shiny.clone(),
                    }
                )
            }
            Category::Shiny => {
                let (matched_range, excluded_range) = split(&part_range.shiny, self.is_lower_limit, self.limit);
                (
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: part_range.musical.clone(),
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: matched_range,
                    },
                    PartRange {
                        extremely_cool_looking: part_range.extremely_cool_looking.clone(),
                        musical: part_range.musical.clone(),
                        aerodynamic: part_range.aerodynamic.clone(),
                        shiny: excluded_range,
                    }
                )
            }
        }
    }
}

impl Category {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Category::ExtremelyCoolLooking, tag("x")),
            value(Category::Musical, tag("m")),
            value(Category::Aerodynamic, tag("a")),
            value(Category::Shiny, tag("s")),
        ))
            .parse(input)
    }
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Self> {
        delimited(
            tag("{"),
            tuple((
                preceded(tag("x="), complete::u32),
                preceded(tag(",m="), complete::u32),
                preceded(tag(",a="), complete::u32),
                preceded(tag(",s="), complete::u32),
            )),
            tag("}"),
        )
            .map(|(extremely_cool_looking, musical, aerodynamic, shiny)| Self { extremely_cool_looking, musical, aerodynamic, shiny })
            .parse(input)
    }

    fn is_accepted(&self, workflows: &HashMap<&str, &Workflow>, start: &str) -> bool {
        let mut current_workflow = start;

        while !["A", "R"].contains(&current_workflow) {
            current_workflow = workflows[current_workflow].send(self)
        }

        current_workflow == "A"
    }

    fn get_rating(&self) -> u32 {
        self.extremely_cool_looking + self.musical + self.aerodynamic + self.shiny
    }

    fn get_category_rating(&self, category: Category) -> u32 {
        match category {
            Category::ExtremelyCoolLooking => self.extremely_cool_looking,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }
}

impl PartRange {
    fn count(&self) -> u64 {
        1
            * (self.extremely_cool_looking.end - self.extremely_cool_looking.start) as u64
            * (self.musical.end - self.musical.start) as u64
            * (self.aerodynamic.end - self.aerodynamic.start) as u64
            * (self.shiny.end - self.shiny.start) as u64
    }

    fn empty() -> PartRange {
        PartRange {
            extremely_cool_looking: 0..0,
            musical: 0..0,
            aerodynamic: 0..0,
            shiny: 0..0,
        }
    }

    fn is_empty(&self) -> bool {
        false
            || self.extremely_cool_looking.is_empty()
            || self.musical.is_empty()
            || self.aerodynamic.is_empty()
            || self.shiny.is_empty()
    }
}

fn split(range: &Range<u32>, is_lower_limit: bool, limit: u32) -> (Range<u32>, Range<u32>) {
    if is_lower_limit {
        (range.start.max(limit + 1)..range.end, range.start..range.end.min(limit + 1))
    } else {
        (range.start..range.end.min(limit), range.start.max(limit)..range.end, )
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_19.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            workflows: vec![
                Workflow {
                    name: "px".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Aerodynamic, is_lower_limit: false, limit: 2006 }), destination: "qkq".to_string() },
                        Rule { condition: Some(Condition { category: Category::Musical, is_lower_limit: true, limit: 2090 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "rfg".to_string() },
                    ],
                },
                Workflow {
                    name: "pv".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Aerodynamic, is_lower_limit: true, limit: 1716 }), destination: "R".to_string() },
                        Rule { condition: None, destination: "A".to_string() },
                    ],
                },
                Workflow {
                    name: "lnx".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Musical, is_lower_limit: true, limit: 1548 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "A".to_string() },
                    ],
                },
                Workflow {
                    name: "rfg".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Shiny, is_lower_limit: false, limit: 537 }), destination: "gd".to_string() },
                        Rule { condition: Some(Condition { category: Category::ExtremelyCoolLooking, is_lower_limit: true, limit: 2440 }), destination: "R".to_string() },
                        Rule { condition: None, destination: "A".to_string() }],
                },
                Workflow {
                    name: "qs".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Shiny, is_lower_limit: true, limit: 3448 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "lnx".to_string() },
                    ],
                },
                Workflow {
                    name: "qkq".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::ExtremelyCoolLooking, is_lower_limit: false, limit: 1416 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "crn".to_string() },
                    ],
                },
                Workflow {
                    name: "crn".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::ExtremelyCoolLooking, is_lower_limit: true, limit: 2662 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "R".to_string() },
                    ],
                },
                Workflow {
                    name: "in".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Shiny, is_lower_limit: false, limit: 1351 }), destination: "px".to_string() },
                        Rule { condition: None, destination: "qqz".to_string() },
                    ],
                },
                Workflow {
                    name: "qqz".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Shiny, is_lower_limit: true, limit: 2770 }), destination: "qs".to_string() },
                        Rule { condition: Some(Condition { category: Category::Musical, is_lower_limit: false, limit: 1801 }), destination: "hdj".to_string() },
                        Rule { condition: None, destination: "R".to_string() },
                    ],
                },
                Workflow {
                    name: "gd".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Aerodynamic, is_lower_limit: true, limit: 3333 }), destination: "R".to_string() },
                        Rule { condition: None, destination: "R".to_string() }],
                },
                Workflow {
                    name: "hdj".to_string(),
                    rules: vec![
                        Rule { condition: Some(Condition { category: Category::Musical, is_lower_limit: true, limit: 838 }), destination: "A".to_string() },
                        Rule { condition: None, destination: "pv".to_string() },
                    ],
                },
            ],
            parts: vec![
                Part { extremely_cool_looking: 787, musical: 2655, aerodynamic: 1222, shiny: 2876 },
                Part { extremely_cool_looking: 1679, musical: 44, aerodynamic: 2067, shiny: 496 },
                Part { extremely_cool_looking: 2036, musical: 264, aerodynamic: 79, shiny: 2244 },
                Part { extremely_cool_looking: 2461, musical: 1339, aerodynamic: 466, shiny: 291 },
                Part { extremely_cool_looking: 2127, musical: 1623, aerodynamic: 2188, shiny: 1013 },
            ],
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "19114");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "167409079868000");
    }
}