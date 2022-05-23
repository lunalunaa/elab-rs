use derive_new::new;

use super::core::CoreDef;

#[allow(unused)]
#[derive(new, Debug)]
enum Command {
    Open {
        module_name: String,
        open_as: String,
    },
}

#[allow(unused)]
#[derive(new, Debug)]
enum Stmt {
    Defn(CoreDef),
    Comm(Command),
}
