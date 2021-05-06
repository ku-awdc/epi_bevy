//! Mathematical tools that are typically available in R/Python etc. 
//! 

use std::fmt::Debug;

use num_traits::{Float, ToPrimitive};
use rand::Rng;

/// Round to the nearest integer through the sampling the fractional part.
pub fn round_stoch(value: impl Float + ToPrimitive + Debug, rng: &mut impl Rng) -> impl Float + Debug{
    if rng.gen_bool(value.fract().to_f64().unwrap()) {
        value.ceil()
    } else {
        value.floor()
    }
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, prelude::StdRng};

    use super::*;

    #[test]
    fn round_stochastically() {
        let mut rng = StdRng::seed_from_u64(202105);
        dbg!(round_stoch(1.2, &mut rng));
        dbg!(round_stoch(1.8, &mut rng));
        
    }
}