use std::{
    cell::{RefCell, Ref},
    collections::HashMap,
    rc::{Rc, Weak}, ops::Deref,
};

use derive_new::new;

use crate::syntax::{
    binding::Name,
    core::{Core, CoreDef},
};

#[derive(new, Debug, Clone)]
pub struct Typing(pub Name, pub Core);

impl Into<Typing> for (Name, Core) {
    fn into(self) -> Typing {
        Typing(self.0, self.1)
    }
}

#[allow(unused)]
#[derive(new, Debug)]
pub struct Context {
    parent: Weak<RefCell<ContextData>>,
    context_data: Rc<RefCell<ContextData>>,
}

impl Context {
    pub fn derive(&self) -> Self {
        Self {
            parent: Rc::downgrade(&self.context_data),
            context_data: self.context_data.clone(),
        }
    }

    pub fn weaken(&self, typing: Typing) {
        self.context_data.as_ref().borrow_mut().weaken(typing);
    }

    pub fn define(&self, def: CoreDef) {
        self.context_data.as_ref().borrow_mut().define(def);
    }

    // I want to return an Option<&Core>. this is what https://stackoverflow.com/a/51341957 tells me but it didn't work
    pub fn lookup_local(&self, name: &Name) -> impl Deref<Target = Option<&Core>> + '_ {
        Ref::map(self.context_data.borrow(), |c| &c.lookup_local(name))
    }

    // same problem
    pub fn lookup_global(&self, name: &Name) -> Option<&CoreDef> {
        self.context_data.borrow().lookup_global(name)
    }

    pub fn lookup_global_rc(&self, name: &Name) -> Option<Rc<CoreDef>> {
        self.context_data.borrow().lookup_global_rc(name)
    }
}

#[derive(new, Clone, Debug)]
struct ContextData {
    gamma: HashMap<Name, Core>,
    defs: HashMap<Name, Rc<CoreDef>>,
    mutual_defs: Vec<(CoreDef, CoreDef)>,
}

impl ContextData {
    pub fn weaken(&mut self, typing: Typing) {
        self.gamma.insert(typing.0, typing.1);
    }

    pub fn define(&mut self, def: CoreDef) {
        self.defs.insert(def.name.clone(), Rc::new(def));
    }

    pub fn lookup_local(&self, name: &Name) -> Option<&Core> {
        self.gamma.get(name)
    }

    pub fn lookup_global(&self, name: &Name) -> Option<&CoreDef> {
        self.defs.get(name).map(|def_rc| def_rc.as_ref())
    }

    pub fn lookup_global_rc(&self, name: &Name) -> Option<Rc<CoreDef>> {
        self.defs.get(name).map(|def_rc| def_rc.clone())
    }
}
