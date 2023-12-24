use anyhow::{bail, Context, Error, Result};
use std::{collections::HashMap, str::FromStr};

enum Tile {
    Garden,
    Rock,
}

type Posn = (usize, usize);

struct Map {
    start: Posn,
    tiles: Vec<Vec<Tile>>,
}

enum Wrapping {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut start: Option<Posn> = None;
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for l in input.lines() {
            let mut row: Vec<Tile> = Vec::new();
            for c in l.chars() {
                match c {
                    '.' => row.push(Tile::Garden),
                    '#' => row.push(Tile::Rock),
                    'S' => {
                        if start.is_some() {
                            bail!("multiple start tiles parsed");
                        }
                        start = Some((tiles.len(), row.len()));
                        row.push(Tile::Garden);
                    }
                    _ => bail!("Unexpected tile (should be '.', '#', or 'S'): {c}"),
                }
            }
            tiles.push(row);
        }
        Ok(Self { tiles, start: start.context("Did not find the start tile")? })
    }
}

impl Map {
    fn successors(&self, posn: &Posn) -> Vec<Posn> {
        let mut successors: Vec<Posn> = Vec::new();
        if posn.0 != 0 {
            if let Tile::Garden = self.tiles[posn.0 - 1][posn.1] {
                successors.push((posn.0 - 1, posn.1));
            }
        }
        if posn.0 != self.tiles.len() - 1 {
            if let Tile::Garden = self.tiles[posn.0 + 1][posn.1] {
                successors.push((posn.0 + 1, posn.1));
            }
        }
        if posn.1 != 0 {
            if let Tile::Garden = self.tiles[posn.0][posn.1 - 1] {
                successors.push((posn.0, posn.1 - 1));
            }
        }
        if posn.1 != self.tiles[posn.0].len() - 1 {
            if let Tile::Garden = self.tiles[posn.0][posn.1 + 1] {
                successors.push((posn.0, posn.1 + 1));
            }
        }
        successors
    }

    fn successors_with_wrapping(&self, posn: &Posn) -> Vec<(Posn, Option<Wrapping>)> {
        let mut successors: Vec<(Posn, Option<Wrapping>)> = Vec::new();
        if posn.0 != 0 {
            if let Tile::Garden = self.tiles[posn.0 - 1][posn.1] {
                successors.push(((posn.0 - 1, posn.1), None));
            }
        } else {
            if let Tile::Garden = self.tiles[self.tiles.len() - 1][posn.1] {
                successors.push(((self.tiles.len() - 1, posn.1), Some(Wrapping::Up)));
            }
        }
        if posn.0 != self.tiles.len() - 1 {
            if let Tile::Garden = self.tiles[posn.0 + 1][posn.1] {
                successors.push(((posn.0 + 1, posn.1), None));
            }
        } else {
            if let Tile::Garden = self.tiles[0][posn.1] {
                successors.push(((0, posn.1), Some(Wrapping::Down)));
            }
        }
        if posn.1 != 0 {
            if let Tile::Garden = self.tiles[posn.0][posn.1 - 1] {
                successors.push(((posn.0, posn.1 - 1), None));
            }
        } else {
            if let Tile::Garden = self.tiles[posn.0][self.tiles[posn.0].len() - 1] {
                successors.push(((posn.0, self.tiles[posn.0].len() - 1), Some(Wrapping::Left)));
            }
        }
        if posn.1 != self.tiles[posn.0].len() - 1 {
            if let Tile::Garden = self.tiles[posn.0][posn.1 + 1] {
                successors.push(((posn.0, posn.1 + 1), None));
            }
        } else {
            if let Tile::Garden = self.tiles[posn.0][0] {
                successors.push(((posn.0, 0), Some(Wrapping::Right)));
            }
        }
        successors
    }
}

pub fn part1(input: &str, steps: usize) -> Result<usize> {
    let map: Map = input.parse()?;
    let mut reachable: Vec<Posn> = vec![map.start];
    for _ in 0..steps {
        reachable = reachable.into_iter().flat_map(|s| map.successors(&s)).collect();
        reachable.sort_unstable();
        reachable.dedup();
    }
    Ok(reachable.len())
}

pub fn part2(input: &str, steps: usize) -> Result<usize> {
    // Since we can always move away from a plot and then back on to it, once we have reached
    // a plot in an even number of steps, we can reach it any any subsequent even number of steps.
    // The same goes for odd numbers: any subsequent odd number of steps we can be at the plot.
    // Thus finding the minimum odd and even number of steps to reach a garden plot is crucial:
    // Any number of steps after this point will also be able to reach that plot.
    // There should be many copies of the map that we can reach completely
    // Then, for the "edge cases", we just need to figure out how much of them can be filled
    //

    let map: Map = input.parse()?;
    let mut reachable: Vec<(Posn, isize, isize)> = vec![(map.start, 0, 0)];
    for _ in 0..steps {
        reachable = reachable
            .into_iter()
            .flat_map(|(posn, garden_i, garden_j)| {
                map.successors_with_wrapping(&posn).into_iter().map(move |(neighbour, wrapping)| match wrapping {
                    Some(Wrapping::Up) => (neighbour, garden_i - 1, garden_j),
                    Some(Wrapping::Down) => (neighbour, garden_i + 1, garden_j),
                    Some(Wrapping::Left) => (neighbour, garden_i, garden_j - 1),
                    Some(Wrapping::Right) => (neighbour, garden_i, garden_j + 1),
                    None => (neighbour, garden_i, garden_j),
                })
            })
            .collect();
        reachable.sort_unstable();
        reachable.dedup();
    }
    Ok(reachable.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE, 6).unwrap(), 16);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2(EXAMPLE, 6).unwrap(), 16);
        assert_eq!(part2(EXAMPLE, 10).unwrap(), 50);
        assert_eq!(part2(EXAMPLE, 50).unwrap(), 1594);
        assert_eq!(part2(EXAMPLE, 100).unwrap(), 6536);
        // assert_eq!(part2(EXAMPLE, 500).unwrap(), 167004);
        // assert_eq!(part2(EXAMPLE, 1000).unwrap(), 668697);
        // assert_eq!(part2(EXAMPLE, 5000).unwrap(), 16733044);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day21.txt").unwrap();
        assert_eq!(part1(&input, 64).unwrap(), 3748);
        // assert_eq!(part2(&input, 26501365).unwrap(), 0);
    }
}
