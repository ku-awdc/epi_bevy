use crate::prelude::*;

/// Used to fuse the chained systems, such that one doesn't get a compilation
/// error even though the result of the last system isn't used.
///
/// As [SystemStage] requires that the system-chain ends with unit-type `()`
/// we have to use this to fuse the chain.
///
/// Author: @TheRuwuMeatball
pub fn dispose<T>(_: In<T>) {}
