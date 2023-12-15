use anyhow::{bail, Error, Result};
use std::{collections::HashMap, str::FromStr};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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

#[derive(Default, Eq, PartialEq, Debug, Hash)]
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
                match column[i] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i + 1,
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

    fn tilt_south(&self) -> Self {
        let mut columnar = self.transpose();
        for column in &mut columnar.0 {
            let mut next_available_slot = column.len() - 1;
            for i in (0..column.len()).rev() {
                match column[i] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i.saturating_sub(1),
                    Rock::Round => {
                        column[i] = Rock::None;
                        column[next_available_slot] = Rock::Round;
                        next_available_slot = next_available_slot.saturating_sub(1);
                    }
                }
            }
        }
        columnar.transpose()
    }

    fn tilt_west(mut self) -> Self {
        for row in &mut self.0 {
            let mut next_available_slot = 0;
            for i in 0..row.len() {
                match row[i] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i + 1,
                    Rock::Round => {
                        row[i] = Rock::None;
                        row[next_available_slot] = Rock::Round;
                        next_available_slot += 1;
                    }
                }
            }
        }
        self
    }

    fn tilt_east(mut self) -> Self {
        for row in &mut self.0 {
            let mut next_available_slot = row.len() - 1;
            for i in (0..row.len()).rev() {
                match row[i] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i.saturating_sub(1),
                    Rock::Round => {
                        row[i] = Rock::None;
                        row[next_available_slot] = Rock::Round;
                        next_available_slot = next_available_slot.saturating_sub(1);
                    }
                }
            }
        }
        self
    }

    fn tilt_cycle(&self) -> Self {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    fn load_on_north_support_beams(&self) -> usize {
        let mut total_value = 0;
        for (i, row) in self.0.iter().enumerate() {
            let row_value = row.iter().filter(|&r| r == &Rock::Round).count() * (self.0.len() - i);
            total_value += row_value;
        }
        total_value
    }
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(input.parse::<Platform>()?.tilt_north().load_on_north_support_beams())
}

pub fn part2(input: &str) -> Result<usize> {
    let mut platform: Platform = input.parse()?;
    let mut visited: HashMap<Platform, usize> = HashMap::default();
    const NUM_ITERATIONS: usize = 1000000000;

    for i in 0..NUM_ITERATIONS {
        let next_platform = platform.tilt_cycle();
        if let Some(visited_before) = visited.insert(platform, i) {
            // We have detected a cycle
            let cycle_length = i - visited_before;
            let mut remaining_iterations = (NUM_ITERATIONS - visited_before) % cycle_length;
            if remaining_iterations == 0 {
                // bit hacky: because we already computed the next platform and move the current one
                // we will need to cycle an extra time if the current platform is the ending state.
                remaining_iterations = cycle_length;
            }
            platform = next_platform;
            // Remove 1 since we already computed one iteration
            for _ in 0..(remaining_iterations - 1) {
                platform = platform.tilt_cycle();
            }
            break;
        }
        platform = next_platform;
    }
    Ok(platform.load_on_north_support_beams())
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

    const EXAMPLE_TILT_CYCLE_1: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
";

    #[test]
    fn test_tilt_example_north() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        let platform_tilted_north: Platform = EXAMPLE_TILTED_NORTH.parse().unwrap();
        assert_eq!(platform.tilt_north(), platform_tilted_north);
        assert_eq!(platform.tilt_south().tilt_north(), platform_tilted_north);
    }

    #[test]
    fn test_transpose() {
        let platform: Platform = "#.#\n.O.".parse().unwrap();
        let expected: Platform = "#.\n.O\n#.".parse().unwrap();
        assert_eq!(platform.transpose(), expected);

        let example: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(example.transpose().transpose(), example);
    }

    #[test]
    fn test_tilt_cycle() {
        let platform: Platform = EXAMPLE.parse().unwrap();
        assert_eq!(platform.tilt_cycle(), EXAMPLE_TILT_CYCLE_1.parse().unwrap());
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 136);
        assert_eq!(part2(EXAMPLE).unwrap(), 64);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 108759);
        assert_eq!(part2(&input).unwrap(), 89089);
    }
}
