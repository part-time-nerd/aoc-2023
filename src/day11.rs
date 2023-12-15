type Posn = (usize, usize);

fn parse_galaxies(input: &str, expansion_rate: usize) -> Vec<Posn> {
    let mtx: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let empty_rows: Vec<usize> = mtx.iter().enumerate().filter(|(_, l)| !l.contains(&'#')).map(|(i, _)| i).collect();
    let empty_cols: Vec<usize> = (0..mtx[0].len()).filter(|j| mtx.iter().all(|r| r[*j] == '.')).collect();

    let mut galaxies: Vec<Posn> = Vec::new();
    for (i, row) in input.lines().enumerate() {
        for (j, c) in row.char_indices() {
            if c != '#' {
                continue;
            }
            // Could do this in a more efficient way but this is good enough
            let expanded_i = i + empty_rows.iter().filter(|&&r| r < i).count() * (expansion_rate - 1);
            let expanded_j = j + empty_cols.iter().filter(|&&c| c < j).count() * (expansion_rate - 1);
            galaxies.push((expanded_i, expanded_j));
        }
    }

    galaxies
}

fn manhatten(p1: &Posn, p2: &Posn) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn total_distance(galaxies: &[Posn]) -> usize {
    let mut total_distance = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(i + 1) {
            total_distance += manhatten(g1, g2);
        }
    }
    total_distance
}

pub fn part1(input: &str) -> usize {
    total_distance(&parse_galaxies(input, 2))
}

pub fn part2(input: &str) -> usize {
    total_distance(&parse_galaxies(input, 1000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_galaxy_dist() {
        let galaxies = parse_galaxies(EXAMPLE, 2);
        assert_eq!(manhatten(&galaxies[0], &galaxies[6]), 15);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 374);
        assert_eq!(total_distance(&parse_galaxies(EXAMPLE, 10)), 1030);
        assert_eq!(total_distance(&parse_galaxies(EXAMPLE, 100)), 8410);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
        assert_eq!(part1(&input), 9805264);
        assert_eq!(part2(&input), 779032247216);
    }
}
