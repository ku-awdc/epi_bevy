//!
//!

#[readonly::make]
#[derive(Debug)]
pub struct FarmId(pub usize);

#[cfg_attr(features = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
struct CattlePopulationRecord {
    // [serde(rename)]
    farm_ids: Vec<FarmId>,
    herd_sizes: Vec<usize>,
    adjacent_farms: Vec<Vec<FarmId>>,
}

// #[cfg_attr(
//     features = "serialize",
//     derive(serde::Serialize, serde::Deserialize, serde::DeserializeOwned)
// )]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// #[serde(deny_unknown_fields)]
struct Record {
    farm_id: usize,
    herd_size: usize,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    #[cfg(feature = "serialize")]
    fn test_loading_ring_population() {
        let population_info_file = File::open("assets/population_info.json").unwrap();
        let population_info_reader = BufReader::with_capacity(100_000_000, population_info_file);

        let pop_record: Vec<Record> = serde_json::from_reader(population_info_reader).unwrap();
        dbg!(pop_record);
    }
}
