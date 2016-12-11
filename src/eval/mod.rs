use std::collections::HashMap;

pub struct EvalContext {
    pub bindings: HashMap<String, InternValue>,
}

pub enum InternValue {
    Integer(isize),
    BuiltinFunction(BuiltinFunction),
}

pub type BuiltinFunction = Box<Fn(&mut EvalContext, Vec<InternValue>)>;
