//! A representation of inductively defined heterogeneous list.

use super::{
    boolean::{Boolean, False, True},
    nat::Nat,
};
use std::marker::PhantomData;

pub trait HList: private::Sealed {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HNil;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HCons<Head, Tail: HList>(pub Head, pub Tail);

impl HList for HNil {}

impl<Head, Tail: HList> HList for HCons<Head, Tail> {}

impl<Head, Tail: HList> HCons<Head, Tail> {
    pub fn first(&self) -> &Head {
        &self.0
    }
}

macro_rules! hlist {
    () => {
        HNil
    };

    ($head:expr) => {
        HCons($head, hlist![])
    };

    ($head:expr,$($tail:tt)*) => {
        HCons($head, hlist![$($tail)*])
    };
}

pub trait Sized: HList {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Sized for HNil {
    fn len(&self) -> usize {
        0
    }
}

impl<Head, Tail: Sized> Sized for HCons<Head, Tail> {
    fn len(&self) -> usize {
        1 + self.1.len()
    }
}

// FIXME: Make this place nice with type-inference.
pub trait Contains<Target, Index: Control>: HList {
    type Out: Boolean;

    fn contains(&self) -> Self::Out;
}

pub fn contains<Target, Index: Control, List: Contains<Target, Index>>(list: List) -> List::Out {
    list.contains()
}

impl<Target, Index: Control> Contains<Target, Index> for HNil {
    type Out = False;

    fn contains(&self) -> Self::Out {
        False
    }
}

impl<Target, Tail: HList> Contains<Target, Break> for HCons<Target, Tail> {
    type Out = True;

    fn contains(&self) -> Self::Out {
        True
    }
}

impl<Target, TailIndex, Head, Tail> Contains<Target, Continue<TailIndex>> for HCons<Head, Tail>
where
    TailIndex: Control,
    Tail: Contains<Target, TailIndex>,
{
    type Out = Tail::Out;

    fn contains(&self) -> Self::Out {
        self.1.contains()
    }
}

pub trait Extract<Target, Index: Control>: HList {
    type Rest: HList;

    fn extract(self) -> (Target, Self::Rest);
}

impl<Target, Tail: HList> Extract<Target, Break> for HCons<Target, Tail> {
    type Rest = Tail;

    fn extract(self) -> (Target, Self::Rest) {
        (self.0, self.1)
    }
}

impl<Target, TailIndex, Head, Tail> Extract<Target, Continue<TailIndex>> for HCons<Head, Tail>
where
    TailIndex: Control,
    Tail: Extract<Target, TailIndex>,
{
    type Rest = HCons<Head, Tail::Rest>;

    fn extract(self) -> (Target, Self::Rest) {
        let (head, tail) = (self.0, self.1);
        let (target, rest) = tail.extract();
        (target, HCons(head, rest))
    }
}

pub trait Map<Functions: HList>: HList {
    type Out: HList;

    fn map(self, functions: Functions) -> Self::Out;
}

impl Map<HNil> for HNil {
    type Out = HNil;

    fn map(self, _functions: HNil) -> Self::Out {
        HNil
    }
}

impl<F, A, B, FunctionTail, Tail> Map<HCons<F, FunctionTail>> for HCons<A, Tail>
where
    F: FnOnce(A) -> B,
    FunctionTail: HList,
    Tail: Map<FunctionTail>,
{
    type Out = HCons<B, Tail::Out>;

    fn map(self, functions: HCons<F, FunctionTail>) -> Self::Out {
        let (head, tail) = (self.0, self.1);
        let (head_f, tail_f) = (functions.0, functions.1);

        HCons(head_f(head), tail.map(tail_f))
    }
}

pub trait Control: private::Sealed {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Break;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Continue<T>(PhantomData<T>);

impl Control for Break {}

impl<Tail: Control> Control for Continue<Tail> {}

mod private {
    use super::{Break, Continue, HCons, HList, HNil};
    pub trait Sealed {}

    impl Sealed for HNil {}

    impl<Head, Tail: HList> Sealed for HCons<Head, Tail> {}

    impl Sealed for Break {}

    impl<T: Sealed> Sealed for Continue<T> {}
}

#[cfg(test)]
mod tests {
    use super::{Extract, HCons, HNil, Map};

    #[test]
    fn extract_type_head() {
        // Precondition.
        let list = hlist![1, 2.0, "3"];

        // Action.
        let (target, rest): (i32, _) = list.extract();

        // Postcondition.
        assert_eq!(target, 1);
        assert_eq!(rest, HCons(2.0, HCons("3", HNil)));
    }

    #[test]
    fn extract_type_tail() {
        // Precondition.
        let list = hlist![1, 2.0, "3"];

        // Action.
        let (target, rest): (f64, _) = list.extract();

        // Postcondition.
        assert_eq!(target, 2.0);
        assert_eq!(rest, HCons(1, HCons("3", HNil)));
    }

    #[test]
    fn map_nil() {
        // Precondition.
        let list = hlist![];
        let functions = hlist![];

        // Action.
        let mapped = list.map(functions);

        // Postcondition.
        assert_eq!(mapped, hlist![]);
    }

    #[test]
    fn map_cons() {
        // Precondition.
        let list = hlist![1, 2.0, "3"];
        let functions = hlist![|x: i32| x.to_string(), |x: f64| x + 1.0, |x: &str| x.len()];

        // Action.
        let mapped = list.map(functions);

        // Postcondition.
        assert_eq!(mapped, hlist!["1".into(), 3.0, 1]);
    }
}
