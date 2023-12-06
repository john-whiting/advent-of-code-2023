use std::{collections::HashMap, ops::Range};

use day_05::{
    info::{
        Fertilizer, Humidity, Light, Location, MapInput, MapInputs, SingleTuple, Soil, Temperature,
        Water,
    },
    parsing::{mapped_inputs, seeds},
};

trait MyContains<T> {
    fn my_contains(&self, ele: T) -> bool;
}

impl<T: SingleTuple> MyContains<T> for Range<u64> {
    fn my_contains(&self, ele: T) -> bool {
        let num = ele.to_tuple().0;
        self.start <= num && num < self.end
    }
}

fn map_to_next<F: SingleTuple + Copy, T: SingleTuple>(
    inputs: Vec<MapInput>,
    prev: Vec<F>,
) -> Vec<T> {
    prev.iter()
        .map(|tup| {
            match (inputs
                .iter()
                .filter_map(|input| {
                    match ((input.1..(input.1 + input.2)) as Range<u64>).my_contains(*tup) {
                        true => Some((input.0 + (tup.to_tuple().0 - input.1),)),
                        false => None,
                    }
                })
                .next())
            {
                Some(x) => T::from_tuple(x),
                None => T::from_tuple(tup.to_tuple()),
            }
        })
        .collect()
}

fn map_to_next2<
    F: day_05::info::SingleTuple + Eq + std::hash::Hash,
    T: day_05::info::SingleTuple + Eq + std::hash::Hash + Copy,
>(
    inputs: Vec<MapInput>,
    prev: Vec<F>,
) -> Vec<T> {
    let hash_map: HashMap<F, T> = MapInputs(inputs).into();
    prev.iter()
        .map(|p| match hash_map.get(p) {
            Some(x) => *x,
            None => T::from_tuple(p.to_tuple()),
        })
        .collect()
}

fn process_part1(input: &str) -> u64 {
    let (input, seeds) = seeds(input).expect("should contain seeds");
    let (input, seed_to_soil) =
        mapped_inputs("seed-to-soil map:")(input).expect("seed-to-soil map");
    let (input, soil_to_fertilizer) =
        mapped_inputs("soil-to-fertilizer map:")(input).expect("seed-to-soil map");
    let (input, fertilizer_to_water) =
        mapped_inputs("fertilizer-to-water map:")(input).expect("seed-to-soil map");
    let (input, water_to_light) =
        mapped_inputs("water-to-light map:")(input).expect("seed-to-soil map");
    let (input, light_to_temperature) =
        mapped_inputs("light-to-temperature map:")(input).expect("seed-to-soil map");
    let (input, temp_to_humidity) =
        mapped_inputs("temperature-to-humidity map:")(input).expect("seed-to-soil map");
    let (_, humidity_to_location) =
        mapped_inputs("humidity-to-location map:")(input).expect("seed-to-soil map");

    let soils: Vec<Soil> = map_to_next(seed_to_soil, seeds);
    let fertilizers: Vec<Fertilizer> = map_to_next(soil_to_fertilizer, soils);
    let waters: Vec<Water> = map_to_next(fertilizer_to_water, fertilizers);
    let lights: Vec<Light> = map_to_next(water_to_light, waters);
    let temperatures: Vec<Temperature> = map_to_next(light_to_temperature, lights);
    let humidities: Vec<Humidity> = map_to_next(temp_to_humidity, temperatures);
    let locations: Vec<Location> = map_to_next(humidity_to_location, humidities);

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
