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
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn value(&self) -> usize {
        if let Some(line) = self.vertical_reflection_line() {
            return line;
        }

        if let Some(line) = self.horizontal_reflection_line() {
            return 100 * line;
        }

        panic!("No line of reflection");
    }

    fn is_vertical_reflection_line(&self, mut right_idx: usize) -> bool {
        let mut left_idx = right_idx - 1; // Assume that right_idx >= 1
        loop {
            for row in self.0.iter() {
                if row[left_idx] != row[right_idx] {
                    // There is a mismatch: this is not a line of reflection
                    return false;
                }
            }
            if left_idx == 0 || right_idx == self.width() - 1 {
                // We ignore any extra columns on one side or the other: we have found a line of reflection
                return true
            }
            // Updating indicies is safe due to the while loop condition
            left_idx -= 1;
            right_idx += 1;
        }
    }

    fn is_horizontal_reflection_line(&self, mut down_idx: usize) -> bool {
        let mut up_idx = down_idx - 1; // Assume that down_idx >= 1
        loop {
            if self.0[up_idx] != self.0[down_idx] {
                return false;
            }
            if up_idx == 0 || down_idx == self.height() - 1 {
                // Ignoring any extra rows, this is a line of reflection
                return true
            }
            up_idx -= 1;
            down_idx += 1;
        }
    }

    fn vertical_reflection_line(&self) -> Option<usize> {
        for i in 1..self.width() {
            if self.is_vertical_reflection_line(i) {
                return Some(i);
            }
        }
        None
    }
    fn horizontal_reflection_line(&self) -> Option<usize> {
        for i in 1..self.height() {
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
        assert_eq!(part1(EXAMPLE).unwrap(), 405);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 39939);
    }
}
