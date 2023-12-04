use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug, PartialEq)]
struct Card {
    winning_numbers: Vec<u8>,
    my_numbers: Vec<u8>,
}

impl Card {
    fn points(&self) -> u32 {
        // Contains should be fast with such a small vector
        let num_winning = self.my_numbers.iter().filter(|n| self.winning_numbers.contains(n)).count();
        if num_winning == 0 {
            0
        } else {
            // Take a power of two by left bitshift
            1 << (num_winning - 1)
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((_, numbers)) = s.split_once(':') {
            if let Some((winning, mine)) = numbers.split_once('|') {
                return Ok(Self {
                    winning_numbers: winning.split_ascii_whitespace().flat_map(|val| val.parse()).collect(),
                    my_numbers: mine.split_ascii_whitespace().flat_map(|val| val.parse()).collect(),
                });
            }
        }
        Ok(Self { winning_numbers: vec![], my_numbers: vec![] })
    }
}

pub fn part1(input: &str) -> u32 {
    // NOTE: this silently drops any parsing errors.
    input.lines().flat_map(|l| l.parse::<Card>()).map(|c| c.points()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_parse_card() {
        let card: Card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse().unwrap();
        assert_eq!(
            card,
            Card { winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] }
        );
    }

    #[test]
    fn test_card_points() {
        let card = Card { winning_numbers: vec![41, 48, 83, 86, 17], my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53] };
        assert_eq!(card.points(), 8);
        assert_eq!(Card { winning_numbers: vec![], my_numbers: vec![] }.points(), 0);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
        assert_eq!(part1(&input), 15205);
    }
}
