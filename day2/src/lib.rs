use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn get_lines_from_filename(filename: String) -> Result<Vec<String>, io::Error> {
    let path = filename;
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    return Ok(lines);
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Color, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BagPull {
    count: i32,
    color: Color,
}

#[derive(Clone, Debug)]
pub struct BagPullSet {
    bag_pulls: Vec<BagPull>,
}

impl BagPullSet {
    pub fn get_bag_pull_sets_from_line(line: &String) -> Vec<BagPullSet> {
        line.split(';')
            .map(|set| {
                // get rid of the whitespace
                let bag_pulls = set
                    .trim()
                    // split out each bag pull instance
                    .split(',')
                    .filter_map(|bp| {
                        let parts: Vec<&str> = bp.trim().split_whitespace().collect();
                        if parts.len() == 2 {
                            // count defaults to 0 if parse fails
                            let count = parts[0].parse::<i32>().unwrap_or(0);
                            // color defaults to Red if parse fails
                            let color = Color::from_str(parts[1]).unwrap_or(Color::Red);
                            Some(BagPull { count, color })
                        } else {
                            None
                        }
                    })
                    .collect();
                BagPullSet { bag_pulls }
            })
            .collect()
    }
}

pub struct Game {
    id: i32,
    bag_pull_sets: Vec<BagPullSet>,
}

impl Game {
    fn new(id: i32, bag_pull_sets: Vec<BagPullSet>) -> Game {
        Game { id, bag_pull_sets }
    }

    fn get_id_from_line(line: &String) -> Option<i32> {
        line.split(':')
            .next()?
            .trim()
            .split_whitespace()
            .last()?
            .parse()
            .ok()
    }

    fn from_line(line: &String) -> Option<Game> {
        let id = Game::get_id_from_line(line)?;
        let bag_pull_sets = BagPullSet::get_bag_pull_sets_from_line(line);
        Some(Game::new(id, bag_pull_sets))
    }

    fn clone(&self) -> Self {
        Game {
            id: self.id,
            bag_pull_sets: self.bag_pull_sets.clone(),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color = match *self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue",
        };
        write!(f, "{}", color)
    }
}

impl fmt::Display for BagPull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.count, self.color)
    }
}

impl fmt::Display for BagPullSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pulls: Vec<String> = self.bag_pulls.iter().map(|bp| bp.to_string()).collect();
        write!(f, "{}", pulls.join(", "))
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sets: Vec<String> = self
            .bag_pull_sets
            .iter()
            .map(|set| set.to_string())
            .collect();
        write!(f, "Game {}: {}", self.id, sets.join("; "))
    }
}

pub fn process_lines(lines: Vec<String>) -> i32 {
    const MAX_RED_CUBES: i32 = 12;
    const MAX_GREEN_CUBES: i32 = 13;
    const MAX_BLUE_CUBES: i32 = 14;

    let games: Vec<Game> = lines
        .iter()
        .filter_map(|line| Game::from_line(line))
        .collect();

    let mut valid_games: Vec<Game> = Vec::new();

    for game in games.iter() {
        let mut is_valid_game: bool = true;

        for bag_pull_set in game.bag_pull_sets.iter() {
            for bag_pull in bag_pull_set.bag_pulls.iter() {
                match bag_pull.color {
                    Color::Red => {
                        if bag_pull.count > MAX_RED_CUBES {
                            eprintln!("INVALID RED CUBES COUNT: {}", bag_pull.count);
                            is_valid_game = false;
                        }
                    }
                    Color::Green => {
                        if bag_pull.count > MAX_GREEN_CUBES {
                            eprintln!("INVALID GREEN CUBES COUNT: {}", bag_pull.count);
                            is_valid_game = false;
                        }
                    }
                    Color::Blue => {
                        if bag_pull.count > MAX_BLUE_CUBES {
                            eprintln!("INVALID BLUE CUBES COUNT: {}", bag_pull.count);
                            is_valid_game = false;
                        }
                    }
                }
            }
        }

        if is_valid_game {
            valid_games.push(game.clone());
        }
    }

    return valid_games.iter().map(|game| game.id).sum();
}
