use anyhow::{anyhow, Context, Result};

#[derive(Debug, PartialEq)]
struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn distance_will_travel(&self, time_button_held: usize) -> usize {
        (self.time - time_button_held) * time_button_held
    }

    fn wins(&self, time_button_held: usize) -> bool {
        self.distance_will_travel(time_button_held) > self.dist
    }

    #[allow(dead_code)]
    fn brute_force_num_ways_to_win(&self) -> usize {
        // Approx 1500ms on part2 with puzzle input
        (0..=self.time).filter(|&i| self.wins(i)).count()
    }

    #[allow(dead_code)]
    fn smarter_num_ways_to_win(&self) -> usize {
        // Approx 150ms on part2 with puzzle input
        //
        // Since we have a quadratic function, we know that it reaches an optimum at a point
        // By construction of the problem its safe to assume this is a maximum (could also just analyze the function)
        // In simpler math: (where t = race time, and x is the time the button is held)
        //     f(x) = tx - x^2
        // We can differentiate to find the maximum point:
        //     f'(x) = t - 2x
        //     => max_x = t/2
        // So we know that around t/2, the function will be maximized.
        // Below it and above it will be where we cross the current distance record

        let t = self.time / 2; // Because this is integer math it might not exactly be the max, but should be fine

        let mut low = t;
        let mut high = t;

        while self.wins(low) {
            low -= 1;
        }
        low += 1; // We go a bit too far

        while self.wins(high) {
            high += 1;
        }
        high -= 1; // We go a bit too far

        high - low + 1
    }

    fn binary_search_num_ways_to_win(&self) -> usize {
        // negligible runtime on part2 with puzzle input
        //
        // Same as smarter_num_ways_to_win, but with binary search

        let t = self.time / 2;

        // Once the binary search has narrowed the search range, switch to a linear scan
        const LINEAR_THRESHOLD: usize = 1;

        let mut l = 0;
        let mut h = t;
        while h - l > LINEAR_THRESHOLD {
            let m = (h + l) / 2;
            if self.wins(m) {
                h = m;
            } else {
                l = m;
            }
        }
        let mut low = l;
        while !self.wins(low) {
            low += 1;
        }

        let mut l = t;
        let mut h = self.time;
        while h - l > LINEAR_THRESHOLD {
            let m = (h + l) / 2;
            if self.wins(m) {
                l = m;
            } else {
                h = m;
            }
        }
        let mut high = h;
        while !self.wins(high) {
            high -= 1;
        }

        high - low + 1
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
    Ok(parse_input(input)?.into_iter().map(|race| race.smarter_num_ways_to_win()).product())
}

pub fn part2(input: &str) -> Result<usize> {
    let mut actual_race_time: String = String::default();
    let mut actual_race_distance: String = String::default();

    for race in parse_input(input)? {
        actual_race_time += &race.time.to_string();
        actual_race_distance += &race.dist.to_string();
    }

    let actual_race = Race { time: actual_race_time.parse()?, dist: actual_race_distance.parse()? };
    // Ok(actual_race.brute_force_num_ways_to_win())
    // Ok(actual_race.smarter_num_ways_to_win())
    Ok(actual_race.binary_search_num_ways_to_win())
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
        assert_eq!(part1(EXAMPLE).unwrap(), 288);
        assert_eq!(part2(EXAMPLE).unwrap(), 71503);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 131376);
        assert_eq!(part2(&input).unwrap(), 34123437);
    }
}
