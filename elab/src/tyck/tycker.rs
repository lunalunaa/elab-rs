use std::rc::{Rc, Weak};

use derive_new::new;

use crate::syntax::{
    binding::Var,
    core::{Core, Renaming},
    generic::{ElabError, Param, ParamMaybe, Severity, WithPos},
    raw::Expr as RE,
    raw::{ConcParamMaybe, Raw},
};

use super::env::{Context, Typing};

pub struct Tycker {
    ctx: Context,
}

#[derive(Debug, new)]
pub struct TyckResult {
    term: Core,
    ty: Core,
}

impl TyckResult {
    pub fn is_error(&self) -> bool {
        match self.term {
            Core::Error(_) => true,
            _ => false,
        }
    }
}

#[allow(unused)]
#[derive(Debug, new)]
pub struct TyckError {
    msg: &'static str,
    tm: Raw,
    ty: Option<Core>,
}

impl TyckError {
    pub fn to_term(&self, severity: Severity) -> Core {
        Core::Error(ElabError::new(self.msg, severity))
    }
}

pub struct UnifyError {
    msg: &'static str,
    original: (Core, Core),
}

impl Tycker {
    fn check_ty(&mut self, raw: Raw, ty: Core) -> TyckResult {
        let Raw(re, pos) = raw;

        match (re, ty) {
            (RE::Lam(lam_param, lam_body), Core::Pi(pi_param, pi_body)) => {
                let ParamMaybe(lam_param_name, _) = lam_param.as_ref();
                let Param(pi_param_name, pi_param_ty) = pi_param.as_ref();
                let pi_substed = pi_body.rename_local(Renaming {
                    from: pi_param_name,
                    to: lam_param_name,
                });
                // TODO: should unify, but we ignore here for now
                self.ctx
                    .weaken(Typing::new(lam_param_name.clone(), pi_param_ty.clone()));

                self.check_ty(*lam_body, pi_substed)
            }
            (RE::Univ, Core::Univ) => TyckResult::new(Core::Univ, Core::Univ),
            (re, ty) => {
                let inferred = self.infer_ty(Raw(re, pos));
                match self.unify(inferred.ty, ty) {
                    Ok(ty) => TyckResult::new(inferred.term, ty),
                    Err(err) => TyckResult::new(err.original.0, err.original.1),
                };
                todo!()
            }
        }
    }

    fn unify(&mut self, ty1: Core, ty2: Core) -> Result<Core, UnifyError> {
        todo!()
    }

    fn infer_ty(&mut self, expr: Raw) -> TyckResult {
        let Raw(re, pos) = expr;
        match re {
            RE::Ref(r) => match r {
                Var::Local(name, _) => match self.ctx.lookup_local(&name) {
                    Some(c) => TyckResult::new(Core::Ref(Var::Local(name, None)), c.clone()),
                    None => todo!(),
                },
                Var::Global(name, _) => {
                    let ty_rc = self.ctx.lookup_global_rc(&name).unwrap();
                    TyckResult {
                        ty: ty_rc.as_ref().ty.clone(),
                        term: Core::Ref(Var::Global(name, Rc::downgrade(&ty_rc))),
                    }
                }
                Var::Meta(_) => todo!(),
                _ => todo!("report internal error and panic"),
            },
            RE::Lam(param, body) => {
                let pi = Self::gen_pi(&param, &body);
                let lam = Raw::with_pos(RE::Lam(param, body), pos);
                self.check_ty(lam, pi)
            }
            RE::Univ => TyckResult {
                term: Core::Univ,
                ty: Core::Univ,
            },
            RE::Error(_) => todo!(),
            _ => todo!(),
        }
    }

    fn gen_pi(param: &Box<ConcParamMaybe>, body: &Box<Raw>) -> Core {
        todo!()
    }
}
