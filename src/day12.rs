use anyhow::{anyhow, Context, Error, Result};

#[derive(Eq, PartialEq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = Error;
    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(anyhow!("value is not a valid spring condition: {value}")),
        }
    }
}

fn parse_spring_conditions(input: &str) -> Result<Vec<(Condition, usize)>> {
    let mut spring_conditions: Vec<(Condition, usize)> = Vec::new();
    let mut current_condition: Option<Condition> = None;
    let mut current_count: usize = 0;
    for condition in input.chars() {
        let condition: Condition = condition.try_into()?;
        match current_condition {
            Some(ref cc) if cc == &condition => current_count += 1,
            Some(cc) => {
                spring_conditions.push((cc, current_count));
                current_condition = Some(condition);
                current_count = 1;
            }
            None => {
                current_condition = Some(condition);
                current_count = 1;
            }
        }
    }
    if let Some(cc) = current_condition {
        spring_conditions.push((cc, current_count));
    }
    Ok(spring_conditions)
}

fn parse_damaged_info(input: &str) -> Result<Vec<usize>> {
    let mut damaged_info: Vec<usize> = Vec::new();
    for c in input.split(',') {
        damaged_info.push(c.parse()?);
    }
    Ok(damaged_info)
}

fn parse_input(input: &str) -> Result<Vec<(Vec<(Condition, usize)>, Vec<usize>)>> {
    let mut spring_info: Vec<(Vec<(Condition, usize)>, Vec<usize>)> = Vec::new();
    for l in input.lines() {
        let (conditions, damaged) = l.split_once(' ').context("Could not split line {l}")?;
        spring_info.push((parse_spring_conditions(conditions)?, parse_damaged_info(damaged)?));
    }
    Ok(spring_info)
}

fn is_valid_arrangment(conditions: &[(Condition, usize)], damaged: &[usize]) -> bool {
    let mut previous_condition: Option<&Condition> = None;
    let mut damaged_idx = 0;
    for (condition, size) in conditions {
        if *condition == Condition::Unknown || *size == 0 || previous_condition.is_some_and(|c| c == condition) {
            return false;
        }
        if *condition == Condition::Damaged {
            if damaged_idx >= damaged.len() || *size != damaged[damaged_idx] {
                return false;
            }
            damaged_idx += 1;
        }
        previous_condition = Some(condition);
    }
    damaged_idx == damaged.len()
}

fn possible_arrangements(conditions: &[(Condition, usize)], damaged: &[usize]) -> usize {
    todo!()
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.into_iter().map(|(c, d)| possible_arrangements(&c, &d)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_example() {
        // assert_eq!(part1(EXAMPLE).unwrap(), 21);
    }
}
