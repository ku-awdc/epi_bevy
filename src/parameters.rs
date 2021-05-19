//! Parameters used in distributions, simulation modules, etc. are defined
//! here.
//!
//! These ensure the conversion between different parameter types follow
//! the right formulas.
//!
//!
//!
use anyhow::Result;
use std::{
    array::IntoIter,
    convert::{TryFrom, TryInto},
    ops::Neg,
};
use thiserror::Error;

#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug, Clone, Copy, derive_more::Into, derive_more::Display, derive_more::Add, derive_more::Sum,
)]
pub struct Rate(pub f64);

impl Rate {
    pub fn new(value: f64) -> Result<Self> {
        Ok(value.try_into()?)
    }
}

// #[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug,
    Clone,
    Copy,
    derive_more::Display,
    derive_more::Into,
    // derive_more::MulSelf,
    // derive_more::Mul, // doesn't work with product
    // derive_more::Product,
)]
pub struct Probability(pub f64);

impl Probability {
    pub fn new(value: f64) -> Result<Self> {
        Ok(value.try_into()?)
    }

    pub fn complement(mut self) -> Self {
        self.0 = 1. - self.0;
        self
    }
}

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("negative float is not a valid rate")]
    NegativeRate,
    #[error("float is not between 0 and 1, thus not a valid probability")]
    NotInbetweenZeroAndOne,
}

impl TryFrom<f64> for Rate {
    type Error = ConversionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0. {
            Err(Self::Error::NegativeRate)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<f64> for Probability {
    type Error = ConversionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !(0_f64..=1.).contains(&value) {
            Err(Self::Error::NotInbetweenZeroAndOne)
        } else {
            Ok(Self(value))
        }
    }
}

/// $\lambda = -\log(1-p)$
impl From<Probability> for Rate {
    fn from(probability: Probability) -> Self {
        Self(-(1. - probability.0).ln())
    }
}

/// $p=1-\exp(-\lambda)$
impl From<Rate> for Probability {
    fn from(rate: Rate) -> Self {
        Self(1. - (-rate.0).exp())
    }
}

impl Probability {
    // but this doesn't really work...
    // pub fn compound(self, other: Self) -> Self {
    //     Self((1. - self.0) * (1. - other.0))
    // }

    /// Composition is done according to [Poisson binomial distribution](https://www.wikiwand.com/en/Poisson_binomial_distribution)
    ///
    pub fn compound<const N: usize>(probabilities: [Self; N]) -> Self {
        Self(
            1. - std::array::IntoIter::new(probabilities)
                .map(|x| 1. - x.0)
                .product::<f64>(),
        )
    }
}

///
///
/// Formula is: probability = 1 - exp(-\sum_i \lambda_i)
pub fn compound_rates<const N: usize>(rates: [Rate; N]) -> Probability {
    Probability::try_from(1. - IntoIter::new(rates).sum::<Rate>().0.neg().exp()).unwrap()
}

/// This is less numerically efficient/precise than [compound_rates], but it
/// should yield the same result.
fn compound_probabilities<const N: usize>(probabilities: [Probability; N]) -> Probability {
    IntoIter::new(probabilities)
        .map(|p| p.complement())
        // .product::<Probability>() // use this instead when derive_more works
        // with Product and Mul
        .fold(Probability::new(1.).unwrap(), |mut acc, x| {
            acc.0 *= x.0;
            acc
        })
        .complement()
}
