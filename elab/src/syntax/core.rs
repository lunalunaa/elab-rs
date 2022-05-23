use std::{collections::HashMap, rc::Weak};

use super::{
    binding::Var,
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
            Core::Lam { param, body } => {
                let (name, tm) = *param.clone();
                let body = body.clone();
                let local_var = CoreVar::new_local(name);
                body.subst_local(local_var, tm)
            }
            _ => Core::App(Box::new(expr), Box::new(arg)),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Core {
    Ref(CoreVar),
    Lam {
        param: Box<CoreParam>,
        body: Box<Core>,
    },
    App(Box<Core>, Box<CoreArg>),
    Pi {
        tele: CoreTele,
        cod: Box<Core>,
    },
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
            Lam { param: _, body } => body.do_subst(sub),
            App(a, b) => App(Box::new(a.do_subst(sub)), Box::new(b.do_subst(sub))),
            Pi { tele, cod } => Pi {
                tele,
                cod: Box::new(cod.do_subst(sub)),
            },
            misc => misc,
        }
    }
}

impl Syntax for Core {
    type E = Core;

    fn expr(&self) -> &Self::E {
        &self
    }
}
