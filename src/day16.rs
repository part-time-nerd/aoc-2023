use std::{str::FromStr, collections::HashSet};

use anyhow::{bail, Error, Result};

enum Tile {
    Empty,
    MirrorFwd,
    MirrorBack,
    SplitterVert,
    SplitterHori,
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '.' => Self::Empty,
            '/' => Self::MirrorFwd,
            '\\' => Self::MirrorBack,
            '|' => Self::SplitterVert,
            '-' => Self::SplitterHori,
            _ => bail!("Expected a valid tile character (., /, \\, |, -), got {value}"),
        })
    }
}

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    #[default]
    East,
    West,
}

type Posn = (usize, usize);

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone)]
struct Beam {
    position: Posn,
    direction: Direction,
}

struct Contraption(Vec<Vec<Tile>>);

impl FromStr for Contraption {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut contraption: Vec<Vec<Tile>> = Vec::new();
        for l in input.lines() {
            let mut row: Vec<Tile> = Vec::new();
            for c in l.chars() {
                row.push(c.try_into()?);
            }
            contraption.push(row);
        }
        Ok(Self(contraption))
    }
}

fn try_move(posn: &Posn, direction: &Direction, contraption_dimensions: &Posn) -> Option<Posn> {
    match direction {
        Direction::North => {
            if posn.0 == 0 {
                None
            } else {
                Some((posn.0 - 1, posn.1))
            }
        }
        Direction::South => {
            if posn.0 < contraption_dimensions.0 - 1 {
                Some((posn.0 + 1, posn.1))
            } else {
                None
            }
        }
        Direction::East => {
            if posn.1 < contraption_dimensions.1 - 1 {
                Some((posn.0, posn.1 + 1))
            } else {
                None
            }
        }
        Direction::West => {
            if posn.1 == 0 {
                None
            } else {
                Some((posn.0, posn.1 - 1))
            }
        }
    }
}

impl Contraption {
    fn energized_tiles(&self, start: Beam) -> Vec<Vec<bool>> {
        let mut energized_tiles: Vec<Vec<bool>> = self.0.iter().map(|r| r.iter().map(|_| false).collect()).collect();
        let mut beams: Vec<Beam> = vec![start];
        let mut beam_cache: HashSet<Beam> = HashSet::default();
        beam_cache.insert(start.clone());
        let contraption_dimensions: Posn = (self.0.len(), self.0[0].len());
        while !beams.is_empty() {
            let mut new_beams: Vec<Beam> = Vec::new();
            for mut beam in beams {
                energized_tiles[beam.position.0][beam.position.1] = true;
                match self.0[beam.position.0][beam.position.1] {
                    Tile::Empty => {
                        if let Some(position) = try_move(&beam.position, &beam.direction, &contraption_dimensions) {
                            beam.position = position;
                            new_beams.push(beam);
                        }
                    }
                    Tile::MirrorFwd => {
                        beam.direction = match beam.direction {
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                            Direction::East => Direction::North,
                            Direction::West => Direction::South,
                        };
                        if let Some(position) = try_move(&beam.position, &beam.direction, &contraption_dimensions) {
                            beam.position = position;
                            new_beams.push(beam);
                        }
                    }
                    Tile::MirrorBack => {
                        beam.direction = match beam.direction {
                            Direction::North => Direction::West,
                            Direction::South => Direction::East,
                            Direction::East => Direction::South,
                            Direction::West => Direction::North,
                        };
                        if let Some(position) = try_move(&beam.position, &beam.direction, &contraption_dimensions) {
                            beam.position = position;
                            new_beams.push(beam);
                        }
                    }
                    Tile::SplitterVert => {
                        match beam.direction {
                            Direction::North | Direction::South => {
                                if let Some(position) =
                                    try_move(&beam.position, &beam.direction, &contraption_dimensions)
                                {
                                    beam.position = position;
                                    new_beams.push(beam);
                                }
                            }
                            Direction::East | Direction::West => {
                                for direction in [Direction::North, Direction::South] {
                                    if let Some(position) =
                                        try_move(&beam.position, &direction, &contraption_dimensions)
                                    {
                                        new_beams.push(Beam { position, direction });
                                    }
                                }
                            }
                        };
                    }
                    Tile::SplitterHori => {
                        match beam.direction {
                            Direction::East | Direction::West => {
                                if let Some(position) =
                                    try_move(&beam.position, &beam.direction, &contraption_dimensions)
                                {
                                    beam.position = position;
                                    new_beams.push(beam);
                                }
                            }
                            Direction::North | Direction::South => {
                                for direction in [Direction::East, Direction::West] {
                                    if let Some(position) =
                                        try_move(&beam.position, &direction, &contraption_dimensions)
                                    {
                                        new_beams.push(Beam { position, direction });
                                    }
                                }
                            }
                        };
                    }
                }
            }
            beams = Vec::new();
            for beam in new_beams {
                if beam_cache.insert(beam.clone()) {
                    beams.push(beam);
                }
            }
        }

        energized_tiles
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let contraption: Contraption = input.parse()?;
    Ok(contraption.energized_tiles(Beam::default()).iter().map(|r| r.iter().filter(|&&x| x).count()).sum())
}

pub fn part2(input: &str) -> Result<usize> {
    let contraption: Contraption = input.parse()?;
    let mut current_max_energized = 0;
    for i in 0..contraption.0.len() {
        let start = Beam{direction: Direction::East, position: (i, 0)};
        let energized: usize = contraption.energized_tiles(start).iter().map(|r| r.iter().filter(|&&x| x).count()).sum();
        current_max_energized = energized.max(current_max_energized);
        
        let start = Beam{direction: Direction::West, position: (i, contraption.0[0].len() - 1)};
        let energized: usize = contraption.energized_tiles(start).iter().map(|r| r.iter().filter(|&&x| x).count()).sum();
        current_max_energized = energized.max(current_max_energized);
    }

    
    for j in 0..contraption.0[0].len() {
        let start = Beam{direction: Direction::South, position: (0, j)};
        let energized: usize = contraption.energized_tiles(start).iter().map(|r| r.iter().filter(|&&x| x).count()).sum();
        current_max_energized = energized.max(current_max_energized);
        
        let start = Beam{direction: Direction::North, position: (contraption.0.len() - 1, j)};
        let energized: usize = contraption.energized_tiles(start).iter().map(|r| r.iter().filter(|&&x| x).count()).sum();
        current_max_energized = energized.max(current_max_energized);
    }
    Ok(current_max_energized)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 46);
        assert_eq!(part2(EXAMPLE).unwrap(), 51);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 7415);
    }
}
