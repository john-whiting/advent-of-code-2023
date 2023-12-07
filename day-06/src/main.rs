use nom::{
    character::complete::{self, multispace1, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_supreme::{tag::complete::tag, ParserExt};

fn get_time_range(hold_time: i64, target_distance: i64) -> (i64, i64) {
    let hold_time = hold_time as f64;
    let target_distance = target_distance as f64;

    let max_distance = (hold_time * hold_time) / 4.0;
    let max_distance_time = hold_time / 2.0;

    let low = (max_distance_time - (max_distance - target_distance).sqrt()).floor() as i64;
    let high = (max_distance_time + (max_distance - target_distance).sqrt()).ceil() as i64;

    (low + 1, high - 1)
}

fn get_margin(hold_time: i64, target_distance: i64) -> i64 {
    let (low, high) = get_time_range(hold_time, target_distance);

    (high - low) + 1
}

fn parse_values(input: &str) -> IResult<&str, (Vec<i64>, Vec<i64>)> {
    separated_pair(
        tag("Time:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::i64)),
        multispace1,
        tag("Distance:")
            .precedes(space1)
            .precedes(separated_list1(space1, complete::i64)),
    )(input)
}

fn process_part1(input: &str) -> i64 {
    let (_, (times, distances)) = parse_values(input).expect("should parse times and distances");

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| get_margin(time, distance))
        .product()
}

fn process_part2(input: &str) -> i64 {
    let (_, (times, distances)) = parse_values(input).expect("should parse times and distances");

    let time = times
        .iter()
        .fold(String::new(), |acc, time| acc + &time.to_string())
        .parse()
        .expect("time expected");

    let distance = distances
        .iter()
        .fold(String::new(), |acc, distance| acc + &distance.to_string())
        .parse()
        .expect("distance expected");

    get_margin(time, distance)
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
    fn test_time_range() {
        assert_eq!(get_time_range(7, 9), (2, 5));
        assert_eq!(get_time_range(15, 40), (4, 11));
        assert_eq!(get_time_range(30, 200), (11, 19));
    }

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
            Distance:  9  40  200";
        assert_eq!(process_part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
            Distance:  9  40  200";
        assert_eq!(process_part2(input), 71503);
    }
}
