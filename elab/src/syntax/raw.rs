use derive_new::new;

use super::{
    binding::{Name, Var},
    core::CoreDef,
    generic::{Def, ElabError, SourcePos, Syntax, WithPos},
};

pub type ConcDef = Def<Raw>;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expr {
    Ref(Var<Raw, CoreDef>),
    Lam {
        param: Name,
        dom: Option<Box<Raw>>,
        cod: Box<Raw>,
    },
    Pi {
        param: Name,
        dom: Box<Raw>,
        cod: Box<Raw>,
    },
    App {
        expr1: Box<Raw>,
        expr2: Box<Raw>,
    },
    Univ,
    Error(ElabError),
}

#[allow(unused)]
#[derive(new, Debug, Clone)]
pub struct Raw(pub Expr, pub SourcePos);

impl Syntax for Raw {
    type E = Expr;

    fn expr(&self) -> &Self::E {
        &self.0
    }
}

impl WithPos for Raw {
    fn with_pos(expr: Self::E, pos: SourcePos) -> Self {
        Raw(expr, pos)
    }
}
