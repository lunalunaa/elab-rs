use super::Docable;
use crate::syntax::core::Core;
use pretty::RcDoc;

impl Docable for Core {
    type T = Self;
    fn to_doc(&self) -> RcDoc<Self::T> {
        todo!("pretty core")
    }
}
