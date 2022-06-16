use derive_new::new;
use std::hash::Hash;
use std::rc::Weak;
use uuid::Uuid;

use super::generic::Syntax;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Var<T: Syntax, D> {
    Local(Name, Option<Weak<T>>),
    Global(Name, Weak<D>),
    Unresolved(Name),
    Unused,
    Meta(Name),
}

impl<T: Syntax, D> PartialEq for Var<T, D> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var::Local(name1, _), Var::Local(name2, _)) => name1 == name2,
            (Var::Global(name1, _), Var::Local(name2, _)) => name1 == name2,
            (Var::Unresolved(name1), Var::Unresolved(name2)) => name1 == name2,
            (Var::Unused, Var::Unused) => false,
            (Var::Meta(name1), Var::Meta(name2)) => name1 == name2,
            _ => false,
        }
    }
}

impl<T: Syntax, D> Eq for Var<T, D> {}

impl<T: Syntax, D> Hash for Var<T, D> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Var::Local(name, _) => name.hash(state),
            Var::Global(name, _) => name.hash(state),
            Var::Unresolved(name) => name.hash(state),
            Var::Unused => {}
            Var::Meta(name) => name.hash(state),
        }
    }
}

impl<T: Syntax, D> Var<T, D> {
    pub fn fresh_meta() -> Var<T, D> {
        Var::Meta(Name::fresh_id())
    }

    pub fn new_local(name: Name) -> Var<T, D> {
        Var::Local(name, None)
    }
}

#[allow(unused)]
#[derive(new, Debug, Clone)]
pub struct Name {
    name: Option<String>,
    lvl: Option<usize>,
    uuid: Uuid,
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl Name {
    pub fn fresh_id() -> Self {
        Name {
            name: None,
            lvl: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn str_name() -> Self {
        Name {
            name: None,
            lvl: None,
            uuid: Uuid::new_v4(),
        }
    }
}
