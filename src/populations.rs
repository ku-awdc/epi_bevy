//!
//! Use `()` as the default [Population] if the multiple populations are not
//! the target.
//!
use std::{hash::Hash, marker::PhantomData};

use bevy::ecs::component::Component;

// ! This is (supposed) to replace [crate::cattle_population].

// TODO: Create a macro that generates a lot of this.

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum EmbeddedPopulation {
    Default(()),
    Cattle(Cattle),
    Pig(Pig),
    Sheep(Sheep),
}

// this currently doesn't work. Nor does the `either` crate help with this.

// impl EmbeddedPopulation {
//     pub fn inner(self) -> Box<dyn Population> {
//         match self {
//             EmbeddedPopulation::Default(a) => a,
//             EmbeddedPopulation::Cattle(a) => a,
//             EmbeddedPopulation::Pig(a) => a,
//             EmbeddedPopulation::Sheep(a) => a,
//         }
//     }
// }

pub trait Population: Component + Hash + Eq + PartialEq {}

impl Population for () {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Cattle;

impl Population for Cattle {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Pig;

impl Population for Pig {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Sheep;

impl Population for Sheep {}

// #[readonly::make]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Bundle)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct FarmBundle<P: Population = ()> {
    population: P,
    pub farm_id: FarmId<P>,
    pub herd_size: HerdSize<P>,
    adjacent_farms: AdjacentFarms<P>,
}

// #[readonly::make]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct FarmId<P: Population = ()>(
    pub usize,
    // #[serde(default)]
    #[serde(skip_deserializing, skip_serializing)]
    PhantomData<P>,
);

#[readonly::make]
#[derive(Debug, Clone, Copy, derive_new::new, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HerdSize<P: Population = ()>(pub usize, #[serde(skip_deserializing)] PhantomData<P>);

#[readonly::make]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AdjacentFarms<P: Population = ()>(pub Vec<FarmId<P>>, PhantomData<P>);

/// Intended to be stored as a global available resource
/// for each added population to the [crate::scenario_builder::Scenario].
#[readonly::make]
#[derive(Debug, Clone, Copy, derive_new::new, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct TotalFarms<P: Population = ()>(pub usize, PhantomData<P>);

//TODO: this part could be a derive macro..

impl TotalFarms {
    pub fn new_single_population(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

impl<P: Population> AdjacentFarms<P> {
    pub fn new_single_population(value: Vec<FarmId<P>>) -> Self {
        Self(value, PhantomData)
    }
}

impl HerdSize {
    pub fn new_single_population(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

impl<P: Population> FarmId<P> {
    pub fn new_single_population(value: usize) -> Self {
        Self(value, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_farms() {
        let mut world = World::new();

        let total_sheep = TotalFarms::<Sheep>::new(80);
        let total_cattle = TotalFarms::<Cattle>::new(140);
        let total_pig = TotalFarms::<Pig>::new(350);

        // world.insert_resource((total_sheep, total_cattle, total_pig));
        world.insert_resource(total_sheep);
        world.insert_resource(total_cattle);
        world.insert_resource(total_pig);

        dbg!(world.get_resource::<TotalFarms<Sheep>>().unwrap());
        dbg!(world.get_resource::<TotalFarms<Cattle>>().unwrap());
        dbg!(world.get_resource::<TotalFarms<Pig>>().unwrap());

        // Alright, next add a herd-size that isn't tied to any one particular
        // population
        world.insert_resource(HerdSize::<()>::new(1000));

        dbg!(world.get_resource::<HerdSize>().unwrap());
    }

    #[test]
    fn test_default_population() {
        let mut world = World::new();

        let total_farms = TotalFarms::new_single_population(123);

        world.insert_resource(total_farms);

        dbg!(world.get_resource::<TotalFarms>().unwrap());
    }
}
