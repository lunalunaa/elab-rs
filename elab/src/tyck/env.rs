use std::rc::Rc;

use derive_new::new;

use crate::syntax::{
    core::{Core, CoreDef},
    generic::Param,
};

type Typing = Param<Core>;

#[allow(unused)]
#[derive(new, Debug)]
pub struct Context {
    gamma: Vec<Typing>,
    defs: Vec<Rc<CoreDef>>,
    mutual_defs: Vec<(CoreDef, CoreDef)>,
}

impl Context {
    pub fn derive(&self) -> Self {
        Self {
            gamma: self.gamma.clone(),
            defs: self.defs.clone(),
            mutual_defs: self.mutual_defs.clone(),
        }
    }

    pub fn weaken(&mut self, typing: Typing) {
        self.gamma.push(typing)
    }

    pub fn define(&mut self, def: CoreDef) {
        self.defs.push(Rc::new(def));
    }
}
