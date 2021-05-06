//! Parameters used in distributions, simulation modules, etc. are defined
//! here.
//!
//! These ensure the conversion between different parameter types follow
//! the right formulas.
//!
//!
//!
use anyhow::Result;
use std::convert::{TryFrom, TryInto};
use thiserror::Error;

#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
pub struct Rate(pub f64);

impl Rate {
    pub fn new(value: f64) -> Result<Self> {
        Ok(value.try_into()?)
    }
}

#[readonly::make]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, derive_more::Display, derive_more::Into)]
pub struct Probability(pub f64);

impl Probability {
    pub fn new(value: f64) -> Result<Self> {
        Ok(value.try_into()?)
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

// TODO: Combining/Compounding parameters is not really that trivial.

// impl Rate {
//     /// See [Probability::compound] for details.
//     pub fn compound<const N: usize>(rates: [Self; N]) -> Self {
//         let probabilities: [Probability; N] = rates.into();
//         todo!()
//         // std::array::IntoIter::new(rates)
//         //     .map_into::<Probability>()

//     }
// }
