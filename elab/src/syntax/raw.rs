use derive_new::new;

use super::{
    binding::Var,
    core::CoreDef,
    generic::{Def, ElabError, Param, ParamMaybe, SourcePos, Syntax, WithPos},
};

pub type ConcDef = Def<Raw>;
pub type ConcParam = Param<Raw>;
pub type ConcParamMaybe = ParamMaybe<Raw>;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Expr {
    Ref(Var<Raw, CoreDef>),
    Lam(Box<ConcParamMaybe>, Box<Raw>),
    Pi(Box<ConcParam>, Box<Raw>),
    App(Box<Raw>, Box<Raw>),
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
