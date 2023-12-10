fn differences(sequence: &[isize]) -> Vec<isize> {
    sequence.iter().zip(sequence.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn next_value(sequence: &[isize]) -> isize {
    if sequence.iter().all(|&x| x == 0) {
        return 0; // Note this also handles the empty case so the subsequent unwrap is safe
    }
    sequence.last().unwrap() + next_value(&differences(sequence))
}

fn previous_value(sequence: &[isize]) -> isize {
    if sequence.iter().all(|&x| x == 0) {
        return 0; // Note this also handles the empty case so the subsequent unwrap is safe
    }
    sequence.first().unwrap() - previous_value(&differences(sequence))
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|l| l.split_ascii_whitespace().flat_map(str::parse).collect()).collect()
}

pub fn part1(input: &str) -> isize {
    parse_input(input).into_iter().map(|s| next_value(&s)).sum()
}

pub fn part2(input: &str) -> isize {
    parse_input(input).into_iter().map(|s| previous_value(&s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_differences() {
        assert_eq!(differences(&[0, 3, 6, 9, 12, 15]), [3; 5]);
        assert_eq!(differences(&differences(&[0, 3, 6, 9, 12, 15])), [0; 4]);
    }

    #[test]
    fn test_next_value() {
        assert_eq!(next_value(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(next_value(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(next_value(&[10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(next_value(&[]), 0);
    }

    #[test]
    fn test_previous_value() {
        assert_eq!(previous_value(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(previous_value(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(previous_value(&[10, 13, 16, 21, 30, 45]), 5);
        assert_eq!(previous_value(&[]), 0);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 114);
        assert_eq!(part2(EXAMPLE), 2);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day09.txt").unwrap();
        assert_eq!(part1(&input), 1637452029);
        assert_eq!(part2(&input), 908);
    }
}
