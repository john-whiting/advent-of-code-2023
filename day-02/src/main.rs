use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::max;



static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (?<id>\d*)").unwrap());
static RED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d*) red").unwrap());
static GREEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d*) green").unwrap());
static BLUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<count>\d*) blue").unwrap());

fn capture_count(regex: &Lazy<Regex>, set: &str) -> usize {
    if let Some(caps) = regex.captures(set) {
        if let Ok(count) = caps["count"].parse::<usize>() {
            return count;
        }
    }

    return 0;
}



#[derive(Clone, Copy, PartialEq, Debug)]
struct Handful(usize, usize, usize);

impl Handful {
    fn new(set: &str) -> Self {
        let red_count = capture_count(&RED_REGEX, set);
        let green_count = capture_count(&GREEN_REGEX, set);
        let blue_count = capture_count(&BLUE_REGEX, set);

        Self(red_count, green_count, blue_count)
    }
}



#[derive(Clone, PartialEq, Debug)]
struct Game {
    id: usize,
    handfuls: Vec<Handful>,
}

impl Game {
    fn new(desc: &str) -> Self {
        // NOTE: Unwrapping is used when the input is GUARANTEED. The code should not work without
        // proper input strings.
        let (game, handful_strs) = desc.split_once(":").unwrap();

        Self {
            id: GAME_REGEX.captures(game).unwrap()["id"].parse::<usize>().unwrap(),
            handfuls: handful_strs.split(";").map(Handful::new).collect::<Vec<_>>(),
        }
    }

    fn minimum_cubes(&self) -> (usize, usize, usize) {
        self.handfuls.iter()
            .fold((0, 0, 0), |acc, handful| (max(acc.0, handful.0), max(acc.1, handful.1), max(acc.2, handful.2)))
    }

    fn is_within_max(&self, r: usize, g: usize, b: usize) -> bool {
        let min = self.minimum_cubes();

        return min.0 <= r && min.1 <= g && min.2 <= b;
    }

    fn power_of_min_set(&self) -> usize {
        let min = self.minimum_cubes();

        min.0 * min.1 * min.2
    }
}



fn main() {
    let aoc_input = include_str!("./input.txt");
    let games = aoc_input.lines().map(Game::new).collect::<Vec<_>>();

    // Part 1
    let valid_games = games.iter().filter(|game| game.is_within_max(12, 13, 14));
    let part1_answer: usize = valid_games.map(|game| game.id).sum();

    println!("Part 1 sum: {}", part1_answer);

    // Part 2
    let part2_answer: usize = games.iter().map(Game::power_of_min_set).sum();
    println!("Part 2 sum: {}", part2_answer);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handful_parsing() {
        let input = "3 blue, 4 red";
        assert_eq!(Handful::new(input), Handful(4, 0, 3));

        let input = "1 red, 2 green, 6 blue";
        assert_eq!(Handful::new(input), Handful(1, 2, 6));

        let input = "2 green";
        assert_eq!(Handful::new(input), Handful(0, 2, 0));

        let input = "1 blue, 2 green";
        assert_eq!(Handful::new(input), Handful(0, 2, 1));

        let input = "3 green, 4 blue, 1 red";
        assert_eq!(Handful::new(input), Handful(1, 3, 4));
    }

    #[test]
    fn game_parsing() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let handfuls = vec![Handful(4, 0, 3), Handful(1, 2, 6), Handful(0, 2, 0)];
        assert_eq!(Game::new(input), Game { id: 1, handfuls });

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let handfuls = vec![Handful(0, 2, 1), Handful(1, 3, 4), Handful(0, 1, 1)];
        assert_eq!(Game::new(input), Game { id: 2, handfuls });
    }

    #[test]
    fn game_min_cubes() {
        let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game1.minimum_cubes(), (4, 2, 6));

        let game2 = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(game2.minimum_cubes(), (1, 3, 4));

        let game3 = Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(game3.minimum_cubes(), (20, 13, 6));

        let game4 = Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(game4.minimum_cubes(), (14, 3, 15));

        let game5 = Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(game5.minimum_cubes(), (6, 3, 2));
    }

    #[test]
    fn game_within_max() {
        let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert!(game1.is_within_max(12, 13, 14));

        let game2 = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert!(game2.is_within_max(12, 13, 14));

        let game3 = Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert!(!game3.is_within_max(12, 13, 14));

        let game4 = Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert!(!game4.is_within_max(12, 13, 14));

        let game5 = Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert!(game5.is_within_max(12, 13, 14));
    }

    #[test]
    fn game_min_powers() {
        let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game1.power_of_min_set(), 48);

        let game2 = Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(game2.power_of_min_set(), 12);

        let game3 = Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(game3.power_of_min_set(), 1560);

        let game4 = Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(game4.power_of_min_set(), 630);

        let game5 = Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(game5.power_of_min_set(), 36);
    }
}
