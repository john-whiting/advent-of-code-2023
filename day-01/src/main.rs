#[derive(Clone, Copy, PartialEq)]
enum AOCMode {
    Part1,
    Part2,
}

#[derive(Clone, Copy)]
struct Findable<'a>(&'a str, usize, AOCMode);

fn is_for_part1(findable: &Findable<'_>) -> bool {
    findable.2 == AOCMode::Part1
}

const FINDABLES: [Findable; 18] = [
    Findable("1", 1, AOCMode::Part1),
    Findable("one", 1, AOCMode::Part2),
    Findable("2", 2, AOCMode::Part1),
    Findable("two", 2, AOCMode::Part2),
    Findable("3", 3, AOCMode::Part1),
    Findable("three", 3, AOCMode::Part2),
    Findable("4", 4, AOCMode::Part1),
    Findable("four", 4, AOCMode::Part2),
    Findable("5", 5, AOCMode::Part1),
    Findable("five", 5, AOCMode::Part2),
    Findable("6", 6, AOCMode::Part1),
    Findable("six", 6, AOCMode::Part2),
    Findable("7", 7, AOCMode::Part1),
    Findable("seven", 7, AOCMode::Part2),
    Findable("8", 8, AOCMode::Part1),
    Findable("eight", 8, AOCMode::Part2),
    Findable("9", 9, AOCMode::Part1),
    Findable("nine", 9, AOCMode::Part2),
];

fn get_calibration_value(line: &str, aoc_mode: AOCMode) -> usize {
    let findables = FINDABLES
        .iter()
        // Limit findables to ones that exist in the line AND for the specific AOC parts
        .filter(|findable| {
            (aoc_mode == AOCMode::Part2 || is_for_part1(findable)) && line.contains(findable.0)
        });

    let first_value = findables
        .clone()
        .min_by_key(|findable| line.find(findable.0).unwrap())
        .unwrap()
        .1; // NOTE: Panic if not found (it should always be found)

    let last_value = findables
        .max_by_key(|findable| line.rfind(findable.0).unwrap())
        .unwrap()
        .1; // NOTE: Panic if not found (it should always be found)

    (first_value * 10) + last_value
}

fn get_sum_of_input(input: &str, aoc_mode: AOCMode) -> usize {
    input
        .lines()
        .map(|line| get_calibration_value(line, aoc_mode))
        .sum()
}

fn main() {
    let aoc_input = include_str!("./input.txt");

    println!(
        "Part 1 sum: {}",
        get_sum_of_input(aoc_input, AOCMode::Part1)
    );
    println!(
        "Part 2 sum: {}",
        get_sum_of_input(aoc_input, AOCMode::Part2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value_part1() {
        assert_eq!(get_calibration_value("1", AOCMode::Part1), 11);
        assert_eq!(get_calibration_value("1abc2", AOCMode::Part1), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx", AOCMode::Part1), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f", AOCMode::Part1), 15);
        assert_eq!(get_calibration_value("treb7uchet", AOCMode::Part1), 77);
    }

    #[test]
    fn calibration_value_part2() {
        assert_eq!(get_calibration_value("onetwo", AOCMode::Part2), 12);
        assert_eq!(get_calibration_value("twone", AOCMode::Part2), 21);
        assert_eq!(get_calibration_value("two1", AOCMode::Part2), 21);
    }

    #[test]
    fn test_input_sum() {
        let input = "onetwo
            twone";
        assert_eq!(get_sum_of_input(input, AOCMode::Part2), 33);
    }
}
