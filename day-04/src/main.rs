use std::{collections::HashSet, u32};

use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn card_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(many1(tag(" ")), map_res(digit1, str::parse))(input)
}

fn card_separator(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag::<_, _, (_, ErrorKind)>(" |")(input).expect("expected list separator");
    let (input, _) = many1::<_, _, (_, ErrorKind), _>(tag(" "))(input)
        .expect("should contain space after separator");
    Ok((input, ""))
}

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    selected_numbers: Vec<usize>,
}

impl Card {
    fn new(input: &str) -> Self {
        let (input, _) =
            tag::<_, _, (_, ErrorKind)>("Card")(input).expect("should start with 'Card'");
        let (input, _) = many1::<_, _, (_, ErrorKind), _>(tag(" "))(input)
            .expect("should contain space after 'Card'");
        let (input, id) = map_res::<_, _, _, (_, ErrorKind), _, _, _>(digit1, str::parse)(input)
            .expect("should contain a Game ID'");
        let (input, _) =
            tag::<_, _, (_, ErrorKind)>(":")(input).expect("should have delimiting colon");
        let (input, _) = many1::<_, _, (_, ErrorKind), _>(tag(" "))(input)
            .expect("should contain space after ':'");
        let (_, (winning_numbers, selected_numbers)) =
            separated_pair(card_numbers, card_separator, card_numbers)(input)
                .expect("should contain two number lists");

        Self {
            id,
            winning_numbers: HashSet::from_iter(winning_numbers),
            selected_numbers,
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
