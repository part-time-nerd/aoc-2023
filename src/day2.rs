use anyhow::{bail, Error, Result};
use std::str::FromStr;

#[derive(Default, Debug, PartialEq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Cubes {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut set = Self::default();

        for pair in s.split(',') {
            if let Some((num, color)) = pair.trim().split_once(' ') {
                let n = num.parse::<u32>()?;
                match color {
                    "red" => set.red += n,
                    "green" => set.green += n,
                    "blue" => set.blue += n,
                    _ => bail!("Expected red, green, or blue. Got {color}"),
                }
            } else {
                bail!("Expected a space-separated tuple, but {pair} could not be parsed");
            }
        }

        Ok(set)
    }
}

impl Cubes {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn max(self, other: Self) -> Self {
        Self { red: other.red.max(self.red), green: other.green.max(self.green), blue: other.blue.max(self.blue) }
    }
}

#[derive(Default, Debug, PartialEq)]
struct Game {
    id: u32,
    hands: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut game = Self::default();
        if let Some((header, games)) = s.trim().split_once(':') {
            if let Some(id) = header.strip_prefix("Game ") {
                game.id = id.parse()?;
            } else {
                bail!("Expected the header to start with 'Game ', but it did not: {header}");
            }
            for hand in games.split(';') {
                game.hands.push(hand.parse()?);
            }
        }
        Ok(game)
    }
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    let mut games: Vec<Game> = Vec::new();
    for input_line in input.lines() {
        games.push(input_line.parse()?);
    }
    Ok(games)
}

pub fn part1(input: &str) -> Result<u32> {
    Ok(parse_input(input)?.into_iter().filter(|g| g.hands.iter().all(|h| h.is_valid())).map(|g| g.id).sum())
}

pub fn part2(input: &str) -> Result<u32> {
    Ok(parse_input(input)?.into_iter().map(|g| g.hands.into_iter().fold(Cubes::default(), Cubes::max).power()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_example() {
        assert_eq!(part1(EXAMPLE).unwrap(), 8);
        assert_eq!(part2(EXAMPLE).unwrap(), 2286);
    }

    #[test]
    fn test_solution() {
        let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
        assert_eq!(part1(&input).unwrap(), 2551);
        assert_eq!(part2(&input).unwrap(), 62811);
    }

    #[test]
    fn test_cubes_from_str() {
        assert_eq!(" 3 blue, 4 red".parse::<Cubes>().unwrap(), Cubes { green: 0, red: 4, blue: 3 });
        assert_eq!(" 20 red, 8 green, 6 blue ".parse::<Cubes>().unwrap(), Cubes { green: 8, red: 20, blue: 6 })
    }

    #[test]
    fn test_game_from_str() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse::<Game>().unwrap(),
            Game {
                id: 1,
                hands: vec![
                    Cubes { blue: 3, red: 4, green: 0 },
                    Cubes { red: 1, green: 2, blue: 6 },
                    Cubes { green: 2, blue: 0, red: 0 }
                ]
            }
        )
    }
}
