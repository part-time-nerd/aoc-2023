use anyhow::{Context, Result};

fn holiday_ascii_string_helper(s: &str) -> usize {
    s.chars().fold(0, |h, c| ((h + c as usize) * 17) % 256)
}

pub fn part1(input: &str) -> usize {
    input.trim().split(',').map(holiday_ascii_string_helper).sum()
}

type Lens<'a> = (&'a str, usize);

pub fn part2(input: &str) -> Result<usize> {
    // Initialize the array of lenses
    let mut lenses: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        lenses.push(Vec::new());
    }

    'instructions: for instruction in input.trim().split(',') {
        if let Some(label) = instruction.strip_suffix('-') {
            let bucket = holiday_ascii_string_helper(label);
            lenses[bucket].retain(|x| x.0 != label);
        } else {
            let (label, value) = instruction.split_once('=').context("could not split on =")?;
            let focal_length: usize = value.parse()?;
            let bucket = holiday_ascii_string_helper(label);
            for l in lenses[bucket].iter_mut() {
                if l.0 == label {
                    l.1 = focal_length;
                    continue 'instructions;
                }
            }
            lenses[bucket].push((label, focal_length));
        }
    }

    let mut total = 0;
    for (i, l) in lenses.into_iter().enumerate() {
        for (j, (_, f)) in l.into_iter().enumerate() {
            total += (i + 1) * (j + 1) * f;
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 1320);
        assert_eq!(part2(EXAMPLE).unwrap(), 145);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
        assert_eq!(part1(&input), 515974);
        assert_eq!(part2(&input).unwrap(), 265894);
    }
}
