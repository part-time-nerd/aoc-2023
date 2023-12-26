use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

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

    fn value(&self, changes: usize) -> Result<usize> {
        for i in 1..self.width() {
            if self.required_changes_for_vertical_line_of_reflection(i) == changes {
                return Ok(i);
            }
        }

        for i in 1..self.height() {
            if self.required_changes_for_horizontal_line_of_reflection(i) == changes {
                return Ok(100 * i);
            }
        }

        Err(anyhow!("No line of reflection"))
    }

    fn required_changes_for_vertical_line_of_reflection(&self, mut right_idx: usize) -> usize {
        let mut mismatches = 0;

        let mut left_idx = right_idx - 1; // Assume that right_idx >= 1
        loop {
            for row in self.0.iter() {
                if row[left_idx] != row[right_idx] {
                    mismatches += 1;
                }
            }
            if left_idx == 0 || right_idx == self.width() - 1 {
                return mismatches;
            }
            left_idx -= 1;
            right_idx += 1;
        }
    }

    fn required_changes_for_horizontal_line_of_reflection(&self, mut down_idx: usize) -> usize {
        let mut mismatches = 0;

        let mut up_idx = down_idx - 1; // Assume that down_idx >= 1
        loop {
            mismatches += self.0[up_idx].iter().zip(self.0[down_idx].iter()).filter(|(a, b)| a != b).count();
            if up_idx == 0 || down_idx == self.height() - 1 {
                return mismatches;
            }
            up_idx -= 1;
            down_idx += 1;
        }
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let mut total = 0;
    for pattern in parse_input(input)? {
        total += pattern.value(0)?;
    }
    Ok(total)
}

pub fn part2(input: &str) -> Result<usize> {
    let mut total = 0;
    for pattern in parse_input(input)? {
        total += pattern.value(1)?;
    }
    Ok(total)
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
        assert_eq!(part2(EXAMPLE).unwrap(), 400);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 39939);
        assert_eq!(part2(&input).unwrap(), 32069);
    }
}
