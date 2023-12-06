use anyhow::{anyhow, Context, Result};

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn distance_will_travel(&self, time_button_held: usize) -> usize {
        assert!(time_button_held <= self.time, "Cannot hold the button for longer than the race");
        (self.time - time_button_held) * time_button_held
    }

    fn brute_force_num_ways_to_win(&self) -> usize {
        (0..=self.time).filter(|&i| self.distance_will_travel(i) > self.dist).count()
    }
}

fn parse_input(input: &str) -> Result<Vec<Race>> {
    let mut lines = input.lines();
    let times = lines.next().context("Expected race times")?;
    let dists = lines.next().context("Expected race dists")?;
    if let Some(l) = lines.next() {
        return Err(anyhow!("Expected end of input, but found another line: {l}"));
    }

    let mut times_vec: Vec<usize> = Vec::new();
    let mut dists_vec: Vec<usize> = Vec::new();
    for time in times.strip_prefix("Time:").context("expected 'Time:' prefix")?.split_ascii_whitespace() {
        times_vec.push(time.parse()?);
    }
    for dist in dists.strip_prefix("Distance:").context("expected 'Distance:' prefix")?.split_ascii_whitespace() {
        dists_vec.push(dist.parse()?);
    }
    if times_vec.len() != dists_vec.len() {
        return Err(anyhow!("Expected equal number of times ({}) and dists ({})", times_vec.len(), dists_vec.len()));
    }

    Ok(times_vec.into_iter().zip(dists_vec).map(|(time, dist)| Race { time, dist }).collect())
}

pub fn part1(input: &str) -> Result<usize> {
    Ok(parse_input(input)?.into_iter().map(|race| race.brute_force_num_ways_to_win()).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(EXAMPLE).unwrap(),
            [Race { time: 7, dist: 9 }, Race { time: 15, dist: 40 }, Race { time: 30, dist: 200 }]
        );
    }

    #[test]
    fn test_distance_will_travel() {
        let race = Race { time: 7, dist: 9 };
        assert_eq!(race.distance_will_travel(0), 0);
        assert_eq!(race.distance_will_travel(1), 6);
        assert_eq!(race.distance_will_travel(2), 10);
        assert_eq!(race.distance_will_travel(3), 12);
        assert_eq!(race.distance_will_travel(4), 12);
        assert_eq!(race.distance_will_travel(5), 10);
        assert_eq!(race.distance_will_travel(6), 6);
        assert_eq!(race.distance_will_travel(7), 0);
    }

    #[test]
    fn test_brute_force_num_ways_to_win() {
        assert_eq!(Race { time: 7, dist: 9 }.brute_force_num_ways_to_win(), 4);
        assert_eq!(Race { time: 15, dist: 40 }.brute_force_num_ways_to_win(), 8);
        assert_eq!(Race { time: 30, dist: 200 }.brute_force_num_ways_to_win(), 9);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 288)
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 131376);
    }
}
