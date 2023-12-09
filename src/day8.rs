use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;

#[derive(Eq, PartialEq)]
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

struct GraphP1 {
    start: usize,
    end: usize,
    edges: Vec<(usize, usize)>,
}

impl FromStr for GraphP1 {
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

        let mut edges: Vec<(usize, usize)> = Vec::new();
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

fn solve_p1(instructions: &[Instruction], graph: &GraphP1) -> usize {
    let mut posn = graph.start;
    for step in 0.. {
        if posn == graph.end {
            return step;
        }
        posn = match instructions[step % instructions.len()] {
            Instruction::Left => graph.edges[posn].0,
            Instruction::Right => graph.edges[posn].1,
        };
    }
    panic!("Reached end of iteration (usize max) without finding the end of the graph");
}

pub fn part1(input: &str) -> Result<usize> {
    let (i, g) = input.split_once("\n\n").context("Could not split instructions and graph")?;
    let (instructions, graph): (Vec<Instruction>, GraphP1) = (i.chars().map(|c| c.into()).collect(), g.parse()?);
    Ok(solve_p1(&instructions, &graph))
}

struct GraphP2 {
    start: Vec<usize>,
    end: Vec<usize>,
    edges: Vec<(usize, usize)>,
}

impl FromStr for GraphP2 {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        // This first chunk is copied from P1
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

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for (left, right) in label_edges {
            let left_idx = nodes.iter().position(|&n| n == left).context("Unable to find left edge index")?;
            let right_idx = nodes.iter().position(|&n| n == right).context("Unable to find left edge index")?;
            edges.push((left_idx, right_idx))
        }

        // This part is different from P1
        let start: Vec<usize> = nodes.iter().enumerate().filter(|(_, n)| n.ends_with('A')).map(|(i, _)| i).collect();
        let mut end: Vec<usize> = nodes.iter().enumerate().filter(|(_, n)| n.ends_with('Z')).map(|(i, _)| i).collect();
        assert_eq!(start.len(), end.len(), "There should be the same number of starting and ending nodes");

        // Sort the end so the comparisons can be faster
        end.sort();

        Ok(Self { start, end, edges })
    }
}

pub fn part2(input: &str) -> Result<usize> {
    let (i, g) = input.split_once("\n\n").context("Could not split instructions and graph")?;
    let (instructions, graph): (Vec<Instruction>, GraphP2) = (i.chars().map(|c| c.into()).collect(), g.parse()?);

    let mut posn = graph.start;
    for step in 0.. {
        posn.sort(); // The ending positions are also sorted
        if posn == graph.end {
            return Ok(step);
        }
        posn = posn
            .into_iter()
            .map(|p| match instructions[step % instructions.len()] {
                Instruction::Left => graph.edges[p].0,
                Instruction::Right => graph.edges[p].1,
            })
            .collect();
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

    const EXAMPLE_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_example_p1() {
        assert_eq!(part1(EXAMPLE).unwrap(), 2);
        assert_eq!(part1(EXAMPLE_2).unwrap(), 6);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(part2(EXAMPLE_3).unwrap(), 6);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 21389);
    }
}
