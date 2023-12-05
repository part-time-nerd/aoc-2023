use anyhow::{anyhow, bail, Context, Error, Result};
use std::str::FromStr;

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

// Can't figure out how to impl FromStr for Vec<Map> (if its even possible)
fn parse_maps(input: &str) -> Result<Vec<Map>> {
    let mut maps: Vec<Map> = Vec::new();
    for l in input.lines().skip(1) {
        maps.push(l.parse()?);
    }
    Ok(maps)
}

#[derive(Debug, PartialEq)]
struct Almanac {
    seeds: Vec<usize>,
    // Just going to use Vec for the ranges its probably good enough
    // TODO: Can we do binary search on this so its fast?
    seed_to_soil: Vec<Map>,
    soil_to_fert: Vec<Map>,
    fert_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temp: Vec<Map>,
    temp_to_humid: Vec<Map>,
    humid_to_loc: Vec<Map>,
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
                seed_to_soil: parse_maps(s2s)?,
                soil_to_fert: parse_maps(s2f)?,
                fert_to_water: parse_maps(f2w)?,
                water_to_light: parse_maps(w2l)?,
                light_to_temp: parse_maps(l2t)?,
                temp_to_humid: parse_maps(t2h)?,
                humid_to_loc: parse_maps(h2l)?,
            })
        } else {
            println!("{:?}", input_sections);
            Err(anyhow!("Could not parse all of the sections from the input"))
        }
    }
}

impl Almanac {
    fn seed_location(&self, seed: usize) -> usize {
        let all_maps = [
            &self.seed_to_soil,
            &self.soil_to_fert,
            &self.fert_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humid,
            &self.humid_to_loc,
        ];
        let mut current = seed;
        for maps in all_maps {
            if let Some(map) = maps.iter().filter(|m| current >= m.source && current < m.source + m.length).next() {
                current = map.dest + current - map.source;
            }
            // No mapping found: keep the same source -> dest
        }
        current
    }

    fn seed_locations(&self) -> Vec<usize> {
        self.seeds.iter().map(|&seed| self.seed_location(seed)).collect()
    }
}

pub fn part1(input: &str) -> Result<usize> {
    let almanac: Almanac = input.parse()?;
    almanac.seed_locations().into_iter().min().context("No seed locations")
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
            seed_to_soil: vec![Map { dest: 50, source: 98, length: 2 }, Map { dest: 52, source: 50, length: 48 }],
            soil_to_fert: vec![
                Map { dest: 0, source: 15, length: 37 },
                Map { dest: 37, source: 52, length: 2 },
                Map { dest: 39, source: 0, length: 15 },
            ],
            fert_to_water: vec![
                Map { dest: 49, source: 53, length: 8 },
                Map { dest: 0, source: 11, length: 42 },
                Map { dest: 42, source: 0, length: 7 },
                Map { dest: 57, source: 7, length: 4 },
            ],
            water_to_light: vec![Map { dest: 88, source: 18, length: 7 }, Map { dest: 18, source: 25, length: 70 }],
            light_to_temp: vec![
                Map { dest: 45, source: 77, length: 23 },
                Map { dest: 81, source: 45, length: 19 },
                Map { dest: 68, source: 64, length: 13 },
            ],
            temp_to_humid: vec![Map { dest: 0, source: 69, length: 1 }, Map { dest: 1, source: 0, length: 69 }],
            humid_to_loc: vec![Map { dest: 60, source: 56, length: 37 }, Map { dest: 56, source: 93, length: 4 }],
        };

        assert_eq!(EXAMPLE.parse::<Almanac>().unwrap(), expected);
    }

    #[test]
    fn test_seed_location() {
        let almanac: Almanac = EXAMPLE.parse().unwrap();
        assert_eq!(almanac.seed_location(79), 82);
    }

    #[test]
    fn test_seed_locations() {
        let almanac: Almanac = EXAMPLE.parse().unwrap();
        assert_eq!(almanac.seed_locations(), [82, 43, 86, 35]);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 35);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 111627841);
    }
}
