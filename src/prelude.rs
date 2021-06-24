//! Major packages used in this simulation has `prelude`-modules and these
//! are all conjoined here.
//!
//!
pub use bevy::prelude::*;

pub use itertools::Itertools;
#[cfg(feature = "serialize")]
pub use serde::{Deserialize, Serialize};

pub use rand::prelude::*;
pub use rand::SeedableRng;

pub use anyhow::Result;

//TODO: add [bevy::ecs::system::SystemParam] as I believe this should
// be included in the prelude, but it isn't.
