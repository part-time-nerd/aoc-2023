use anyhow::{anyhow, Error, Result};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '0' => panic!("Card can't be 0"),
            '1' => panic!("Card can't be 1"),
            c if c.is_ascii_digit() => Card(c.to_digit(10).unwrap() as u8),
            'T' => Card(10),
            'J' => Card(11),
            'Q' => Card(12),
            'K' => Card(13),
            'A' => Card(14),
            _ => panic!("Card can't be {value}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(other.cards) {
                    match c1.cmp(&c2) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Into<u8> for &HandType {
    fn into(self) -> u8 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let _self: u8 = self.into();
        _self.cmp(&other.into())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts: HashMap<Card, u8> = HashMap::default();

        for card in self.cards.iter() {
            if let Some(count) = counts.get_mut(card) {
                *count += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => match counts.values().into_iter().collect::<Vec<&u8>>()[..] {
                [3, 2] | [2, 3] => HandType::FullHouse,
                [4, 1] | [1, 4] => HandType::FourOfAKind,
                _ => panic!("Unexpected counts of cards"),
            },
            3 => match counts.values().into_iter().collect::<Vec<&u8>>()[..] {
                [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => HandType::TwoPair,
                [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::ThreeOfAKind,
                _ => panic!("Unexpected counts of cards"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unexpected number of entries"),
        }
    }
}

impl FromStr for Hand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((cards, bid)) = s.split_once(' ') {
            Ok(Self {
                cards: cards.chars().map(|c| c.into()).collect::<Vec<Card>>()[..].try_into()?,
                bid: bid.parse()?,
            })
        } else {
            Err(anyhow!("Could not split the hand into a (cards, bid) tuple: {s}"))
        }
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let mut hands: Vec<Hand> = Vec::new();
    for hand in input.lines() {
        hands.push(hand.parse()?);
    }
    hands.sort();
    Ok(hands.into_iter().enumerate().map(|(i, hand)| (i + 1) * hand.bid).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_hand_from_string() {
        let hand: Hand = "32T3K 765".parse().unwrap();
        assert_eq!(hand, Hand{cards: [Card(3), Card(2), Card(10), Card(3), Card(13)], bid: 765 });
    }

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::ThreeOfAKind < HandType::FullHouse);
        assert!(HandType::OnePair == HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_hand_type() {
        let h1 = Hand { cards: [Card(3), Card(4), Card(5), Card(6), Card(9)], bid: 0 };
        let h2 = Hand { cards: [Card(2), Card(2), Card(3), Card(4), Card(5)], bid: 0 };
        let h3 = Hand { cards: [Card(3), Card(3), Card(2), Card(3), Card(2)], bid: 0 };
        assert_eq!(h1.hand_type(), HandType::HighCard);
        assert_eq!(h2.hand_type(), HandType::OnePair);
        assert_eq!(h3.hand_type(), HandType::FullHouse);
    }

    #[test]
    fn test_hand_ordering() {
        let h1 = Hand { cards: [Card(3), Card(4), Card(5), Card(6), Card(9)], bid: 0 };
        let h2 = Hand { cards: [Card(3), Card(4), Card(5), Card(8), Card(6)], bid: 0 };
        let h3 = Hand { cards: [Card(2), Card(2), Card(3), Card(4), Card(5)], bid: 0 };
        assert!(h1 == h1);
        assert!(h1 < h2);
        assert!(h3 > h1);
        assert!(h3 > h2);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 6440);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 256448566);
    }
}
