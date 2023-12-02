use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::max;

static GAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game (\d*)").unwrap());
static RED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*) red").unwrap());
static GREEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*) green").unwrap());
static BLUE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d*) blue").unwrap());

fn capture_num(regex: &Lazy<Regex>, haystack: &str) -> Option<usize> {
    let caps = regex.captures(haystack)?;
    return caps[1].parse::<usize>().ok();
}



#[derive(Clone, Copy, PartialEq, Debug)]
struct Handful(usize, usize, usize);

impl Handful {
    fn new(input: &str) -> Self {
        let red_count = capture_num(&RED_REGEX, input).unwrap_or(0);
        let green_count = capture_num(&GREEN_REGEX, input).unwrap_or(0);
        let blue_count = capture_num(&BLUE_REGEX, input).unwrap_or(0);

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
        // NOTE: Unwrapping is used when the input is GUARANTEED.
        // The code should not work without proper input strings.
        let (game, handful_strs) = desc.split_once(":").unwrap();

        Self {
            id: capture_num(&GAME_REGEX, game).unwrap(),
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

        return min.0 * min.1 * min.2;
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
        assert_eq!(Handful::new("3 blue, 4 red"), Handful(4, 0, 3));
        assert_eq!(Handful::new("1 red, 2 green, 6 blue"), Handful(1, 2, 6));
        assert_eq!(Handful::new("2 green"), Handful(0, 2, 0));
        assert_eq!(Handful::new("1 blue, 2 green"), Handful(0, 2, 1));
        assert_eq!(Handful::new("3 green, 4 blue, 1 red"), Handful(1, 3, 4));
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

    fn _make_games() -> [Game; 5] {
        return [
            Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game::new("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            Game::new("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            Game::new("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            Game::new("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
    }

    #[test]
    fn game_min_cubes() {
        let games = _make_games();
        assert_eq!(games[0].minimum_cubes(), (4, 2, 6));
        assert_eq!(games[1].minimum_cubes(), (1, 3, 4));
        assert_eq!(games[2].minimum_cubes(), (20, 13, 6));
        assert_eq!(games[3].minimum_cubes(), (14, 3, 15));
        assert_eq!(games[4].minimum_cubes(), (6, 3, 2));
    }

    #[test]
    fn game_within_max() {
        let games = _make_games();
        assert!(games[0].is_within_max(12, 13, 14));
        assert!(games[1].is_within_max(12, 13, 14));
        assert!(!games[2].is_within_max(12, 13, 14));
        assert!(!games[3].is_within_max(12, 13, 14));
        assert!(games[4].is_within_max(12, 13, 14));
    }

    #[test]
    fn game_min_powers() {
        let games = _make_games();
        assert_eq!(games[0].power_of_min_set(), 48);
        assert_eq!(games[1].power_of_min_set(), 12);
        assert_eq!(games[2].power_of_min_set(), 1560);
        assert_eq!(games[3].power_of_min_set(), 630);
        assert_eq!(games[4].power_of_min_set(), 36);
    }
}
