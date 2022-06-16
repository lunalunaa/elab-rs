use derive_new::new;

use std::{collections::HashMap, rc::Weak};

use super::{
    binding::{Name, Var},
    generic::{Arg, Args, Def, ElabError, Param, Syntax, Tele},
};

pub type CoreDef = Def<Core>;
pub type CoreArg = Arg<Core>;
pub type CoreArgs = Args<Core>;
pub type CoreParam = Param<Core>;
pub type CoreVar = Var<Core, CoreDef>;
pub type CoreTele = Tele<Core>;
pub type Subst = HashMap<CoreVar, Core>;

#[derive(Debug, Clone)]
pub enum CallTerm {
    Func { def: Weak<CoreDef>, args: CoreArgs },
}

impl CallTerm {
    pub fn make(expr: &Core, arg: CoreArg) -> Core {
        let expr = expr.clone();
        match expr.expr() {
            Core::Lam(param, body) => {
                let name = param.0.clone();
                let local_var = CoreVar::new_local(name);
                body.as_ref().clone().subst_local(local_var, arg)
            }
            _ => Core::App(Box::new(expr), Box::new(arg)),
        }
    }
}

#[derive(Debug, Clone, new)]
pub struct Renaming<'a> {
    pub from: &'a Name,
    pub to: &'a Name,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Core {
    Ref(CoreVar),
    Lam(Box<CoreParam>, Box<Core>),
    App(Box<Core>, Box<CoreArg>),
    Pi(Box<CoreParam>, Box<Core>),
    Call(Box<CallTerm>),
    Univ,
    Error(ElabError),
}

impl Core {
    pub fn subst_local(self, var: CoreVar, tm: Core) -> Self {
        let mut sub = Subst::new();
        sub.insert(var, tm);
        self.do_subst(&sub)
    }

    pub fn do_subst(self, sub: &Subst) -> Self {
        use Core::*;
        match self {
            Ref(a) => sub.get(&a).unwrap_or(&Ref(a)).clone(),
            Lam(param, body) => body.do_subst(sub),
            App(a, b) => App(Box::new(a.do_subst(sub)), Box::new(b.do_subst(sub))),
            Pi(param, cod) => Pi(param, Box::new(cod.do_subst(sub))),
            misc => misc,
        }
    }

    pub fn rename_local(self, Renaming { from, to }: Renaming) -> Self {
        let mut sub = Subst::new();
        let (var, tm) = (
            CoreVar::Local(from.clone(), None),
            Core::Ref(CoreVar::Local(to.clone(), None)),
        );
        sub.insert(var, tm);
        self.do_subst(&sub)
    }
}

impl Syntax for Core {
    type E = Core;

    fn expr(&self) -> &Self::E {
        &self
    }
}
