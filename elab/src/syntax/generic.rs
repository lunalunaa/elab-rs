use std::collections::VecDeque;

use super::binding::Name;
use crate::pretty::Docable;

use derive_new::new;

//pub type Param<T> = (Name, T);

#[derive(Debug, Clone, new)]
pub struct Param<T: Syntax>(pub Name, pub T);

#[derive(Debug, Clone)]
pub struct ParamMaybe<T: Syntax>(pub Name, pub Option<T>);

pub type Params<T> = Vec<Param<T>>;
pub type ParamsMaybe<T> = Vec<ParamMaybe<T>>;
pub type Tele<T> = Vec<Param<T>>;
pub type TelesMaybe<T> = Vec<ParamsMaybe<T>>;

pub type Arg<T> = T;
pub type Args<T> = Vec<Arg<T>>;

pub trait Syntax: Docable {
    type E;

    fn expr(&self) -> &Self::E;
}

pub trait WithPos: Syntax {
    fn with_pos(expr: Self::E, pos: SourcePos) -> Self;
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Def<T: Syntax> {
    pub name: Name,
    pub tele: Tele<T>,
    pub ty: T,
    pub body: T,
}

#[allow(unused)]
#[derive(new, Debug, Clone)]
pub struct SourcePos {
    line: (i32, i32),
    col: (i32, i32),
}

impl SourcePos {
    pub fn none() -> Self {
        Self {
            line: (0, 0),
            col: (0, 0),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

#[allow(unused)]
#[derive(new, Debug, Clone)]
pub struct ElabError {
    msg: &'static str,
    severity: Severity,
}

impl ElabError {
    pub fn is_error(&self) -> bool {
        match self.severity {
            Severity::Info => false,
            Severity::Warn => false,
            Severity::Error => true,
        }
    }

    pub fn error(msg: &'static str) -> Self {
        Self {
            msg,
            severity: Severity::Error,
        }
    }
}
