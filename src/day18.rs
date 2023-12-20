use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            _ => Err(anyhow!("Could not parse direction from {s}")),
        }
    }
}

impl From<Direction> for Posn {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Default, Copy, Clone)]
struct Color(u32);

impl FromStr for Color {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        Ok(Self(u32::from_str_radix(
            input.strip_prefix("(#").context("missing prefix")?.strip_suffix(')').context("missing suffix")?,
            16,
        )?))
    }
}

struct DigInstruction {
    direction: Direction,
    amount: usize,
    color: Color,
}

impl FromStr for DigInstruction {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        if let [dir, amt, col] = input.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            Ok(Self { direction: dir.parse()?, amount: amt.parse()?, color: col.parse()? })
        } else {
            Err(anyhow!("Could not split the dig instruction into three parts"))
        }
    }
}

type Posn = (isize, isize);
impl DigInstruction {
    fn dig(&self, mut start: Posn) -> Vec<(Posn, Color)> {
        let direction: Posn = self.direction.into();
        let mut dugout: Vec<(Posn, Color)> = Vec::new();
        for _ in 0..self.amount {
            start = (start.0 + direction.0, start.1 + direction.1);
            dugout.push((start, self.color));
        }
        dugout
    }
}

struct DigPlan(Vec<DigInstruction>);

impl FromStr for DigPlan {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut instructions: Vec<DigInstruction> = Vec::new();
        for l in input.lines() {
            instructions.push(l.parse()?);
        }
        Ok(Self(instructions))
    }
}

impl DigPlan {
    fn dig(&self) -> Vec<(Posn, Color)> {
        let mut dugout: Vec<(Posn, Color)> = Vec::new();
        for instruction in self.0.iter() {
            let start = if let Some((start, _)) = dugout.last() { *start } else { (0, 0) };
            dugout.extend(instruction.dig(start));
        }
        dugout
    }

    fn dig_map(&self) -> Vec<Vec<bool>> {
        let dugout = self.dig();
        if dugout.is_empty() {
            return Vec::new(); // Not relevant for AOC, but lets be good coders
        }
        let mut min: Posn = dugout[0].0;
        let mut max: Posn = dugout[0].0;
        for (dug, _) in dugout.iter().skip(1) {
            min = (dug.0.min(min.0), dug.1.min(min.1));
            max = (dug.0.max(max.0), dug.1.max(max.1));
        }

        let height = (max.0 + 1) - min.0;
        let width = (max.1 + 1) - min.1;

        let mut map: Vec<Vec<bool>> = (0..height).map(|_| (0..width).map(|_| false).collect()).collect();

        for (dug, _) in dugout.iter() {
            let dug_idx = (dug.0 - min.0, dug.1 - min.1);
            map[dug_idx.0 as usize][dug_idx.1 as usize] = true;
        }

        map
    }

    fn dig_map_full(&self) -> Vec<Vec<bool>> {
        #[derive(PartialEq, Eq)]
        enum DigState {
            Inside,
            Outside,
            InnerEdge,
            OuterEdge,
        }

        let mut dig_map = self.dig_map();
        for row in dig_map.iter_mut() {
            let mut state = DigState::Outside;
            for item in row.iter_mut() {
                match state {
                    DigState::Outside => {
                        if *item {
                            state = DigState::InnerEdge;
                        }
                    }
                    DigState::Inside => {
                        if *item {
                            state = DigState::OuterEdge;
                        } else {
                            *item = true;
                        }
                    }
                    DigState::InnerEdge => {
                        if !*item {
                            state = DigState::Inside;
                            *item = true;
                        }
                    }
                    DigState::OuterEdge => {
                        if !*item {
                            state = DigState::Outside;
                        }
                    }
                }
            }
        }

        dig_map
    }
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(input.parse::<DigPlan>()?.dig_map_full().iter().map(|r| r.iter().filter(|&&v| v).count()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    const EXPECTED_DIG_MAP: &str = "#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
";

    const EXPECTED_FILLED_DIG_MAP: &str = "#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######
";

    #[test]
    fn test_dig_map() {
        let plan: DigPlan = EXAMPLE.parse().unwrap();
        let expected: Vec<Vec<bool>> =
            EXPECTED_DIG_MAP.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
        assert_eq!(plan.dig_map(), expected);
    }

    #[test]
    fn test_dig_map_full() {
        let plan: DigPlan = EXAMPLE.parse().unwrap();
        let expected: Vec<Vec<bool>> =
            EXPECTED_FILLED_DIG_MAP.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
        assert_eq!(plan.dig_map_full(), expected);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 62);
    }
}
