pub mod preamble;

use crate::syntax::generic::SourcePos;
use crate::syntax::generic::Syntax;
use crate::syntax::generic::TelesMaybe;
use lalrpop_util;

use super::syntax::binding::*;
use super::syntax::core::Core as CE;
use super::syntax::core::*;
use super::syntax::raw::Expr as RE;
use super::syntax::raw::*;

fn uwu() {
    let mut v = vec![1, 2, 3];
    v.first().unwrap();

    // //let x = Some(x);
    // //v.into_iter().rev().fold();
    // let body = Box::new(Raw::new(RE::Univ, SourcePos::new((0, 0), (0, 0))));
    // let r: TelesMaybe<Box<Raw>> = vec![];
    // let body2 = Box::new(Raw::new(RE::Univ, SourcePos::new((0, 0), (0, 0))));
    // let a = body2.clone();
    // //let opt = Some(body2);
    // r.into_iter().rev().fold(body, |acc, (binders, dom)| {
    //     binders.into_iter().rev().fold(acc, |ac, name| {
    //         Box::new(Raw(
    //             RE::Lam {
    //                 param: name,
    //                 dom: dom.clone(),
    //                 cod: ac,
    //             },
    //             SourcePos::none(),
    //         ))
    //     })
    // });
}
