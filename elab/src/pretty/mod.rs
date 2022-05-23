use pretty::RcDoc;

mod concrete_pretty;
mod core_pretty;

pub trait Docable {
    type T;
    fn to_doc(&self) -> RcDoc<Self::T>;
}
