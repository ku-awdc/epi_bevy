//!
//!
use std::marker::PhantomData;

use bevy::ecs::component::Component;

// ! This is (supposed) to replace [crate::cattle_population].

use crate::prelude::*;

pub trait Population: Component {}

impl Population for () {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Cattle;

impl Population for Cattle {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Pig;

impl Population for Pig {}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Sheep;

impl Population for Sheep {}

// #[readonly::make]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Bundle)]
pub struct FarmBundle<P: Population> {
    population: P,
    pub farm_id: FarmId<P>,
    pub herd_size: HerdSize<P>,
    adjacent_farms: AdjacentFarms<P>,
}

// #[readonly::make]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct FarmId<P: Population>(pub usize, PhantomData<P>);

#[readonly::make]
#[derive(Debug, Clone, Copy, derive_new::new)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HerdSize<P: Population = ()>(pub usize, PhantomData<P>);

#[readonly::make]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AdjacentFarms<P: Population>(pub Vec<FarmId<P>>);

/// Intended to be stored as a global available resource
/// for each added population to the [crate::scenario_builder::Scenario].
#[readonly::make]
#[derive(Debug, Clone, Copy, derive_new::new)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct TotalFarms<P: Population>(pub usize, PhantomData<P>);

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
}
