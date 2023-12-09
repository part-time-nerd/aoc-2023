use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Cannot parse an instruction from {value}"),
        }
    }
}

type Edge = (usize, usize); // left and right

#[derive(Default, Debug)]
struct Graph {
    start: usize,
    end: usize,
    edges: Vec<Edge>,
}

impl FromStr for Graph {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut nodes: Vec<&str> = Vec::new();
        let mut label_edges: Vec<(&str, &str)> = Vec::new();

        for l in input.lines() {
            let (node, raw_edge) = l.split_once(" = ").context("Could not split into node and edge")?;
            let edge = raw_edge
                .strip_prefix('(')
                .context("Expected '('")?
                .strip_suffix(')')
                .context("Expected ')'")?
                .split_once(", ")
                .context("Could not split edge into left and right parts")?;
            nodes.push(node);
            label_edges.push(edge);
        }

        let mut edges: Vec<Edge> = Vec::new();
        for (left, right) in label_edges {
            let left_idx = nodes.iter().position(|&n| n == left).context("Unable to find left edge index")?;
            let right_idx = nodes.iter().position(|&n| n == right).context("Unable to find left edge index")?;
            edges.push((left_idx, right_idx))
        }

        let start = nodes.iter().position(|&n| n == "AAA").context("Could not find starting node")?;
        let end = nodes.iter().position(|&n| n == "ZZZ").context("Could not find ending node")?;

        Ok(Self { start, end, edges })
    }
}

fn parse_input(input: &str) -> Result<(Vec<Instruction>, Graph)> {
    let (instructions, graph) = input.split_once("\n\n").context("Could not split instructions and graph")?;
    Ok((instructions.chars().map(|c| c.into()).collect(), graph.parse()?))
}

pub fn part1(input: &str) -> Result<usize> {
    let (instructions, graph) = parse_input(input)?;
    let mut posn = graph.start;

    for step in 0.. {
        if posn == graph.end {
            return Ok(step);
        }
        posn = match instructions[step % instructions.len()] {
            Instruction::Left => graph.edges[posn].0,
            Instruction::Right => graph.edges[posn].1,
        };
    }
    Err(anyhow!("Reached end of iteration (max usize) without finding the end of the graph"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const EXAMPLE_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 2);
        assert_eq!(part1(EXAMPLE_2).unwrap(), 6);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 21389);
    }
}
