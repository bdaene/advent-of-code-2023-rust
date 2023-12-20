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
        let modules: HashMap<&str, &Module> = HashMap::from_iter(self.modules.iter().map(|module| (module.name.as_str(), module)));
        let mut states = self.init_states();

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
        let modules: HashMap<&str, &Module> = HashMap::from_iter(self.modules.iter().map(|module| (module.name.as_str(), module)));

        modules["broadcaster"].destinations.iter()
            .map(|counter| {
                let mut bit: usize = 1;
                let mut limit: usize = 0;
                let mut flip_flop = modules.get(counter.as_str()).unwrap();
                loop {
                    let mut next_flip_flop = None;
                    flip_flop.destinations.iter()
                        .filter_map(|destination| modules.get(destination.as_str()))
                        .for_each(|module| match module.module_type {
                            ModuleType::Conjunction => { limit |= bit; }
                            ModuleType::FlipFlop => next_flip_flop = Some(module),
                            _ => (),
                        });
                    if next_flip_flop.is_some() {
                        bit <<= 1;
                        flip_flop = next_flip_flop.unwrap();
                    } else {
                        break;
                    }
                }
                limit
            })
            .product::<usize>()
            .to_string()
    }
}

impl Puzzle {
    fn init_states(&self) -> HashMap<&str, State> {
        HashMap::from_iter(self.modules.iter()
            .map(|module| (
                module.name.as_str(),
                match module.module_type {
                    ModuleType::FlipFlop => State::FlipFlop(false),
                    ModuleType::Conjunction => State::Conjunction(HashMap::from_iter(self.get_inputs(&module.name).iter()
                        .map(|module_| (module_.name.as_str(), false)))),
                    ModuleType::Broadcaster => State::Broadcaster,
                }
            ))
        )
    }

    fn get_inputs(&self, module: &String) -> Vec<&Module> {
        self.modules.iter().filter(|module_| module_.destinations.contains(module)).collect()
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
        assert_eq!(get_puzzle(1).part_1(), "32000000");
        assert_eq!(get_puzzle(2).part_1(), "11687500");
    }
}