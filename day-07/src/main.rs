use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{self, anychar, multispace1, space1},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Card(usize);

impl Card {
    fn new(c: char) -> Self {
        Self(match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            'X' => 1, // NOTE: JOKER!
            x => x.to_digit(10).expect("should be digit char") as usize,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut frequencies: Vec<_> = [value.0, value.1, value.2, value.3, value.4]
            .iter()
            .filter(|card| !matches!(card, Card(1))) // Remove jokers
            .fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            })
            .into_values()
            .collect();

        frequencies.sort();

        match frequencies {
            x if x == vec![5] => HandType::FiveOfAKind,
            x if x == vec![4] => HandType::FiveOfAKind,
            x if x == vec![3] => HandType::FiveOfAKind,
            x if x == vec![2] => HandType::FiveOfAKind,
            x if x == vec![1] => HandType::FiveOfAKind,
            x if x == vec![] => HandType::FiveOfAKind,
            x if x == vec![1, 4] => HandType::FourOfAKind,
            x if x == vec![1, 3] => HandType::FourOfAKind,
            x if x == vec![1, 2] => HandType::FourOfAKind,
            x if x == vec![1, 1] => HandType::FourOfAKind,
            x if x == vec![2, 3] => HandType::FullHouse,
            x if x == vec![2, 2] => HandType::FullHouse,
            x if x == vec![1, 1, 3] => HandType::ThreeOfAKind,
            x if x == vec![1, 1, 2] => HandType::ThreeOfAKind,
            x if x == vec![1, 1, 1] => HandType::ThreeOfAKind,
            x if x == vec![1, 2, 2] => HandType::TwoPair,
            x if x == vec![1, 1, 1, 2] => HandType::OnePair,
            x if x == vec![1, 1, 1, 1] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(Card, Card, Card, Card, Card, usize);

impl Hand {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (cards, bid)) = separated_pair(
            tuple((anychar, anychar, anychar, anychar, anychar)),
            space1,
            complete::u32,
        )(input)?;

        Ok((
            input,
            Self(
                Card::new(cards.0),
                Card::new(cards.1),
                Card::new(cards.2),
                Card::new(cards.3),
                Card::new(cards.4),
                bid as usize,
            ),
        ))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand: HandType = self.into();
        let other_hand: HandType = other.into();

        // Check for non-equal hand_type
        match self_hand.cmp(&other_hand) {
            Ordering::Equal => {}
            x => return x,
        }

        // Then try comparing the order of cards
        let self_tuple: (_, _, _, _, _) = self.into();
        let other_tuple: (_, _, _, _, _) = other.into();

        self_tuple.cmp(&other_tuple)
    }
}

impl From<&Hand> for (Card, Card, Card, Card, Card) {
    fn from(value: &Hand) -> Self {
        (value.0, value.1, value.2, value.3, value.4)
    }
}

fn process_part1(input: &str) -> usize {
    let (_, mut hands) = separated_list1(multispace1, Hand::parse)(input).expect("should parse");
    hands.sort_unstable();

    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.5)
        .sum()
}

fn process_part2(input: &str) -> usize {
    let input = &(input.replace('J', "X")); // Distinguish jokers
    let (_, mut hands) = separated_list1(multispace1, Hand::parse)(input).expect("should parse");
    hands.sort_unstable();

    hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.5)
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
    use rstest::rstest;

    use crate::*;

    #[rstest]
    #[case('A', 14)]
    #[case('K', 13)]
    #[case('Q', 12)]
    #[case('J', 11)]
    #[case('T', 10)]
    #[case('4', 4)]
    #[case('X', 1)]
    fn test_card_matching(#[case] c: char, #[case] value: usize) {
        assert_eq!(Card::new(c), Card(value));
    }

    #[test]
    fn test_card_cmp() {
        assert!(Card::new('A') > Card::new('T'));
        assert!(Card::new('K') > Card::new('Q'));
        assert!(Card::new('4') < Card::new('Q'));
        assert!(Card::new('4') == Card::new('4'));
    }

    #[rstest]
    // Regular tests
    #[case("AAAAA 1", HandType::FiveOfAKind)]
    #[case("AA8AA 1", HandType::FourOfAKind)]
    #[case("23332 1", HandType::FullHouse)]
    #[case("TTT98 1", HandType::ThreeOfAKind)]
    #[case("23432 1", HandType::TwoPair)]
    #[case("A23A4 1", HandType::OnePair)]
    #[case("23456 1", HandType::HighCard)]
    // Joker tests
    #[case("XXXXX 1", HandType::FiveOfAKind)]
    #[case("AAAAX 1", HandType::FiveOfAKind)]
    #[case("AAAXX 1", HandType::FiveOfAKind)]
    #[case("AAXXX 1", HandType::FiveOfAKind)]
    #[case("AXXXX 1", HandType::FiveOfAKind)]
    #[case("AAAX2 1", HandType::FourOfAKind)]
    #[case("AAXX2 1", HandType::FourOfAKind)]
    #[case("AXXX2 1", HandType::FourOfAKind)]
    #[case("AA22X 1", HandType::FullHouse)]
    #[case("AAX32 1", HandType::ThreeOfAKind)]
    #[case("AXX32 1", HandType::ThreeOfAKind)]
    #[case("X5432 1", HandType::OnePair)]
    fn test_hand_type(#[case] input: &str, #[case] hand_type: HandType) {
        let (_, hand) = Hand::parse(input).unwrap();

        assert_eq!(hand_type, (&hand).into());
    }

    #[rstest]
    #[case("AAAAA 1", "AA8AA 1")]
    #[case("AA8AA 1", "23332 1")]
    #[case("23332 1", "TTT98 1")]
    #[case("TTT98 1", "23432 1")]
    #[case("23432 1", "A23A4 1")]
    #[case("A23A4 1", "23456 1")]
    fn test_hand_cmp(#[case] gt: &str, #[case] lt: &str) {
        let (_, gt_hand) = Hand::parse(gt).unwrap();
        let (_, lt_hand) = Hand::parse(lt).unwrap();

        assert!(gt_hand > lt_hand);
    }

    #[test]
    fn test_part1() {
        let input = "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        assert_eq!(process_part1(input), 6440);
    }

    #[test]
    fn test_part2() {
        let input = "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        assert_eq!(process_part2(input), 5905);
    }
}
