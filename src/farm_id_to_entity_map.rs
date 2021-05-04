use crate::cattle_population::FarmId;
use crate::prelude::*;
use std::collections::HashMap;

//TODO: maybe this should implement Default as a panic?
/// A mapping between the ECS's entity id's for the farms, and the inherent
/// id numbering that comes from the scenario configuration (input files, etc.)
#[readonly::make]
#[derive(Debug, derive_more::Into, derive_more::From)]
pub struct FarmIdEntityMap(pub HashMap<FarmId, Entity>);
