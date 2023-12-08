use anyhow::{anyhow, Error, Result};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card {
    Two = 2,
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

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Card can't be {value}"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct HandP1 {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

impl Ord for HandP1 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for HandP1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for HandP1 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((cards, bid)) = s.split_once(' ') {
            let cards: [Card; 5] = cards.chars().map(|c| c.into()).collect::<Vec<Card>>()[..].try_into()?;
            Ok(Self { cards, bid: bid.parse()?, hand_type: HandType::parse_p1(&cards) })
        } else {
            Err(anyhow!("Could not split the hand into a (cards, bid) tuple: {s}"))
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct HandP2 {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

impl Ord for HandP2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                // Could update the cards to contain a Joker, but I will just manually implement this instead
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match (a, b) {
                        (Card::Jack, Card::Jack) => {}
                        (Card::Jack, _) => return Ordering::Less,
                        (_, Card::Jack) => return Ordering::Greater,
                        (_, _) => match a.cmp(b) {
                            Ordering::Equal => {}
                            ordering => return ordering,
                        },
                    }
                }
                Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

impl PartialOrd for HandP2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for HandP2 {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((cards, bid)) = s.split_once(' ') {
            let cards: [Card; 5] = cards.chars().map(|c| c.into()).collect::<Vec<Card>>()[..].try_into()?;
            Ok(Self { cards, bid: bid.parse()?, hand_type: HandType::parse_p2(&cards) })
        } else {
            Err(anyhow!("Could not split the hand into a (cards, bid) tuple: {s}"))
        }
    }
}
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn parse_p1(cards: &[Card; 5]) -> Self {
        let mut counts: HashMap<Card, u8> = HashMap::default();

        for card in cards.iter() {
            if let Some(count) = counts.get_mut(card) {
                *count += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        match counts.len() {
            1 => HandType::FiveOfAKind,
            2 => match counts.values().collect::<Vec<&u8>>()[..] {
                [3, 2] | [2, 3] => HandType::FullHouse,
                [4, 1] | [1, 4] => HandType::FourOfAKind,
                _ => panic!("Unexpected counts of cards"),
            },
            3 => match counts.values().collect::<Vec<&u8>>()[..] {
                [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => HandType::TwoPair,
                [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => HandType::ThreeOfAKind,
                _ => panic!("Unexpected counts of cards"),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Unexpected number of entries"),
        }
    }

    fn upgrade_single_joker(&self) -> Self {
        match self {
            Self::HighCard => Self::OnePair,
            Self::OnePair => Self::ThreeOfAKind,
            Self::TwoPair => Self::FullHouse,
            Self::ThreeOfAKind => Self::FourOfAKind,
            Self::FullHouse => panic!("Cannot upgrade full house with a single joker"),
            Self::FourOfAKind => Self::FiveOfAKind,
            Self::FiveOfAKind => panic!("Cannot upgrade five of a kind with a single joker"),
        }
    }

    fn upgrade_two_jokers(&self) -> Self {
        match self {
            Self::HighCard => panic!("There should be a pair of jokers: high card is impossible"),
            Self::OnePair => Self::ThreeOfAKind, // Pair is jokers
            Self::TwoPair => Self::FourOfAKind,  // One pair is jokers
            Self::ThreeOfAKind => panic!("There should be a pair of jokers: three of a kind means 0, 1, or 3 jokers"),
            Self::FullHouse => Self::FiveOfAKind, // The two cards are jokers
            Self::FourOfAKind => panic!("Cannot upgrade four of a kind with a pair of jokers"),
            Self::FiveOfAKind => panic!("Cannot upgrade five of a kind with a pair of jokers"),
        }
    }

    fn parse_p2(cards: &[Card; 5]) -> Self {
        let remaining_cards: Vec<&Card> = cards.iter().filter(|&&c| c != Card::Jack).collect();
        match remaining_cards.len() {
            5 => Self::parse_p1(cards),
            0 | 1 => Self::FiveOfAKind,
            2 => {
                if remaining_cards[0] == remaining_cards[1] {
                    Self::FiveOfAKind
                } else {
                    Self::FourOfAKind
                }
            }
            3 => Self::parse_p1(cards).upgrade_two_jokers(),
            4 => Self::parse_p1(cards).upgrade_single_joker(),
            _ => panic!(),
        }
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let mut hands: Vec<HandP1> = Vec::new();
    for hand in input.lines() {
        hands.push(hand.parse()?);
    }
    hands.sort();
    Ok(hands.into_iter().enumerate().map(|(i, hand)| (i + 1) * hand.bid).sum())
}

pub fn part2(input: &str) -> Result<usize> {
    let mut hands: Vec<HandP2> = Vec::new();
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
    fn test_hand_p1_from_string() {
        let hand: HandP1 = "32T3K 765".parse().unwrap();
        assert_eq!(
            hand,
            HandP1 {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair,
                bid: 765
            }
        );
    }

    #[test]
    fn test_hand_p2_from_string() {
        let hand: HandP2 = "KTJJT 220".parse().unwrap();
        assert_eq!(
            hand,
            HandP2 {
                cards: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                hand_type: HandType::FourOfAKind,
                bid: 220
            }
        );
    }

    #[test]
    fn test_hand_type_ordering() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::ThreeOfAKind < HandType::FullHouse);
        assert!(HandType::OnePair == HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);
    }

    #[test]
    fn test_hand_ordering() {
        let h1 = HandP1 {
            cards: [Card::Three, Card::Four, Card::Five, Card::Six, Card::Nine],
            hand_type: HandType::HighCard,
            bid: 0,
        };
        let h2 = HandP1 {
            cards: [Card::Three, Card::Four, Card::Five, Card::Eight, Card::Six],
            hand_type: HandType::HighCard,
            bid: 0,
        };
        let h3 = HandP1 {
            cards: [Card::Two, Card::Two, Card::Three, Card::Four, Card::Five],
            hand_type: HandType::OnePair,
            bid: 0,
        };
        assert!(h1 == h1);
        assert!(h1 < h2);
        assert!(h3 > h1);
        assert!(h3 > h2);
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 6440);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(EXAMPLE).unwrap(), 5905);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 256448566);
        assert_eq!(part2(&input).unwrap(), 254412181);
    }
}
