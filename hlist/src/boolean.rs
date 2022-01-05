//! A naive type-level representation of booleans.

pub trait Boolean: private::Sealed {
    fn reify(self) -> bool;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct False;

impl Boolean for False {
    fn reify(self) -> bool {
        false
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct True;

impl Boolean for True {
    fn reify(self) -> bool {
        true
    }
}

mod private {
    use super::{False, True};

    pub trait Sealed {}

    impl Sealed for False {}

    impl Sealed for True {}
}
