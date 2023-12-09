use anyhow::Result;

fn differences(sequence: &[isize]) -> Vec<isize> {
    sequence.iter().zip(sequence.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn next_value(sequence: &[isize]) -> Option<isize> {
    if sequence.iter().all(|&x| x == 0) {
        return Some(0);
    }
    Some(sequence.last()? + next_value(&differences(sequence))?)
}

fn previous_value(sequence: &[isize]) -> Option<isize> {
    if sequence.iter().all(|&x| x == 0) {
        return Some(0);
    }
    Some(sequence.first()? - previous_value(&differences(sequence))?)
}

fn parse_input(input: &str) -> Result<Vec<Vec<isize>>> {
    let mut sequences: Vec<Vec<isize>> = Vec::new();
    for input_line in input.lines() {
        let mut sequence: Vec<isize> = Vec::new();
        for value in input_line.split_ascii_whitespace() {
            sequence.push(value.parse()?);
        }
        sequences.push(sequence);
    }
    Ok(sequences)
}

pub fn part1(input: &str) -> Result<isize> {
    Ok(parse_input(input)?.into_iter().map(|s| next_value(&s).unwrap()).sum())
}

pub fn part2(input: &str) -> Result<isize> {
    Ok(parse_input(input)?.into_iter().map(|s| previous_value(&s).unwrap()).sum())
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
        assert_eq!(differences(&[0, 3, 6, 9, 12, 15]), [3;5]);
        assert_eq!(differences(&differences(&[0, 3, 6, 9, 12, 15])), [0;4]);
    }

    #[test]
    fn test_next_value() {
        assert_eq!(next_value(&[0, 3, 6, 9, 12, 15]), Some(18));
        assert_eq!(next_value(&[1, 3, 6, 10, 15, 21]), Some(28));
        assert_eq!(next_value(&[10, 13, 16, 21, 30, 45]), Some(68));
    }

    #[test]
    fn test_previous_value() {
        assert_eq!(previous_value(&[0, 3, 6, 9, 12, 15]), Some(-3));
        assert_eq!(previous_value(&[1, 3, 6, 10, 15, 21]), Some(0));
        assert_eq!(previous_value(&[10, 13, 16, 21, 30, 45]), Some(5));
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 114);
        assert_eq!(part2(EXAMPLE).unwrap(), 2);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 1637452029);
        assert_eq!(part2(&input).unwrap(), 908);
    }
}
