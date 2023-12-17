use anyhow::{Context, Error, Result};
use pathfinding::prelude::dijkstra;
use std::str::FromStr;

struct City(Vec<Vec<usize>>);

impl FromStr for City {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut map: Vec<Vec<usize>> = Vec::new();
        for l in s.lines() {
            let mut row: Vec<usize> = Vec::new();
            for c in l.chars() {
                row.push(c.to_digit(10).context("Not a number")? as usize);
            }
            map.push(row);
        }
        Ok(Self(map))
    }
}

type Posn = (isize, isize);

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    #[default]
    East,
    West,
}

impl From<Direction> for Posn {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

fn posn_add((a, b): Posn, (c, d): Posn) -> Posn {
    (a + c, b + d)
}

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy)]
struct Crucible {
    direction: Direction,
    moves_in_direction: u8,
    position: Posn,
}

impl Crucible {
    fn cost(&self, map: &City) -> usize {
        map.0[self.position.0 as usize][self.position.1 as usize]
    }

    fn is_valid(&self, map: &City) -> bool {
        self.position.0 >= 0
            && self.position.1 >= 0
            && self.position.0 < map.0.len() as isize
            && self.position.1 < map.0[0].len() as isize
    }

    fn neighbours(&self, map: &City) -> Vec<(Crucible, usize)> {
        let mut neighbours: Vec<(Crucible, usize)> = Vec::new();

        let next_directions: [Direction; 2] = match self.direction {
            Direction::East | Direction::West => [Direction::North, Direction::South],
            Direction::North | Direction::South => [Direction::East, Direction::West],
        };
        for direction in next_directions {
            let neighbour =
                Crucible { direction, moves_in_direction: 1, position: posn_add(self.position, direction.into()) };
            if neighbour.is_valid(map) {
                neighbours.push((neighbour, neighbour.cost(map)));
            }
        }

        if self.moves_in_direction < 3 {
            // We can continue to move forward as well
            let neighbour = Crucible {
                direction: self.direction,
                moves_in_direction: self.moves_in_direction + 1,
                position: posn_add(self.position, self.direction.into()),
            };
            if neighbour.is_valid(map) {
                neighbours.push((neighbour, neighbour.cost(map)));
            }
        }

        neighbours
    }

    fn ultra_neighbours(&self, map: &City) -> Vec<(Crucible, usize)> {
        let mut neighbours: Vec<(Crucible, usize)> = Vec::new();

        if self.moves_in_direction < 10 {
            let neighbour = Crucible {
                direction: self.direction,
                moves_in_direction: self.moves_in_direction + 1,
                position: posn_add(self.position, self.direction.into()),
            };
            if neighbour.is_valid(map) {
                neighbours.push((neighbour, neighbour.cost(map)));
            }
        }

        if self.moves_in_direction >= 4 || self.moves_in_direction == 0 {
            let next_directions: [Direction; 2] = match self.direction {
                Direction::East | Direction::West => [Direction::North, Direction::South],
                Direction::North | Direction::South => [Direction::East, Direction::West],
            };
            for direction in next_directions {
                let neighbour =
                    Crucible { direction, moves_in_direction: 1, position: posn_add(self.position, direction.into()) };
                if neighbour.is_valid(map) {
                    neighbours.push((neighbour, neighbour.cost(map)));
                }
            }
        }

        neighbours
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let map: City = input.parse()?;
    let end: Posn = (map.0.len() as isize - 1, map.0[0].len() as isize - 1);
    let (_path, cost) = dijkstra(&Crucible::default(), |c| c.neighbours(&map), |c| c.position == end)
        .context("Could not find a path through the city using dijkstra's algorithm.")?;
    Ok(cost)
}

pub fn part2(input: &str) -> Result<usize> {
    let map: City = input.parse()?;
    let end: Posn = (map.0.len() as isize - 1, map.0[0].len() as isize - 1);
    let (_path, cost) = dijkstra(
        &Crucible::default(),
        |c| c.ultra_neighbours(&map),
        |c| c.position == end && c.moves_in_direction >= 4,
    )
    .context("Could not find a path through the city using dijkstra's algorithm.")?;
    Ok(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const EXAMPLE2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 102);
        assert_eq!(part2(EXAMPLE).unwrap(), 94);
        assert_eq!(part2(EXAMPLE2).unwrap(), 71);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 870);
        assert_eq!(part2(&input).unwrap(), 1063);
    }
}
