//!
//!
//!
use bevy::prelude::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

use itertools::Itertools;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CattleFarm;

#[readonly::make]
#[derive(Debug, Copy, Clone, PartialEq, derive_more::Display, Hash, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct FarmId(pub usize);

#[readonly::make]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Bundle)]
pub struct CattleFarmBundle {
    cattle_farm: CattleFarm,
    pub farm_id: FarmId,
    pub herd_size: HerdSize,
    adjacent_farms: AdjacentFarms,
}

#[readonly::make]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct HerdSize(pub usize);

#[readonly::make]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AdjacentFarms(pub Vec<FarmId>);

#[cfg(feature = "serialize")]
pub fn load_ring_population() -> impl Iterator<Item = CattleFarmBundle> + Clone {
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    struct PopulationRecord {
        farm_id: FarmId,
        herd_size: HerdSize,
    }
    let population_info_file = std::fs::File::open("assets/population_info.json").unwrap();
    let population_info_reader =
        std::io::BufReader::with_capacity(100_000_000, population_info_file);

    let pop_record: Vec<PopulationRecord> =
        serde_json::from_reader(population_info_reader).unwrap();

    // dbg!(pop_record.iter().take(10).collect_vec());

    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    struct AdjacencyRecord {
        farm_id: FarmId,
        #[serde(rename(deserialize = "adjacent"))]
        adjacent_farms: AdjacentFarms,
    }

    let adjacency = std::fs::File::open("assets/ring_adjacency.json").unwrap();
    let adjacency_file_buffer = std::io::BufReader::with_capacity(100_000_000, adjacency);

    let adjacency: Vec<AdjacencyRecord> = serde_json::from_reader(adjacency_file_buffer).unwrap();

    // dbg!(adjacency.iter().take(10).collect_vec());

    pop_record.into_iter().zip_eq(adjacency).map(|(info, adj)| {
        let PopulationRecord {
            farm_id: pop_farm_id,
            herd_size,
        } = info;
        let AdjacencyRecord {
            farm_id,
            adjacent_farms,
        } = adj;
        assert_eq!(
            pop_farm_id, farm_id,
            "farm id from adjacency and from population info should match"
        );

        CattleFarmBundle {
            cattle_farm: CattleFarm,
            farm_id,
            herd_size,
            adjacent_farms,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serialize")]
    fn test_loading_ring_population() {
        let iter_cattle_farm_bundle = load_ring_population();

        info!("{:#?}", iter_cattle_farm_bundle.take(10).collect_vec());
    }
}
