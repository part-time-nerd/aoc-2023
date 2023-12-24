use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{bail, Context, Error, Result};

#[derive(Debug)]
struct Vec3(f64, f64, f64);

impl FromStr for Vec3 {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let [x, y, z] = input.split(',').collect::<Vec<&str>>()[..] else {
            bail!("Expected 3 comma separated components from {input}");
        };
        Ok(Self(x.trim().parse()?, y.trim().parse()?, z.trim().parse()?))
    }
}

#[derive(Debug)]
struct HailStone {
    position: Vec3,
    velocity: Vec3,
}

impl FromStr for HailStone {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let (posn, vel) = input.split_once('@').context("Expected hailstone to be @-separated")?;
        Ok(Self { position: posn.parse()?, velocity: vel.parse()? })
    }
}

enum LinearCollision {
    None,                     // Parallel lines
    Point { x: f64, y: f64 }, // Non-parallel non-identical lines
    Line { m: f64, b: f64 },  // identical lines
}

fn float_eq(a: f64, b: f64) -> bool {
    const TOLERANCE: f64 = 1e-10;
    (a - b).abs() < TOLERANCE
}

impl HailStone {
    fn collides_xy(&self, other: &HailStone) -> LinearCollision {
        // Using the standard y = mx + b formulation of the line
        let m = self.velocity.1 / self.velocity.0;
        let b = self.position.1 - m * self.position.0;

        let m2 = other.velocity.1 / other.velocity.0;
        let b2 = other.position.1 - m2 * other.position.0;

        if float_eq(m, m2) {
            if float_eq(b, b2) {
                return LinearCollision::Line { m, b };
            } else {
                return LinearCollision::None;
            }
        }
        let x = (b - b2) / (m2 - m);
        // Both y should be identical if we have done this correctly
        let y = m * x + b;
        // let y2 = m2 * x + b2;
        // assert!(float_eq(y, y2), "{y} != {y2}");
        LinearCollision::Point { x, y }
    }

    fn is_in_future(&self, x: f64) -> bool {
        if x > self.position.0 {
            self.velocity.0 > 0.0
        } else if x < self.position.0 {
            self.velocity.0 < 0.0
        } else {
            todo!()
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<HailStone>> {
    let mut hailstones: Vec<HailStone> = Vec::new();
    for l in input.lines() {
        hailstones.push(l.parse()?);
    }
    Ok(hailstones)
}

pub fn part1(input: &str, test_area: RangeInclusive<f64>) -> Result<usize> {
    let hailstones = parse_input(input)?;
    let mut total_collisions = 0;
    for (i, hs) in hailstones.iter().enumerate() {
        for other in hailstones.iter().skip(i + 1) {
            match hs.collides_xy(other) {
                LinearCollision::None => {}
                LinearCollision::Line { m, b } => todo!(),
                LinearCollision::Point { x, y } => {
                    if test_area.contains(&x) && test_area.contains(&y) && hs.is_in_future(x) && other.is_in_future(x) {
                        total_collisions += 1;
                    }
                }
            }
        }
    }
    Ok(total_collisions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE, 7.0..=27.0).unwrap(), 2);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day24.txt").unwrap();
        assert_eq!(part1(&input, 2e14..=4e14).unwrap(), 16812)
    }
}
