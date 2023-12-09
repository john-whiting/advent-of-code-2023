use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use num_integer::lcm;

#[derive(Debug)]
struct DirectionParseError;

type DirectionPair<'a> = (&'a str, &'a str);

enum Direction {
    Right,
    Left,
}

impl Direction {
    fn parse(c: char) -> Result<Self, DirectionParseError> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(DirectionParseError),
        }
    }
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(map_res(
        alt((complete::char('L'), complete::char('R'))),
        Direction::parse,
    ))(input)
}

fn mapping(input: &str) -> IResult<&str, Vec<(&str, DirectionPair<'_>)>> {
    separated_list1(
        multispace1,
        tuple((
            take(3usize),
            delimited(
                tag(" = ("),
                separated_pair(take(3usize), tag(", "), take(3usize)),
                tag(")"),
            ),
        )),
    )(input)
}

fn aoc_input(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, DirectionPair<'_>>)> {
    let (input, (found_directions, found_mappings)) =
        separated_pair(directions, multispace1, mapping)(input)?;

    Ok((
        input,
        (found_directions, found_mappings.into_iter().collect()),
    ))
}

fn path_length_to_end(
    directions: &Vec<Direction>,
    map: &HashMap<&str, (&str, &str)>,
    start_node: &str,
) -> usize {
    let mut count = 0;
    let mut current_node = start_node;

    // Based on part 2 (and testing), we can just check for ending in Z
    while !current_node.ends_with('Z') {
        let next_direction = directions
            .get(count % directions.len())
            .expect("should be valid dirs index");
        current_node = match next_direction {
            Direction::Left => map[current_node].0,
            Direction::Right => map[current_node].1,
        };
        count += 1;
    }

    count
}

fn process_part1(input: &str) -> usize {
    let (_, (dirs, map)) = aoc_input(input).expect("should have AOC input");

    path_length_to_end(&dirs, &map, "AAA")
}

fn process_part2(input: &str) -> usize {
    let (_, (dirs, map)) = aoc_input(input).expect("should have AOC input");

    // Based on testing assumptions and input observation,
    // the LCM of the path length's to the terminal nodes
    // can be used for the result. This is largely becauase
    // the inputs are circular.

    map.keys()
        .filter(|key| key.ends_with('A'))
        .map(|key| path_length_to_end(&dirs, &map, key))
        .fold(1, lcm)
}

fn main() {
    let aoc_input = include_str!("input.txt");
    let part1_solution = process_part1(aoc_input);
    let part2_solution = process_part2(aoc_input);

    println!("Part 1 Solution: {part1_solution}");
    println!("Part 2 Solution: {part2_solution}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ";
        assert_eq!(process_part1(input), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process_part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process_part2(input), 6);
    }
}
