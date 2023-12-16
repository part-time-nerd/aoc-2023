use anyhow::{bail, Error, Result};
use std::{collections::HashMap, str::FromStr};

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
    fn tilt_north(mut self) -> Self {
        for j in 0..self.0[0].len() {
            let mut next_available_slot = 0;
            for i in 0..self.0.len() {
                match self.0[i][j] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i + 1,
                    Rock::Round => {
                        self.0[i][j] = Rock::None;
                        self.0[next_available_slot][j] = Rock::Round;
                        next_available_slot += 1;
                    }
                }
            }
        }
        self
    }

    fn tilt_south(mut self) -> Self {
        for j in 0..self.0[0].len() {
            let mut next_available_slot = self.0[0].len() - 1;
            for i in (0..self.0.len()).rev() {
                match self.0[i][j] {
                    Rock::None => (),
                    Rock::Square => next_available_slot = i.saturating_sub(1),
                    Rock::Round => {
                        self.0[i][j] = Rock::None;
                        self.0[next_available_slot][j] = Rock::Round;
                        next_available_slot = next_available_slot.saturating_sub(1);
                    }
                }
            }
        }
        self
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

    fn tilt_cycle(self) -> Self {
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

    fn identity(&self) -> Vec<(usize, usize)> {
        // Returns a (sorted) vector of round rocks. This is the relevant piece of cycle detection
        let mut round_rocks: Vec<(usize, usize)> = Vec::new();
        for (i, row) in self.0.iter().enumerate() {
            for (j, r) in row.iter().enumerate() {
                if r == &Rock::Round {
                    round_rocks.push((i, j));
                }
            }
        }
        round_rocks
    }
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(input.parse::<Platform>()?.tilt_north().load_on_north_support_beams())
}

pub fn part2(input: &str) -> Result<usize> {
    let mut platform: Platform = input.parse()?;
    let mut visited: HashMap<Vec<(usize, usize)>, usize> = HashMap::default();
    const NUM_ITERATIONS: usize = 1000000000;

    for i in 0..NUM_ITERATIONS {
        if let Some(visited_before) = visited.insert(platform.identity(), i) {
            // There is a cycle: compute the final platform based on the remaining iterations after cycling
            for _ in 0..((NUM_ITERATIONS - visited_before) % (i - visited_before)) {
                platform = platform.tilt_cycle();
            }
            break;
        }
        platform = platform.tilt_cycle();
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
