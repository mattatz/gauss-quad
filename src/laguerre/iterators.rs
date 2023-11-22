//! This module contains the iterators produced by some of the functions on [`GaussLaguerre`].

use super::GaussLaguerre;
use crate::{impl_iterators, slice_map_iter_impl};

impl_iterators! {GaussLaguerre, GaussLaguerreNodes, GaussLaguerreWeights, GaussLaguerreIter, GaussLaguerreIntoIter}
