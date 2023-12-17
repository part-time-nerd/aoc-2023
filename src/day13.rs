use std::str::FromStr;

use anyhow::{Error, Result};

struct Pattern(Vec<Vec<bool>>);

impl FromStr for Pattern {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(s.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect()))
    }
}

fn parse_input(input: &str) -> Result<Vec<Pattern>> {
    let mut patterns: Vec<Pattern> = Vec::new();
    for p in input.split("\n\n") {
        patterns.push(p.parse()?);
    }
    Ok(patterns)
}

impl Pattern {
    fn value(&self) -> usize {
        if let Some(line) = self.vertical_reflection_line() {
            return line;
        }

        if let Some(line) = self.horizontal_reflection_line() {
            return 100 * line;
        }

        panic!("No line of reflection");
    }

    fn is_vertical_reflection_line(&self, idx: usize) -> bool {
        for offset in 1.. {
            if idx > offset || (idx + offset - 1) == self.0[0].len() {
                break;
            }
            for row in self.0.iter() {
                if row[idx - offset] != row[idx + offset - 1] {
                    return false;
                }
            }
        }
        true
    }

    fn is_horizontal_reflection_line(&self, idx: usize) -> bool {
        for offset in 1.. {
            if idx > offset || (idx + offset - 1) == self.0.len() {
                break;
            }
            if self.0[idx - offset] != self.0[idx + offset - 1] {
                return false;
            }
        }
        true
    }

    fn vertical_reflection_line(&self) -> Option<usize> {
        for i in 1..self.0[0].len() {
            if self.is_vertical_reflection_line(i) {
                return Some(i);
            }
        }
        None
    }
    fn horizontal_reflection_line(&self) -> Option<usize> {
        for i in 1..self.0.len() {
            if self.is_horizontal_reflection_line(i) {
                return Some(i);
            }
        }
        None
    }
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.into_iter().map(|p| p.value()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_example() {
        // assert_eq!(part1(EXAMPLE).unwrap(), 405);
    }
}
