use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom_supreme::ParserExt;

use crate::PuzzleBase;

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    hand_bids: Vec<(Hand, u32)>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
struct Hand {
    cards: [Card; 5],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Puzzle {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(
            complete::line_ending,
            separated_pair(Hand::parse, complete::space1, complete::u32),
        )
            .map(|hand_bids| Self { hand_bids })
            .parse(input)
    }
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        tuple((Card::parse, Card::parse, Card::parse, Card::parse, Card::parse))
            .map(|cards| Self { cards: cards.into() })
            .parse(input)
    }

    fn get_type(&self) -> HandType {
        let mut count: [u8; 14] = [0; 14];
        for card in self.cards {
            count[card as usize] += 1
        }
        let jokers = count[Card::Joker as usize];
        count[Card::Joker as usize] = 0;
        count.sort();
        count.reverse();
        match count[0] + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match count[1] {
                2 => HandType::FullHouse,
                1 => HandType::ThreeOfAKind,
                _ => panic!("Five cards expected!")
            },
            2 => match count[1] {
                2 => HandType::TwoPair,
                1 => HandType::OnePair,
                _ => panic!("Five cards expected!")
            },
            1 => HandType::HighCard,
            _ => panic!("Five cards expected!")
        }
    }
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("2").value(Card::Two),
            tag("3").value(Card::Three),
            tag("4").value(Card::Four),
            tag("5").value(Card::Five),
            tag("6").value(Card::Six),
            tag("7").value(Card::Seven),
            tag("8").value(Card::Eight),
            tag("9").value(Card::Nine),
            tag("T").value(Card::Ten),
            tag("J").value(Card::Jack),
            tag("Q").value(Card::Queen),
            tag("K").value(Card::King),
            tag("A").value(Card::Ace),
        ))
            .parse(input)
    }
}


impl PuzzleBase for Puzzle {
    fn new(data: &str) -> Self {
        Puzzle::parse(data).unwrap().1
    }

    fn part_1(&self) -> String {
        let mut hand_bids = self.hand_bids.to_vec();
        hand_bids.sort_by_key(|(hand, _bid)| (hand.get_type(), *hand));
        hand_bids.iter()
            .enumerate()
            .map(|(rank, (_hand, bid))| (rank as u32 + 1) * bid)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut hand_bids: Vec<(Hand, u32)> = self.hand_bids.iter()
            .map(|(hand, bid)| {
                let mut hand_ = hand.to_owned();
                for (i, card) in hand.cards.iter().enumerate() {
                    if *card == Card::Jack {
                        hand_.cards[i] = Card::Joker;
                    }
                }
                (hand_, *bid)
            })
            .collect();

        hand_bids.sort_by_key(|(hand, _bid)| (hand.get_type(), *hand));
        hand_bids.iter()
            .enumerate()
            .map(|(rank, (_hand, bid))| (rank as u32 + 1) * bid)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::PuzzleBase;

    use super::*;

    fn get_puzzle() -> Puzzle {
        let data = fs::read_to_string("data/examples/day_07.txt").unwrap();

        Puzzle::new(&data)
    }

    #[test]
    fn new() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle, Puzzle {
            hand_bids: vec![
                (Hand { cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King] }, 765),
                (Hand { cards: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five] }, 684),
                (Hand { cards: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven] }, 28),
                (Hand { cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten] }, 220),
                (Hand { cards: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace] }, 483),
            ]
        })
    }

    #[test]
    fn part_1() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_1(), "6440");
    }

    #[test]
    fn part_2() {
        let puzzle = get_puzzle();

        assert_eq!(puzzle.part_2(), "5905");
    }
}