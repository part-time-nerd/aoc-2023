use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            '>' => Ok(Self::Right),
            '<' => Ok(Self::Left),
            'v' => Ok(Self::Down),
            '^' => Ok(Self::Up),
            _ => Err(anyhow!("{value} is not a direction (expected '>', '<', '^', or 'v')")),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Self::Path),
            '#' => Ok(Self::Forest),
            v => Ok(Self::Slope(v.try_into()?)),
        }
    }
}

struct Map(Vec<Vec<Tile>>);

impl FromStr for Map {
    type Err = Error;
    fn from_str(input: &str) -> Result<Map> {
        let mut map: Vec<Vec<Tile>> = Vec::new();
        for l in input.lines() {
            let mut row: Vec<Tile> = Vec::new();
            for c in l.chars() {
                row.push(c.try_into()?);
            }
            map.push(row);
        }
        Ok(Self(map))
    }
}

impl Map {
    fn adjacent(&self, posn: Posn) -> Vec<Posn> {
        match &self.0[posn.0][posn.1] {
            Tile::Path => todo!(),
            Tile::Slope(Direction::Down) if posn.0 < self.0.len() - 1 && self.0[posn.0 + 1][posn.1] != Tile::Forest => {
                vec![(posn.0 + 1, posn.1)]
            }
            Tile::Slope(Direction::Up) if posn.0 > 0 && self.0[posn.0 - 1][posn.1] != Tile::Forest => {
                vec![(posn.0 - 1, posn.1)]
            }
            Tile::Slope(Direction::Left) if posn.1 > 0 && self.0[posn.0][posn.1 - 1] != Tile::Forest => {
                vec![(posn.0, posn.1 - 1)]
            }
            Tile::Slope(Direction::Right)
                if posn.1 < self.0[posn.0].len() - 1 && self.0[posn.0][posn.1 + 1] != Tile::Forest =>
            {
                vec![(posn.0, posn.1 + 1)]
            }
            _ => vec![],
        }
    }
}

type Posn = (usize, usize);

pub fn part1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let start: Posn = (0, 1);
    let target: Posn = (map.0.len() - 1, map.0[map.0.len() - 1].len() - 2);
    Ok(0)
}

mod tests {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 94);
    }
}
