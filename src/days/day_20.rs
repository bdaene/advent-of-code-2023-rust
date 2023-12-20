use std::collections::{HashMap, VecDeque};

use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::one_of;
use nom::character::streaming::alpha1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    modules: Vec<Module>,
}

#[derive(Debug, PartialEq, Eq)]
struct Module {
    name: String,
    destinations: Vec<String>,
    module_type: ModuleType,
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Debug, PartialEq, Eq)]
enum State<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster,
}


impl PuzzleBase for Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            Module::parse,
        )
            .map(|modules| Self { modules })
            .parse(input)
    }

    fn part_1(&self) -> String {
        let (modules, mut states) = self.init();

        let (mut highs, mut lows) = (0usize, 0usize);
        for _ in 0..1000 {
            let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::from([("button", "broadcaster", false)]);

            while let Some((source, destination, high)) = pulses.pop_front() {
                // println!("{source} -{high}-> {destination}");
                if high { highs += 1 } else { lows += 1 };
                if let Some(pulse) = states.get_mut(destination).and_then(|state| state.receive(source, high)) {
                    pulses.extend(modules[destination].destinations.iter().map(|dest| (destination, dest.as_str(), pulse)))
                }
            }
        }

        (highs * lows).to_string()
    }

    fn part_2(&self) -> String {
        let zh = self.modules.iter().filter(|module| module.destinations.contains(&"rx".to_string())).next().unwrap();
        let mut cycles:HashMap<&str, Option<usize>> = HashMap::from_iter(self.modules.iter().filter(|module| module.destinations.contains(&zh.name)).map(|module| (module.name.as_str(), None)));

        let (modules, mut states) = self.init();
        let mut button_push: usize = 0;
        loop {
            button_push += 1;
            let mut pulses: VecDeque<(&str, &str, bool)> = VecDeque::from([("button", "broadcaster", false)]);
            while let Some((source, destination, high)) = pulses.pop_front() {
                if high && cycles.contains_key(source) {
                    cycles.insert(source, Some(button_push));
                    if cycles.values().all(|cycle| cycle.is_some()) {
                        return cycles.values().map(|cycle| cycle.unwrap()).product::<usize>().to_string();
                    }
                }
                // if !high && destination == "rx" {
                //     return button_push.to_string();
                // }
                // if high && ["xc", "th", "pd", "bp"].contains(&source) {
                //     println!("{button_push}: {source} -{high}-> {destination} ({:?})", states["zh"]);
                // }
                if let Some(pulse) = states.get_mut(destination).and_then(|state| state.receive(source, high)) {
                    pulses.extend(modules[destination].destinations.iter().map(|dest| (destination, dest.as_str(), pulse)))
                }
            }
        }
    }
}

impl Puzzle {
    fn init(&self) -> (HashMap<&str, &Module>, HashMap<&str, State>) {
        let mut states: HashMap<&str, State> = HashMap::from_iter(self.modules.iter()
            .map(|module| (
                module.name.as_str(),
                match module.module_type {
                    ModuleType::FlipFlop => State::FlipFlop(false),
                    ModuleType::Conjunction => State::Conjunction(HashMap::new()),
                    ModuleType::Broadcaster => State::Broadcaster,
                }
            ))
        );

        for module in self.modules.iter() {
            for destination in module.destinations.iter() {
                if let Some(State::Conjunction(inputs)) = states.get_mut(destination.as_str()) {
                    inputs.insert(&module.name, false);
                }
            }
        }

        let modules: HashMap<&str, &Module> = HashMap::from_iter(self.modules.iter().map(|module| (module.name.as_str(), module)));

        (modules, states)
    }
}

impl Module {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(
            pair(
                opt(one_of("%&")),
                alpha1,
            ),
            tag(" -> "),
            separated_list1(
                tag(", "),
                alpha1,
            ),
        )
            .map(|((module_type, name), destinations)| Self {
                name: String::from(name),
                destinations: destinations.into_iter().map(|destination| String::from(destination)).collect(),
                module_type: match module_type {
                    Some('%') => ModuleType::FlipFlop,
                    Some('&') => ModuleType::Conjunction,
                    None => ModuleType::Broadcaster,
                    _ => panic!("Unrecognized module type")
                },
            })
            .parse(input)
    }
}

impl State<'_> {
    fn receive(&mut self, origin: &str, high: bool) -> Option<bool> {
        match self {
            State::FlipFlop(on) => if high { None } else {
                *on = !(*on);
                Some(*on)
            },
            State::Conjunction(inputs) => {
                *(inputs.get_mut(origin).unwrap()) = high;
                if inputs.values().all(|&input| input) { Some(false) } else { Some(true) }
            }
            State::Broadcaster => Some(high),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle(i: usize) -> Puzzle {
        let data = fs::read_to_string(format!("data/examples/day_20_{i}.txt")).unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        assert_eq!(get_puzzle(1), Puzzle {
            modules: vec![
                Module { name: "broadcaster".to_string(), destinations: vec!["a".to_string(), "b".to_string(), "c".to_string()], module_type: ModuleType::Broadcaster },
                Module { name: "a".to_string(), destinations: vec!["b".to_string()], module_type: ModuleType::FlipFlop },
                Module { name: "b".to_string(), destinations: vec!["c".to_string()], module_type: ModuleType::FlipFlop },
                Module { name: "c".to_string(), destinations: vec!["inv".to_string()], module_type: ModuleType::FlipFlop },
                Module { name: "inv".to_string(), destinations: vec!["a".to_string()], module_type: ModuleType::Conjunction },
            ]
        });
    }

    #[test]
    fn part_1() {
        // assert_eq!(get_puzzle(1).part_1(), "32000000");
        assert_eq!(get_puzzle(2).part_1(), "11687500");
    }
}