use day_03::grid::Grid;
use itertools::Itertools;

fn process_part1(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.cells()
        .filter_map(|cell| {
            if !cell.value.is_ascii_digit() {
                return None;
            }

            if !cell
                .neighbors()
                .any(|c| !c.value.is_ascii_digit() && c.value != '.')
            {
                return None;
            }

            cell.part_number()
        })
        .unique()
        .map(|part_number| part_number.1)
        .sum::<usize>()
}

fn process_part2(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.cells()
        .filter_map(|cell| {
            if cell.value != '*' {
                return None;
            }

            let neighboring_parts = cell
                .neighbors()
                .filter_map(|neighbor| neighbor.part_number())
                .unique();

            if neighboring_parts.clone().count() != 2 {
                return None;
            }

            Some(
                neighboring_parts
                    .map(|part_number| part_number.1)
                    .product::<usize>(),
            )
        })
        .sum::<usize>()
}

fn main() {
    let aoc_input = include_str!("./input.txt");
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
        let schematic = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";
        assert_eq!(process_part1(schematic), 4361);
    }

    #[test]
    fn test_part2() {
        let schematic = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";
        assert_eq!(process_part2(schematic), 467835);
    }
}
