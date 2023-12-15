use std::str::FromStr;

use anyhow::{bail, Error, Result};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Rock {
    Round,
    Square,
    None,
}

impl TryFrom<char> for Rock {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '.' => Rock::None,
            'O' => Rock::Round,
            '#' => Rock::Square,
            _ => bail!("{value} is not a rock or empty space"),
        })
    }
}

#[derive(Default, Eq, PartialEq, Debug)]
struct Platform(Vec<Vec<Rock>>);

impl FromStr for Platform {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut platform = Self::default();
        for l in s.lines() {
            let mut row: Vec<Rock> = Vec::new();
            for c in l.chars() {
                row.push(c.try_into()?);
            }
            platform.0.push(row);
        }
        Ok(platform)
    }
}

impl Platform {
    fn transpose(&self) -> Self {
        let mut transposed = Self::default();
        for col in 0..self.0[0].len() {
            let mut column: Vec<Rock> = Vec::new();
            for row in 0..self.0.len() {
                column.push(self.0[row][col]);
            }
            transposed.0.push(column)
        }
        transposed
    }

    fn tilt_north(&self) -> Self {
        let mut columnar = self.transpose();
        for column in &mut columnar.0 {
            let mut next_available_slot = 0;
            for i in 0..column.len() {
                let current_rock = column[i];
                match current_rock {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i+1,
                    Rock::Round => {
                        column[i] = Rock::None;
                        column[next_available_slot] = Rock::Round;
                        next_available_slot += 1;
                    }
                }
            }
        }
        columnar.transpose()
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let platform: Platform = input.parse()?;
    let north_tilted = platform.tilt_north();
    let num_rows = north_tilted.0.len();
    let mut total_value = 0;
    for (i, row) in north_tilted.0.into_iter().enumerate() {
        let row_value = row.into_iter().filter(|r| r == &Rock::Round).count() * (num_rows - i);
        total_value += row_value;
    }
    Ok(total_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    const EXAMPLE_TILTED_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
";

    #[test]
    fn test_tilt_example_north() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        let platform_tilted_north: Platform = EXAMPLE_TILTED_NORTH.parse().unwrap();
        assert_eq!(platform.tilt_north(), platform_tilted_north);
    }

    #[test]
    fn test_transpose() {
        let platform: Platform = "#.#\n.O.".parse().unwrap();
        let expected: Platform = "#.\n.O\n#.".parse().unwrap();
        assert_eq!(platform.transpose(), expected);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 136);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 108759);
    }
}
