//!TODO: A general problem occurs in the operations themselves and type-safety
//! is that I want there to be a well-defined operations between
//! fairly distinct quantities. `Compartment x Parameters` are not clearly
//! defined anywhere.
//! Thus things like [sir_spread_model::Susceptible] x [parameters::rate] are
//! not defined in code. Thus, everywhere this happens atm, we are accessing
//! the inner-part of these struct. Revise this by adding traits on parameter and
//! compartment.
//!
//!
// use crate::parameters::Rate;

// pub trait Compartment {
//     pub type Output: Self;

//     fn value(&self) -> f64;
// }

// impl std::ops::Mul<Rate> for Compartment {
//     type Output = Rate;

//     fn mul(self, rhs: Rate) -> Self::Output {
//         (self.value() * rhs.0)
//     }
// }
