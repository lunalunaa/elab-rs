use crate::syntax::raw::Raw;

use super::Docable;

use pretty::RcDoc;

impl Docable for Raw {
    type T = Self;
    fn to_doc(&self) -> RcDoc<Self::T> {
        todo!("pretty concrete")
    }
}
