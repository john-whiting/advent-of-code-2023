use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct MapInput(pub u32, pub u32, pub u32);
pub struct MapInputs(pub Vec<MapInput>);

pub trait SingleTuple {
    fn from_tuple(tuple: (u32,)) -> Self;
    fn to_tuple(&self) -> (u32,);
}

macro_rules! impl_single_tuple {
    ($name:ty) => {
        impl SingleTuple for $name {
            fn from_tuple(tuple: (u32,)) -> Self {
                Self(tuple.0)
            }

            fn to_tuple(&self) -> (u32,) {
                (self.0,)
            }
        }
    };
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Seed(pub u32);
impl_single_tuple!(Seed);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Soil(pub u32);
impl_single_tuple!(Soil);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Fertilizer(pub u32);
impl_single_tuple!(Fertilizer);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Water(pub u32);
impl_single_tuple!(Water);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Light(pub u32);
impl_single_tuple!(Light);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Temperature(pub u32);
impl_single_tuple!(Temperature);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Humidity(pub u32);
impl_single_tuple!(Humidity);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Location(pub u32);
impl_single_tuple!(Location);

impl<F, T> From<MapInputs> for HashMap<F, T>
where
    F: SingleTuple + Eq + std::hash::Hash,
    T: SingleTuple + Eq + std::hash::Hash,
{
    fn from(val: MapInputs) -> Self {
        let MapInputs(inputs) = val;

        inputs
            .iter()
            .flat_map(|input| {
                (input.1..(input.1 + input.2))
                    .map(|num| F::from_tuple((num,)))
                    .zip((input.0..(input.0 + input.2)).map(|num| T::from_tuple((num,))))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::info::*;

    #[test]
    fn from_tuple() {
        assert_eq!(Seed::from_tuple((24,)), Seed(24));
        assert_eq!(Soil::from_tuple((24,)), Soil(24));
        assert_eq!(Fertilizer::from_tuple((24,)), Fertilizer(24));
        assert_eq!(Water::from_tuple((24,)), Water(24));
        assert_eq!(Light::from_tuple((24,)), Light(24));
        assert_eq!(Temperature::from_tuple((24,)), Temperature(24));
        assert_eq!(Humidity::from_tuple((24,)), Humidity(24));
        assert_eq!(Location::from_tuple((24,)), Location(24));
    }

    #[test]
    fn to_tuple() {
        assert_eq!(Seed(24).to_tuple(), (24,));
        assert_eq!(Soil(24).to_tuple(), (24,));
        assert_eq!(Fertilizer(24).to_tuple(), (24,));
        assert_eq!(Water(24).to_tuple(), (24,));
        assert_eq!(Light(24).to_tuple(), (24,));
        assert_eq!(Temperature(24).to_tuple(), (24,));
        assert_eq!(Humidity(24).to_tuple(), (24,));
        assert_eq!(Location(24).to_tuple(), (24,));
    }

    #[test]
    fn map_inputs_to_hash_map() {
        let hash_map: HashMap<Seed, Soil> = MapInputs(vec![MapInput(50, 98, 2)]).into();
        let expected = [(Seed(98), Soil(50)), (Seed(99), Soil(51))];
        assert_eq!(hash_map, expected.into_iter().collect::<HashMap<_, _>>())
    }
}
