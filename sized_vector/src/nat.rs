//! A naive type-level representation of natural numbers.
//!
//! A natural number N is either Zero (i.e. 0) or the Succ<N> (i.e. 1 + N).

use std::marker::PhantomData;

pub trait Nat: private::Sealed {
    type Add<M: Nat>: Nat;

    fn reify() -> usize;
}

pub type Add<N, M> = <N as Nat>::Add<M>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Zero;

impl Nat for Zero {
    type Add<M: Nat> = M;

    fn reify() -> usize {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Succ<N: Nat>(PhantomData<N>);

impl<N: Nat> Nat for Succ<N> {
    type Add<M: Nat> = Succ<N>;

    fn reify() -> usize {
        1 + N::reify()
    }
}

mod private {
    use super::{Nat, Succ, Zero};

    pub trait Sealed {}

    impl Sealed for Zero {}

    impl<N: Nat> Sealed for Succ<N> {}
}
