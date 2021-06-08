//!
//!
//!
use crate::populations::{AdjacentFarms, Cattle, FarmId, HerdSize};
use crate::prelude::*;


#[cfg(feature = "serialize")]
pub fn deserialize_generated_farm_id<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<FarmId, D::Error> {
    // let n: usize = usize::deserialize(deserializer)?;
    let n: usize = serde::Deserialize::deserialize(deserializer)?;
    Ok(FarmId::new_single_population(n))
}

#[cfg(feature = "serialize")]
pub fn deserialize_generated_herd_size<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<HerdSize, D::Error> {
    // let n: usize = usize::deserialize(deserializer)?;
    let n: usize = serde::Deserialize::deserialize(deserializer)?;
    Ok(HerdSize::new_single_population(n))
}

#[cfg(feature = "serialize")]
pub fn deserialize_generated_adjacent_farms<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<AdjacentFarms, D::Error> {
    // let n: usize = usize::deserialize(deserializer)?;
    let n: Vec<usize> = serde::Deserialize::deserialize(deserializer)?;
    // dbg!(&n);
    Ok(AdjacentFarms::new_single_population(
        n.into_iter()
            .map(FarmId::new_single_population)
            .collect(),
    ))
}

#[readonly::make]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Bundle)]
pub struct CattleFarmBundle {
    cattle_farm: Cattle,
    #[serde(deserialize_with = "deserialize_generated_farm_id")]
    pub farm_id: FarmId,
    #[serde(deserialize_with = "deserialize_generated_herd_size")]
    pub herd_size: HerdSize,
    adjacent_farms: AdjacentFarms,
}

#[cfg(feature = "serialize")]
pub fn load_ring_population() -> impl Iterator<Item = CattleFarmBundle> + Clone {
    #[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    struct PopulationRecord {
        #[serde(deserialize_with = "deserialize_generated_farm_id")]
        farm_id: FarmId,
        #[serde(deserialize_with = "deserialize_generated_herd_size")]
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
        #[serde(deserialize_with = "deserialize_generated_farm_id")]
        farm_id: FarmId,
        #[serde(rename(deserialize = "adjacent"))]
        #[serde(deserialize_with = "deserialize_generated_adjacent_farms")]
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
            cattle_farm: Cattle,
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
