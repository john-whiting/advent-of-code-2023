use std::{collections::HashSet, u32};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

fn card_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, complete::u32)(input)
}

fn card_prefix(input: &str) -> IResult<&str, (&str, &str, &str, &str, &str)> {
    tuple((tag("Card"), space1, digit1, tag(":"), space1))(input)
}

fn card_separator(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((space1, tag("|"), space1))(input)
}

fn card_sections(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    separated_pair(card_numbers, card_separator, card_numbers)(input)
}

struct Card {
    winning_numbers: HashSet<u32>,
    selected_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Self {
        let (input, _) = card_prefix(input).expect("expected `Card ##:`");
        let (_, (win_nums, sel_nums)) = card_sections(input).expect("expected card sections");

        Self {
            winning_numbers: HashSet::from_iter(win_nums),
            selected_numbers: sel_nums,
        }
    }

    fn count_matching(&self) -> usize {
        self.selected_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }
}

fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::new(line.trim()).count_matching())
        .filter_map(|count| match count {
            0 => None,
            c => Some(2u32.pow((c as u32) - 1)),
        })
        .sum()
}

fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::new(line.trim()).count_matching())
        .rev()
        .fold(Vec::new(), |mut acc, count| {
            acc.insert(0, 1 + &acc[0..count].iter().sum::<u32>());
            acc
        })
        .iter()
        .sum()
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
    fn test_part1() {
        let schematic = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part1(schematic), 13);
    }

    #[test]
    fn test_part2() {
        let schematic = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process_part2(schematic), 30);
    }
}
