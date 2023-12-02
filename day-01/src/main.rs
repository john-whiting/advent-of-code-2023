#[derive(Clone, Copy)]
enum FindableType { Normal, Advanced }

#[derive(Clone, Copy)]
struct Findable<'a> (&'a str, usize, FindableType);

impl<'a> Findable<'a> {
    fn normal(&self) -> bool {
        match self {
            Findable(_, _, FindableType::Normal) => true,
            _ => false,
        }
    }
}

const FINDABLES: [Findable; 18] = [
    Findable("1", 1, FindableType::Normal), Findable("one", 1, FindableType::Advanced),
    Findable("2", 2, FindableType::Normal), Findable("two", 2, FindableType::Advanced),
    Findable("3", 3, FindableType::Normal), Findable("three", 3, FindableType::Advanced),
    Findable("4", 4, FindableType::Normal), Findable("four", 4, FindableType::Advanced),
    Findable("5", 5, FindableType::Normal), Findable("five", 5, FindableType::Advanced),
    Findable("6", 6, FindableType::Normal), Findable("six", 6, FindableType::Advanced),
    Findable("7", 7, FindableType::Normal), Findable("seven", 7, FindableType::Advanced),
    Findable("8", 8, FindableType::Normal), Findable("eight", 8, FindableType::Advanced),
    Findable("9", 9, FindableType::Normal), Findable("nine", 9, FindableType::Advanced),
];

fn get_calibration_value(line: &str, allow_advanced: bool) -> usize {
    let findables = FINDABLES.iter().filter(|findable| allow_advanced || findable.normal());

    let first_value = findables
        .clone()
        .map(|findable| (findable, line.find(findable.0)))
        .filter(|item| item.1.is_some())
        .map(|item| (item.0, item.1.unwrap()))
        .min_by_key(|item| item.1)
        .unwrap() // NOTE: Panic if not found (it should always be found)
        .0.1;

    let last_value = findables
        .map(|findable| (findable, line.rfind(findable.0)))
        .filter(|item| item.1.is_some())
        .map(|item| (item.0, item.1.unwrap()))
        .max_by_key(|item| item.1)
        .unwrap() // NOTE: Panic if not found (it should always be found)
        .0.1;

    return (first_value * 10) + last_value;
}

fn get_sum_of_input(input: &str, allow_advanced: bool) -> usize {
    input
        .lines()
        .map(|line| get_calibration_value(line, allow_advanced))
        .sum()
}


fn main() {
    let aoc_input = include_str!("./input.txt");

    println!("Part 1 sum: {}", get_sum_of_input(aoc_input, false));
    println!("Part 2 sum: {}", get_sum_of_input(aoc_input, true));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibration_value_numbers() {
        assert_eq!(get_calibration_value("1abc2", false), 12);
        assert_eq!(get_calibration_value("pqr3stu8vwx", false), 38);
        assert_eq!(get_calibration_value("a1b2c3d4e5f", false), 15);
        assert_eq!(get_calibration_value("treb7uchet", false), 77);
    }

    #[test]
    fn calibration_value_text() {
        assert_eq!(get_calibration_value("onetwo", true), 12);
        assert_eq!(get_calibration_value("twone", true), 21);
    }

    #[test]
    fn test_input_sum() {
        let input = "onetwo
            twone";
        assert_eq!(get_sum_of_input(input, true), 33);
    }
}
