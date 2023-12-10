pub fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;
    for l in input.lines() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in l.chars() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10);
                if first.is_none() {
                    first = digit;
                }
                last = digit;
            }
        }
        total += first.unwrap() * 10 + last.unwrap();
    }
    total
}

#[derive(Debug, PartialEq)]
enum WordNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl WordNumber {
    fn from_front(text: &str) -> Option<Self> {
        let mut text_chars = text.chars();
        match text_chars.next() {
            Some('o') => {
                if text_chars.next().is_some_and(|c| c == 'n') && text_chars.next().is_some_and(|c| c == 'e') {
                    Some(Self::One)
                } else {
                    None
                }
            }
            Some('t') => match text_chars.next() {
                Some('w') => {
                    if text_chars.next().is_some_and(|c| c == 'o') {
                        Some(Self::Two)
                    } else {
                        None
                    }
                }
                Some('h') => {
                    if text_chars.next().is_some_and(|c| c == 'r')
                        && text_chars.next().is_some_and(|c| c == 'e')
                        && text_chars.next().is_some_and(|c| c == 'e')
                    {
                        Some(Self::Three)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Some('f') => match text_chars.next() {
                Some('o') => {
                    if text_chars.next().is_some_and(|c| c == 'u') && text_chars.next().is_some_and(|c| c == 'r') {
                        Some(Self::Four)
                    } else {
                        None
                    }
                }
                Some('i') => {
                    if text_chars.next().is_some_and(|c| c == 'v') && text_chars.next().is_some_and(|c| c == 'e') {
                        Some(Self::Five)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Some('s') => match text_chars.next() {
                Some('i') => {
                    if text_chars.next().is_some_and(|c| c == 'x') {
                        Some(Self::Six)
                    } else {
                        None
                    }
                }
                Some('e') => {
                    if text_chars.next().is_some_and(|c| c == 'v')
                        && text_chars.next().is_some_and(|c| c == 'e')
                        && text_chars.next().is_some_and(|c| c == 'n')
                    {
                        Some(Self::Seven)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Some('e') => {
                if text_chars.next().is_some_and(|c| c == 'i')
                    && text_chars.next().is_some_and(|c| c == 'g')
                    && text_chars.next().is_some_and(|c| c == 'h')
                    && text_chars.next().is_some_and(|c| c == 't')
                {
                    Some(Self::Eight)
                } else {
                    None
                }
            }
            Some('n') => {
                if text_chars.next().is_some_and(|c| c == 'i')
                    && text_chars.next().is_some_and(|c| c == 'n')
                    && text_chars.next().is_some_and(|c| c == 'e')
                {
                    Some(Self::Nine)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn to_digit(&self) -> u32 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

fn to_digit_sequence(mut input: &str) -> Vec<u32> {
    let mut digit_sequence: Vec<u32> = Vec::new();

    while !input.is_empty() {
        // Slice is not empty, unwrap is safe
        if let Some(digit) = input.chars().next().unwrap().to_digit(10) {
            digit_sequence.push(digit);
        } else if let Some(word_number) = WordNumber::from_front(input) {
            digit_sequence.push(word_number.to_digit());
        }
        input = &input[1..];
    }

    digit_sequence
}

fn calibration_value(input_line: &str) -> u32 {
    let digits = to_digit_sequence(input_line);
    // Assume input has at least one digit (or we panic)
    let start = digits.first().unwrap();
    let end = digits.last().unwrap();
    start * 10 + end
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(calibration_value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_P2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 142);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(EXAMPLE_P2), 281);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(part1(&input), 55130);
        assert_eq!(part2(&input), 54985);
    }

    #[test]
    fn test_word_number_from_front() {
        assert_eq!(WordNumber::from_front("one").unwrap(), WordNumber::One);
        assert_eq!(WordNumber::from_front("two").unwrap(), WordNumber::Two);
        assert_eq!(WordNumber::from_front("three").unwrap(), WordNumber::Three);
        assert_eq!(WordNumber::from_front("four").unwrap(), WordNumber::Four);
        assert_eq!(WordNumber::from_front("five").unwrap(), WordNumber::Five);
        assert_eq!(WordNumber::from_front("six").unwrap(), WordNumber::Six);
        assert_eq!(WordNumber::from_front("seven").unwrap(), WordNumber::Seven);
        assert_eq!(WordNumber::from_front("eight").unwrap(), WordNumber::Eight);
        assert_eq!(WordNumber::from_front("nine").unwrap(), WordNumber::Nine);

        assert_eq!(WordNumber::from_front("oneight").unwrap(), WordNumber::One);

        assert!(WordNumber::from_front("").is_none());
        assert!(WordNumber::from_front("o").is_none());
        assert!(WordNumber::from_front("on").is_none());
        assert!(WordNumber::from_front("ont").is_none());
        assert!(WordNumber::from_front("oone").is_none());
    }

    #[test]
    fn test_to_digit_sequence() {
        assert_eq!(to_digit_sequence("1234"), [1, 2, 3, 4]);
        assert_eq!(to_digit_sequence("arandomstringoftext"), []);
        assert_eq!(to_digit_sequence("one24threeightwo7on9"), [1, 2, 4, 3, 8, 2, 7, 9]);
        assert_eq!(to_digit_sequence(""), []);
        assert_eq!(to_digit_sequence("arandomstringoftext"), []);
    }

    #[test]
    fn test_calibration_value() {
        assert_eq!(calibration_value("eightwothree"), 83);
        assert_eq!(calibration_value("oneight"), 18);
        assert_eq!(calibration_value("3oneight"), 38);
        assert_eq!(calibration_value("oneight9"), 19);
        assert_eq!(calibration_value("one"), 11);
        assert_eq!(calibration_value("onety"), 11);
        assert_eq!(calibration_value("2asdf"), 22);
        assert_eq!(calibration_value("9"), 99);
    }
}
