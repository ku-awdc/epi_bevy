//! Mathematical tools that are typically available in R/Python etc.
//!

// spell-checker: words stoch

use num_traits::Float;
use rand::Rng;

pub trait FloatExt: Float {
    /// Round to the nearest integer through the sampling the fractional part.
    fn round_stoch(self, rng: &mut impl Rng) -> Self
    where
        Self: Sized,
    {
        if rng.gen_bool(self.fract().to_f64().unwrap()) {
            self.ceil()
        } else {
            self.floor()
        }
    }
}

impl FloatExt for f32 {}
impl FloatExt for f64 {}

#[cfg(test)]
mod tests {
    use rand::{prelude::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn round_stochastically() {
        let mut rng = StdRng::seed_from_u64(202105);
        dbg!(1.2.round_stoch(&mut rng));
        dbg!(1.8.round_stoch(&mut rng));
    }
}
