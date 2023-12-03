type Posn = (usize, usize);

struct Number {
    value: u32,
    start: Posn,
    length: usize,
}

impl Number {
    fn is_adjacent(&self, posn: &Posn) -> bool {
        self.start.0.abs_diff(posn.0) <= 1
            && posn.1 <= self.start.1 + self.length
            && (self.start.1 == 0 || posn.1 >= self.start.1 - 1)
    }
}

struct Symbol {
    value: char,
    posn: Posn,
}

fn get_numbers(matrix: &[Vec<char>]) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        let mut current_start: Option<usize> = None;
        let mut current_digits: String = String::new();
        for (j, &val) in row.iter().enumerate() {
            if val.is_ascii_digit() {
                if current_start.is_none() {
                    current_start = Some(j);
                }
                current_digits.push(val);
            } else if let Some(start_column) = current_start {
                numbers.push(Number {
                    value: current_digits.parse().unwrap(),
                    start: (i, start_column),
                    length: current_digits.len(),
                });
                current_start = None;
                current_digits = String::new();
            }
        }

        // Deal with digits at the end of the line
        if let Some(start_column) = current_start {
            numbers.push(Number {
                value: current_digits.parse().unwrap(),
                start: (i, start_column),
                length: current_digits.len(),
            });
        }
    }

    numbers
}

fn get_symbols(matrix: &[Vec<char>]) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                symbols.push(Symbol { value: c, posn: (i, j) });
            }
        }
    }
    symbols
}

pub fn part1(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let numbers = get_numbers(&matrix);
    let symbols = get_symbols(&matrix);
    numbers.into_iter().filter(|n| symbols.iter().any(|s| n.is_adjacent(&s.posn))).map(|n| n.value).sum()
}

pub fn part2(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let numbers = get_numbers(&matrix);
    let symbols = get_symbols(&matrix);

    let mut gear_ratio_sum = 0;
    for symbol in symbols {
        if symbol.value != '*' {
            continue; // This is not a gear
        }
        let adjacent_numbers: Vec<&Number> = numbers.iter().filter(|n| n.is_adjacent(&symbol.posn)).collect();
        if let [first, second] = adjacent_numbers[..] {
            gear_ratio_sum += first.value * second.value;
        } // Any other amount of adjacent numbers implies this is not a gear
    }
    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE), 4361);
        assert_eq!(part2(EXAMPLE), 467835);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
        assert_eq!(part1(&input), 507214);
        assert_eq!(part2(&input), 72553319);
    }
}
