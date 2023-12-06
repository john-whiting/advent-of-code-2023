use std::{iter::Map, ops::Range};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SeedRange(pub u64, pub u64);

impl SeedRange {
    // NOTE: This is *going* to be slow, should optimize later
    pub fn seeds(&self) -> Map<Range<u64>, fn(u64) -> Seed> {
        (self.0..(self.0 + self.1)).map(Seed)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct MapInput(pub u64, pub u64, pub u64);

macro_rules! impl_almanac_property {
    ($from:tt) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        pub struct $from(pub u64);

        impl $from {
            fn new(value: u64) -> Self {
                Self(value)
            }
        }
    };
    ($from:tt, $to:ty) => {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        pub struct $from(pub u64);

        impl $from {
            #[allow(dead_code)]
            fn new(value: u64) -> Self {
                Self(value)
            }

            fn next_number(&self, input: MapInput) -> Option<u64> {
                let num = self.0;
                let range = input.1..(input.1 + input.2);

                // Check that it is in range
                if !(range.start <= num && num < range.end) {
                    return None;
                }

                Some(input.0 + (num - range.start))
            }

            pub fn next(&self, inputs: &[MapInput]) -> $to {
                match inputs.iter().find_map(|input| self.next_number(*input)) {
                    Some(x) => <$to>::new(x),
                    None => <$to>::new(self.0),
                }
            }
        }
    };
}

impl_almanac_property!(Seed, Soil);
impl_almanac_property!(Soil, Fertilizer);
impl_almanac_property!(Fertilizer, Water);
impl_almanac_property!(Water, Light);
impl_almanac_property!(Light, Temperature);
impl_almanac_property!(Temperature, Humidity);
impl_almanac_property!(Humidity, Location);
impl_almanac_property!(Location);
