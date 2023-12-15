use std::collections::HashSet;

use anyhow::{bail, Context, Error, Result};
use pathfinding::directed::dijkstra::dijkstra_all;

type Posn = (usize, usize);
type Edge = (Posn, Posn);

#[derive(PartialEq, Eq)]
enum Tile {
    Vert,
    Hori,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl Tile {
    fn is_connected_east(&self) -> bool {
        [Self::Hori, Self::NE, Self::SE, Self::Start].contains(self)
    }
    fn is_connected_west(&self) -> bool {
        [Self::Hori, Self::NW, Self::SW, Self::Start].contains(self)
    }
    fn is_connected_north(&self) -> bool {
        [Self::Vert, Self::NE, Self::NW, Self::Start].contains(self)
    }
    fn is_connected_south(&self) -> bool {
        [Self::Vert, Self::SE, Self::SW, Self::Start].contains(self)
    }
}

impl TryFrom<char> for Tile {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '|' => Self::Vert,
            '-' => Self::Hori,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => bail!("{value} is not a tile"),
        })
    }
}

fn parse_input_matrix(input: &str) -> Result<Vec<Vec<Tile>>> {
    let mut matrix: Vec<Vec<Tile>> = Vec::new();
    for l in input.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in l.chars() {
            row.push(c.try_into()?);
        }
        matrix.push(row);
    }
    Ok(matrix)
}

fn matrix_start_posn(matrix: &[Vec<Tile>]) -> Option<Posn> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile == &Tile::Start {
                return Some((i, j));
            }
        }
    }
    None
}

fn matrix_adjacency(matrix: &[Vec<Tile>], posn: Posn) -> Vec<Posn> {
    let mut adjacent: Vec<Posn> = Vec::new();
    let (i, j) = posn;
    if matrix[i][j].is_connected_north() && i != 0 && matrix[i - 1][j].is_connected_south() {
        adjacent.push((i - 1, j));
    }
    if matrix[i][j].is_connected_east() && matrix[i].get(j + 1).is_some_and(|t| t.is_connected_west()) {
        adjacent.push((i, j + 1));
    }
    if matrix[i][j].is_connected_south() && matrix.get(i + 1).is_some_and(|r| r[j].is_connected_north()) {
        adjacent.push((i + 1, j));
    }
    if matrix[i][j].is_connected_west() && j != 0 && matrix[i][j - 1].is_connected_east() {
        adjacent.push((i, j - 1));
    }
    adjacent
}

pub fn part1(input: &str) -> Result<usize> {
    let matrix = parse_input_matrix(input)?;
    let start = matrix_start_posn(&matrix).context("Could not find the start position")?;
    let shortest_paths = dijkstra_all(&start, |&posn| matrix_adjacency(&matrix, posn).into_iter().map(|a| (a, 1)));
    shortest_paths.iter().map(|(_node, (_previous, length))| *length as usize).max().context("No paths from start")
}

enum Spin {
    Clockwise,
    CounterClockwise,
}

fn loop_direction(matrix: &[Vec<Tile>]) -> Result<(Posn, Spin)> {
    // Starting at start and travelling to next, what is the spin direction
    let start = matrix_start_posn(&matrix).context("Could not find the start position")?;
    let next_to_start = matrix_adjacency(&matrix, start);
    let first_step = next_to_start[0]; // This is arbitrary: next_to_start[1] would give oposite spin
    todo!();
    let spin = Spin::Clockwise;
    Ok((first_step, spin))
}

pub fn part2(input: &str) -> Result<usize> {
    let matrix = parse_input_matrix(input)?;

    let inside_tiles: HashSet<Posn> = HashSet::new();

    let start = matrix_start_posn(&matrix).context("Could not find the start position")?;
    let next_to_start = matrix_adjacency(&matrix, start);

    let mut current_loop_direction: Option<Spin> = None;

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE_WITH_CRUD: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const COMPLEX_EXAMPLE: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    const COMPLEX_EXAMPLE_WITH_CRUD: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const P2_EXAMPLE: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const LARGER_EXAMPLE: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const LARGER_EXAMPLE_WITH_CRUD: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_example_part1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 4);
        assert_eq!(part1(EXAMPLE_WITH_CRUD).unwrap(), 4);
        assert_eq!(part1(COMPLEX_EXAMPLE).unwrap(), 8);
        assert_eq!(part1(COMPLEX_EXAMPLE_WITH_CRUD).unwrap(), 8);
    }

    #[test]
    fn test_example_part2() {
        // assert_eq!(part2(P2_EXAMPLE).unwrap(), 4);
        // assert_eq!(part2(LARGER_EXAMPLE).unwrap(), 10);
        // assert_eq!(part2(LARGER_EXAMPLE_WITH_CRUD).unwrap(), 10);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 6714);
    }
}
