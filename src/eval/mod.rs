use std::collections::HashMap;

mod builtins;

pub struct EvalContext {
    pub bindings: HashMap<String, InternValue>,
}

impl EvalContext {
    pub fn new() -> Self {
        EvalContext {
            bindings: HashMap::new(),
        }
    }

    pub fn register_builtins(&mut self) {
        builtins::register(self);
    }
}

pub enum InternValue {
    Integer(isize),
    BuiltinFunction(BuiltinFunction),
}

pub type BuiltinFunction = Box<Fn(&mut EvalContext, Vec<InternValue>)>;
