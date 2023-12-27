use std::{str::FromStr, collections::HashSet};

use anyhow::{anyhow, Context, Error, Result};

#[derive(Eq, PartialEq, Clone, Hash)]
struct Posn {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Posn {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        if let [x, y, z] = input.split(',').collect::<Vec<_>>()[..] {
            Ok(Self { x: x.parse()?, y: y.parse()?, z: z.parse()? })
        } else {
            Err(anyhow!("Could not split the input into 3 comma separated parts: {input}"))
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Brick(Posn, Posn);

impl FromStr for Brick {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let (a, b) = input.split_once('~').context("Could not split the input into tilde separated posns")?;
        Ok(Self(a.parse()?, b.parse()?))
    }
}

impl Brick {
    fn all(&self) -> Vec<Posn> {
        let mut all: Vec<Posn> = Vec::new();

        // We assume only one axis is relevant, and that self.0 has the smaller number
        if self.0.x != self.1.x {
            for x in self.0.x..=self.1.x {
                all.push(Posn { x, y: self.0.y, z: self.0.z });
            }
        } else if self.0.y != self.1.y {
            for y in self.0.y..=self.1.y {
                all.push(Posn { x: self.0.x, y, z: self.0.z });
            }
        } else {
            for z in self.0.z..=self.1.z {
                all.push(Posn { x: self.0.x, y: self.0.y, z });
            }
        }

        all
    }

    fn is_grounded(&self) -> bool {
        self.0.z == 0
    }
}

fn settle(bricks: &mut [Brick]) -> HashSet<Posn> {
    bricks.sort_by(|a, b| a.0.z.cmp(&b.0.z));

    let mut settled_cover: HashSet<Posn> = HashSet::default();

    for brick in bricks.iter_mut() {
        loop {
            if brick.is_grounded() {
                break;
            }

            brick.0.z -= 1;
            brick.1.z -= 1;

            if brick.all().iter().any(|p| settled_cover.contains(p)) {
                brick.0.z += 1;
                brick.1.z += 1;
                break;
            }

        }

        for p in brick.all() {
            settled_cover.insert(p);
        }
    }

    settled_cover
}

pub fn part1(input: &str) -> Result<usize> {
    let mut bricks: Vec<Brick> = Vec::new();
    for l in input.lines() {
        bricks.push(l.parse()?);
    }
    let cover = settle(&mut bricks);

    // 10s, not great
    let mut could_disintegrate = 0;
    for i in 0..bricks.len() {
        let mut possibly_settled = bricks.clone();
        possibly_settled.remove(i);
        let mut new_cover = settle(&mut possibly_settled);
        for p in bricks[i].all() {
            new_cover.insert(p);
        }
        if new_cover == cover {
            could_disintegrate += 1;
        }
    }

    Ok(could_disintegrate)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 5);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day22.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 507);
    }
}
