//! A representation of a non-structural, length-index vector.
//!
//! Length invariants are not enforced by construction, hence internal correct depends on careful implementation.
//! However, users might profit from API-provided properties.

use crate::nat::{Add, Nat, Succ, Zero};
use std::marker::PhantomData;

/// A wrapper around a `Vec<T>` with a length encoded as a natural number at the type-level.
///
/// Notice that there is **no** strong relationship between the inner vector and the length,
/// hence it's up to the implementer the responsibility to uphold such a constraint.
/// For an encoding where elements and length are structurally coupled, you may refer to (example expressed in Haskell with GADTs):
/// https://github.com/rvarago/haskell-playground/blob/4e90bef9109b4ab424ba41208959e5350a0eddc4/Vec/Main.hs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IVec<N: Nat, T> {
    entries: Vec<T>,
    len: PhantomData<N>,
}

impl<T> Default for IVec<Zero, T> {
    fn default() -> Self {
        Self::unverified_new(Vec::default())
    }
}

impl<N: Nat, A> IVec<N, A> {
    /// Pushes a new element to a vector of size N to yield a new vector of size N + 1.
    pub fn push(mut self, entry: A) -> IVec<Succ<N>, A> {
        self.entries.push(entry);
        IVec::unverified_new(self.entries)
    }

    /// Appends a vector of size M to a vector of size N to yield a new vector of size N + M.
    pub fn append<M: Nat>(mut self, mut rhs: IVec<M, A>) -> IVec<Add<N, M>, A> {
        self.entries.append(&mut rhs.entries);
        IVec::unverified_new(self.entries)
    }

    /// Computes the underlying number of elements.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Checks whether there's no element.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Maps a vector into a new vector changing type but preserving length.
    pub fn map<F, B>(self, f: F) -> IVec<N, B>
    where
        F: FnMut(A) -> B,
    {
        let entries = self.entries.into_iter().map(f).collect();
        IVec::unverified_new(entries)
    }

    /// Zips two vectors of size N to yield a new vector of pairs.
    pub fn zip<B>(self, rhs: IVec<N, B>) -> IVec<N, (A, B)> {
        let entries = self.entries.into_iter().zip(rhs.entries).collect();
        IVec::unverified_new(entries)
    }

    /// Folds a vector into a summary value.
    pub fn fold<F, B>(self, init: B, f: F) -> B
    where
        F: FnMut(B, A) -> B,
    {
        self.entries.into_iter().fold(init, f)
    }

    /// Constructs a new vector from its inner entries.
    ///
    /// # Important
    ///
    /// **Assumes** length is correct at usage site.
    fn unverified_new(entries: Vec<A>) -> Self {
        Self {
            entries,
            len: PhantomData::default(),
        }
    }
}

impl<N: Nat, A> IVec<Succ<N>, A> {
    /// Accesses the first element of a non-empty vector.
    pub fn first(&self) -> &A {
        self.entries
            .first()
            .expect("a IVec<Succ<N>, A> must have at least one element")
    }
}
