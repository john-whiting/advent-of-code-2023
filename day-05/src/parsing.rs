use nom::{
    character::complete::{self, multispace1, space1},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{parser_ext::ParserExt, tag::complete::tag};

use crate::info::{MapInput, Seed};

pub fn seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    tag("seeds: ")
        .precedes(separated_list1(space1, complete::u32.map(Seed)))
        .parse(input)
}

pub fn mapped_input(input: &str) -> IResult<&str, MapInput> {
    tuple((
        complete::u32,
        space1.precedes(complete::u32),
        space1.precedes(complete::u32),
    ))
    .map(|(dst, src, count)| MapInput(dst, src, count))
    .parse(input)
}

pub fn mapped_inputs(header: &'static str) -> impl Fn(&str) -> IResult<&str, Vec<MapInput>> {
    move |input: &str| {
        tuple((multispace1, tag(header), multispace1))
            .precedes(separated_list1(multispace1, mapped_input))
            .parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsing::*;

    #[test]
    fn parsing() {
        let input = "seeds: 79 14 55 13

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

        let out = seeds(input);

        assert!(
            matches!(&out, Ok((_, found_seeds)) if *found_seeds == vec![Seed(79), Seed(14), Seed(55), Seed(13)])
        );

        let out = mapped_inputs("seed-to-soil map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(50, 98, 2), MapInput(52, 50, 48)])
        );

        let out = mapped_inputs("soil-to-fertilizer map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(0, 15, 37), MapInput(37, 52, 2), MapInput(39, 0, 15)])
        );

        let out = mapped_inputs("fertilizer-to-water map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(49, 53, 8), MapInput(0, 11, 42), MapInput(42, 0, 7), MapInput(57, 7, 4)])
        );

        let out = mapped_inputs("water-to-light map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(88, 18, 7), MapInput(18, 25, 70)])
        );

        let out = mapped_inputs("light-to-temperature map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(45, 77, 23), MapInput(81, 45, 19), MapInput(68, 64, 13)])
        );

        let out = mapped_inputs("temperature-to-humidity map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(0, 69, 1), MapInput(1, 0, 69)])
        );

        let out = mapped_inputs("humidity-to-location map:")(out.unwrap().0);

        assert!(
            matches!(&out, Ok((_, found_inputs)) if *found_inputs == vec![MapInput(60, 56, 37), MapInput(56, 93, 4)])
        );
    }
}
