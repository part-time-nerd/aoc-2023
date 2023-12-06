use anyhow::{anyhow, Context, Error, Result};
use std::{ops::Range, str::FromStr};

#[derive(Debug, PartialEq)]
struct Map {
    dest: usize,
    source: usize,
    length: usize,
}

impl FromStr for Map {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        if let [d, s, l] = input.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            Ok(Self { dest: d.parse()?, source: s.parse()?, length: l.parse()? })
        } else {
            Err(anyhow!("Could not parse three parts from {input}"))
        }
    }
}

impl Map {
    fn domain(&self) -> Range<usize> {
        self.source..self.source + self.length
    }

    fn range(&self) -> Range<usize> {
        self.dest..self.dest + self.length
    }

    fn map(&self, value: usize) -> Option<usize> {
        if self.domain().contains(&value) {
            Some(self.dest + value - self.source)
        } else {
            None
        }
    }

    fn map_range(&self, values: Range<usize>) -> (Range<usize>, Range<usize>, Range<usize>) {
        // Returns a triple (before, mapped_values, after)
        if values.is_empty() {
            return (Range::default(), Range::default(), Range::default());
        }
        match (self.map(values.start), self.map(values.end)) {
            (Some(start), Some(end)) => (Range::default(), start..end, Range::default()),
            (None, Some(end)) => (values.start..self.domain().start, self.range().start..end, Range::default()),
            (Some(start), None) => (Range::default(), start..self.range().end, self.domain().end..values.end),
            (None, None) => {
                if values.start < self.domain().start {
                    if values.end >= self.domain().end {
                        (values.start..self.domain().start, self.range(), self.domain().end..values.end)
                    } else {
                        (values, Range::default(), Range::default())
                    }
                } else {
                    (Range::default(), Range::default(), values)
                }
            }
        }
    }
}

#[derive(Default, Debug, PartialEq)]
struct Maps(Vec<Map>);

impl FromStr for Maps {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut maps = Self::default();
        for l in input.lines().skip(1) {
            maps.0.push(l.parse()?);
        }
        Ok(maps)
    }
}

impl Maps {
    fn map(&self, value: usize) -> usize {
        self.0.iter().find_map(|m| m.map(value)).unwrap_or(value)
    }

    fn map_ranges(&self, mut unmapped: Vec<Range<usize>>) -> Vec<Range<usize>> {
        let mut mapped: Vec<Range<usize>> = Vec::new();

        for map in self.0.iter() {
            let mut new_unmapped: Vec<Range<usize>> = Vec::new();
            for range in unmapped.into_iter() {
                let (unmapped_before, mapped_values, unmapped_after) = map.map_range(range);
                if !unmapped_before.is_empty() {
                    new_unmapped.push(unmapped_before);
                }
                if !unmapped_after.is_empty() {
                    new_unmapped.push(unmapped_after)
                }
                if !mapped_values.is_empty() {
                    mapped.push(mapped_values);
                }
            }
            unmapped = new_unmapped;
        }
        mapped.extend(unmapped); // Unmapped values are mapped identically
        mapped
    }
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Maps>,
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        let input_sections: Vec<&str> = input.split("\n\n").collect();
        if let [s, s2s, s2f, f2w, w2l, l2t, t2h, h2l] = input_sections[..] {
            let mut seeds: Vec<usize> = Vec::new();
            for seed in s.strip_prefix("seeds: ").context("Missing seeds header")?.split_ascii_whitespace() {
                seeds.push(seed.parse()?);
            }
            Ok(Self {
                seeds,
                maps: vec![
                    s2s.parse()?,
                    s2f.parse()?,
                    f2w.parse()?,
                    w2l.parse()?,
                    l2t.parse()?,
                    t2h.parse()?,
                    h2l.parse()?,
                ],
            })
        } else {
            Err(anyhow!("Could not parse all of the sections from the input"))
        }
    }
}

impl Almanac {
    fn seed_location(&self, seed: usize) -> usize {
        let mut current = seed;
        for map in &self.maps {
            current = map.map(current);
        }
        current
    }

    fn seed_locations(&self) -> Vec<usize> {
        self.seeds.iter().map(|&seed| self.seed_location(seed)).collect()
    }

    fn seed_ranges(&self) -> Vec<Range<usize>> {
        // The interpretation of seeds as per part 2
        let mut seed_ranges: Vec<Range<usize>> = Vec::new();
        for chunk in self.seeds.chunks(2) {
            if let &[start, length] = chunk {
                seed_ranges.push(start..start + length);
            } else {
                panic!("Did not expect an odd number of seeds")
            }
        }
        seed_ranges
    }

    fn seed_range_locations(&self) -> Vec<Range<usize>> {
        self.maps.iter().fold(self.seed_ranges(), |current, map| map.map_ranges(current))
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let almanac: Almanac = input.parse()?;
    almanac.seed_locations().into_iter().min().context("No seed locations")
}

pub fn part2(input: &str) -> Result<usize> {
    let almanac: Almanac = input.parse()?;
    almanac.seed_range_locations().into_iter().map(|r| r.start).min().context("No seed range locations")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_parse_almanac() {
        let expected = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Maps(vec![Map { dest: 50, source: 98, length: 2 }, Map { dest: 52, source: 50, length: 48 }]),
                Maps(vec![
                    Map { dest: 0, source: 15, length: 37 },
                    Map { dest: 37, source: 52, length: 2 },
                    Map { dest: 39, source: 0, length: 15 },
                ]),
                Maps(vec![
                    Map { dest: 49, source: 53, length: 8 },
                    Map { dest: 0, source: 11, length: 42 },
                    Map { dest: 42, source: 0, length: 7 },
                    Map { dest: 57, source: 7, length: 4 },
                ]),
                Maps(vec![Map { dest: 88, source: 18, length: 7 }, Map { dest: 18, source: 25, length: 70 }]),
                Maps(vec![
                    Map { dest: 45, source: 77, length: 23 },
                    Map { dest: 81, source: 45, length: 19 },
                    Map { dest: 68, source: 64, length: 13 },
                ]),
                Maps(vec![Map { dest: 0, source: 69, length: 1 }, Map { dest: 1, source: 0, length: 69 }]),
                Maps(vec![Map { dest: 60, source: 56, length: 37 }, Map { dest: 56, source: 93, length: 4 }]),
            ],
        };
        assert_eq!(EXAMPLE.parse::<Almanac>().unwrap(), expected);
    }

    #[test]
    fn test_seed_location() {
        assert_eq!(EXAMPLE.parse::<Almanac>().unwrap().seed_location(79), 82);
    }

    #[test]
    fn test_seed_locations() {
        assert_eq!(EXAMPLE.parse::<Almanac>().unwrap().seed_locations(), [82, 43, 86, 35]);
    }

    #[test]
    fn test_seed_ranges() {
        assert_eq!(EXAMPLE.parse::<Almanac>().unwrap().seed_ranges(), [79..93, 55..68])
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 35);
        assert_eq!(part2(EXAMPLE).unwrap(), 46);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 111627841);
        assert_eq!(part2(&input).unwrap(), 69323688);
    }
}
