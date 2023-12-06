use day_05::{
    almanac::Location,
    parsing::{mapped_inputs, seeds},
};

fn process_part1(input: &str) -> u64 {
    let (input, seeds) = seeds(input).expect("should contain seeds");
    let (input, seed_to_soil) =
        mapped_inputs("seed-to-soil map:")(input).expect("seed-to-soil map");
    let (input, soil_to_fert) =
        mapped_inputs("soil-to-fertilizer map:")(input).expect("seed-to-soil map");
    let (input, fert_to_water) =
        mapped_inputs("fertilizer-to-water map:")(input).expect("seed-to-soil map");
    let (input, water_to_light) =
        mapped_inputs("water-to-light map:")(input).expect("seed-to-soil map");
    let (input, light_to_temp) =
        mapped_inputs("light-to-temperature map:")(input).expect("seed-to-soil map");
    let (input, temp_to_humidity) =
        mapped_inputs("temperature-to-humidity map:")(input).expect("seed-to-soil map");
    let (_, humid_to_locs) =
        mapped_inputs("humidity-to-location map:")(input).expect("seed-to-soil map");

    let soils: Vec<_> = seeds.iter().map(|it| it.next(&seed_to_soil)).collect();
    let ferts: Vec<_> = soils.iter().map(|it| it.next(&soil_to_fert)).collect();
    let waters: Vec<_> = ferts.iter().map(|it| it.next(&fert_to_water)).collect();
    let lights: Vec<_> = waters.iter().map(|it| it.next(&water_to_light)).collect();
    let temps: Vec<_> = lights.iter().map(|it| it.next(&light_to_temp)).collect();
    let humids: Vec<_> = temps.iter().map(|it| it.next(&temp_to_humidity)).collect();
    let locations: Vec<_> = humids.iter().map(|it| it.next(&humid_to_locs)).collect();

    locations
        .into_iter()
        .map(|location| match location {
            Location(x) => x,
        })
        .min()
        .expect("should be min")
}

fn main() {
    let aoc_input = include_str!("input.txt");
    let part1_solution = process_part1(aoc_input);
    // let part2_solution = process_part2(aoc_input);

    println!("Part 1 Solution: {part1_solution}");
    // println!("Part 2 Solution: {part2_solution}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let schematic = "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4";
        assert_eq!(process_part1(schematic), 35);
    }
}
