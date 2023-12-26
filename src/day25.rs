use anyhow::{bail, Context, Error, Result};
use pathfinding::prelude::connected_components;
use pathfinding::prelude::edmonds_karp_dense;
use std::collections::HashMap;
use std::{collections::HashSet, str::FromStr};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Label(char, char, char);

impl FromStr for Label {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars();

        let first = chars.next().context("No first character of label")?;
        let second = chars.next().context("No second character of label")?;
        let third = chars.next().context("No third character of label")?;

        if chars.next().is_some() {
            bail!("Expected only three characters in the label")
        }
        if !first.is_ascii_alphabetic() {
            bail!("Expected first character '{first}' to be ascii alphabetic");
        }
        if !second.is_ascii_alphabetic() {
            bail!("Expected second character '{second}' to be ascii alphabetic");
        }
        if !third.is_ascii_alphabetic() {
            bail!("Expected third character '{third}' to be ascii alphabetic");
        }

        Ok(Self(first, second, third))
    }
}

type Vertex = Label;
type Edge = (Vertex, Vertex);

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let mut edges: Vec<Edge> = Vec::new();
        for l in input.lines() {
            if l.is_empty() {
                continue;
            }
            let (n, es) = l.split_once(": ").context("Could not split vertex from its edges")?;
            let vertex: Vertex = n.parse()?;
            for e in es.split_ascii_whitespace() {
                let other: Vertex = e.parse()?;
                edges.push((vertex, other));
                edges.push((other, vertex));
            }
        }
        edges.sort();
        edges.dedup();

        let mut vertices: Vec<Vertex> = Vec::new();
        for &(a, b) in edges.iter() {
            vertices.push(a);
            vertices.push(b);
        }
        vertices.sort();
        vertices.dedup();

        Ok(Self { vertices, edges })
    }
}

impl Graph {
    fn part1_solution(&self) -> usize {
        // https://en.wikipedia.org/wiki/Minimum_cut
        // Apparently, we can use Karger's Algorithm since the graph is unweighted
        // https://en.wikipedia.org/wiki/Karger%27s_algorithm
        // But this is a random algorithm and can fail
        // We could always just repeat until we end with a cut of size 3
        //
        // Stoer–Wagner might be a better option (and also works with weights if needed)
        // https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
        //
        // What we are instead going to do:
        // Chose an arbitrary vertex
        // For each other vertex, find the max flow / min cut using Edmonds-Carp
        // Once the min cut is 3, stop and return

        for sink in self.vertices.iter().skip(1) {
            let capacities: Vec<(Edge, isize)> = self.edges.iter().map(|&e| (e, 1)).collect();
            let (_flows, _capacity, min_cut) = edmonds_karp_dense(&self.vertices, &self.vertices[0], sink, capacities);
            if min_cut.len() == 3 {
                // We have found the minimum cut: we just need to find the two vertex sets that result from it
                // Any edge that is not in the min cut will be in either one vertex set or the other
                let min_cut_edges: Vec<Edge> = min_cut.into_iter().flat_map(|((a, b), _)| [(a, b), (b, a)]).collect();

                // There must be a more efficient way to do this, but we will just create a map of neighbours and use that
                // to compute connected components using the pathfinding library.
                let mut neighbours: HashMap<Vertex, Vec<Vertex>> = HashMap::default();
                for &vertex in self.vertices.iter() {
                    neighbours.insert(vertex, vec![]);
                }
                for edge in self.edges.iter() {
                    if min_cut_edges.contains(edge) {
                        continue; // This is part of the cut
                    }
                    neighbours.get_mut(&edge.0).unwrap().push(edge.1);
                }
                let components = connected_components(&self.vertices, |v| neighbours.get(v).unwrap().clone());
                return components[0].len() * components[1].len();
            }
        }
        panic!("did not find the min cut");
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let graph: Graph = input.parse()?;
    Ok(graph.part1_solution())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 54);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day25.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 527790);
    }
}
