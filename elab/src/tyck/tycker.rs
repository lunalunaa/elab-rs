use crate::syntax::{core::Core, raw::Raw};

pub struct Tycker {}

pub struct TyckResult {
    term: Raw,
    ty: Core,
}

impl Tycker {
    fn checkTy(&mut self, expr: &Raw, ty: &Core) -> TyckResult {
        todo!()
    }

    fn inferTy(&mut self, expr: &Raw, ty: &Core) -> TyckResult {
        todo!()
    }
}
