use anyhow::{anyhow, Error, Result};
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

// #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
// struct Card(u8);

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
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
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
}

impl FromStr for Hand {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((cards, bid)) = s.split_once(' ') {
            let cards: [Card; 5] = cards.chars().map(|c| c.into()).collect::<Vec<Card>>()[..].try_into()?;
            Ok(Self { cards, bid: bid.parse()?, hand_type: (&cards).into() })
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
        assert_eq!(
            hand,
            Hand {
                cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair,
                bid: 765
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
        let h1 = Hand {
            cards: [Card::Three, Card::Four, Card::Five, Card::Six, Card::Nine],
            hand_type: HandType::HighCard,
            bid: 0,
        };
        let h2 = Hand {
            cards: [Card::Three, Card::Four, Card::Five, Card::Eight, Card::Six],
            hand_type: HandType::HighCard,
            bid: 0,
        };
        let h3 = Hand {
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
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 6440);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 256448566);
    }
}
