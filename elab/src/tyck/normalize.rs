use super::env::Context;
use crate::syntax::{binding::Var, core::*};
use derive_new::new;

#[derive(new, Debug)]
pub struct Normalizer<'a> {
    env: &'a Context,
}

pub enum NormalizeMode {
    WHNF,
    NF,
    None,
}

impl<'a> Normalizer<'a> {
    pub fn normalize(&mut self, term: Core, mode: &NormalizeMode) -> Core {
        use CallTerm::*;
        use Core::*;

        match term {
            r @ Ref(_) => r,
            lam @ Lam { .. } => lam,
            pi @ Pi { .. } => pi,
            App(tm1, tm2) => {
                let tm1_new = self.normalize(*tm1, mode);
                match tm1_new {
                    lam @ Lam { .. } => {
                        let param_normal = self.normalize(*tm2, mode);
                        self.normalize(CallTerm::make(&lam, param_normal), mode)
                    }
                    misc => misc,
                }
            }
            Call(callterm) => {
                let Func { def, args } = *callterm;
                let def = def.upgrade().unwrap();
                let sub = self.build_subst(&def.tele, args, mode);
                let body = def.body.clone();
                self.normalize(body.do_subst(&sub), mode)
            }
            u @ Univ => u,
            err @ Error(_) => err,
        }
    }

    pub fn build_subst(&mut self, tele: &CoreTele, args: CoreArgs, mode: &NormalizeMode) -> Subst {
        let args_normal: CoreArgs = args
            .into_iter()
            .map(|arg| self.normalize(arg, mode))
            .collect();
        let mut sub = Subst::new();
        tele.iter()
            .zip(args_normal.iter())
            .for_each(|((name, _), tm)| {
                sub.insert(Var::Local(name.clone(), None), tm.clone());
            });
        sub
    }
}
