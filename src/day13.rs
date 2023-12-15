use anyhow::Result;

type Matrix<T> = Vec<Vec<T>>;

fn parse_input(input: &str) -> Vec<Matrix<bool>> {
    input.split("\n\n").map(|b| b.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect()).collect()
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_example() {
        // assert_eq!(part1(EXAMPLE).unwrap(), 405);
    }
}
