use ::ast;
use ::std;
use std::collections::HashMap;
use std::sync::Arc;

mod builtins;

#[derive(Clone)]
pub struct EvalContext {
    pub bindings: HashMap<String, InternValue>,
    pub builtins: Arc<BuiltinTable>,
    inner: Option<Box<EvalContext>>,
}

impl EvalContext {
    pub fn new() -> Self {
        EvalContext {
            bindings: HashMap::new(),
            builtins: Arc::new(BuiltinTable::new()),
            inner: None,
        }
    }

    pub fn new_with_inner(inner: EvalContext) -> Self {
        let builtins = inner.builtins.clone();

        EvalContext {
            bindings: HashMap::new(),
            builtins: builtins,
            inner: Some(Box::new(inner.clone()))
        }
    }

    pub fn assign(&mut self, name: &str, value: InternValue) {
        self.bindings.insert(name.to_string(), value);
    }
}

pub struct BuiltinTable {
    pub functions: HashMap<String, BuiltinFunction>,
}

impl BuiltinTable {
    pub fn new() -> Self {
        let mut tbl = BuiltinTable {
            functions: HashMap::new(),
        };

        builtins::register(&mut tbl);

        tbl
    }
}

#[derive(Clone)]
pub enum InternValue {
    Number(f64),
    BuiltinFunction(String),
}

impl std::fmt::Display for InternValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            &InternValue::Number(ref i) => write!(fmt, "{}", i),
            _ => { Ok(()) }
        }
   }
}

pub type BuiltinFunction = Box<Fn(&mut EvalContext, Vec<InternValue>)
                                  -> Result<InternValue, EvalError>>;

pub fn eval_cmd(cmd: &ast::Command, ctx: &mut EvalContext)
                -> Result<InternValue, EvalError> {
    match *cmd {
        ast::Command::RVal(ref rval) => eval_rval(&rval, ctx),
        ast::Command::Assign(ref assign) => eval_cmd_assign(&assign, ctx),
    }
}

pub fn eval_cmd_assign(assign: &ast::Assign, ctx: &mut EvalContext)
                       -> Result<InternValue, EvalError> {
    let &ast::Assign(ref lhs, ref rhs) = assign;
    let rhs = eval_rval(&rhs, ctx)?;
    let res = rhs.clone();

    match lhs{
        &ast::LVal::Var(ast::Ident(ref s)) => ctx.assign(&s, rhs),
    };

    Ok(res)
}

pub fn eval_rval(val: &ast::RVal, ctx: &mut EvalContext) -> Result<InternValue, EvalError> {
    match *val {
        ast::RVal::Num(ast::Num(i))        => Ok(InternValue::Number(i)),
        ast::RVal::LVal(ref lval)          => eval_rval_lval(&lval, ctx),
        ast::RVal::OpAdd(ref lhs, ref rhs) => eval_rval_op_add(&lhs, &rhs, ctx),
        ast::RVal::OpSub(ref lhs, ref rhs) => eval_rval_op_sub(&lhs, &rhs, ctx),
        ast::RVal::OpMul(ref lhs, ref rhs) => eval_rval_op_mul(&lhs, &rhs, ctx),
        ast::RVal::OpDiv(ref lhs, ref rhs) => eval_rval_op_div(&lhs, &rhs, ctx),
        ast::RVal::FuncCall(ref call)      => eval_rval_funccall(call, ctx),
    }
}

fn eval_rval_lval(val: &ast::LVal, ctx: &mut EvalContext)
                  -> Result<InternValue, EvalError> {
    let val = eval_lval(val, ctx)?;
    match val {
        &InternValue::Number(ref n) => Ok(InternValue::Number(n.clone())),

        _ => Err(EvalError::NotImplemented),
    }
}

fn eval_rval_op_add(lhs: &ast::RVal, rhs: &ast::RVal, ctx: &mut EvalContext)
                    -> Result<InternValue, EvalError> {
    let lhs = eval_rval(lhs, ctx)?;
    let rhs = eval_rval(rhs, ctx)?;
    if let (InternValue::Number(lhs), InternValue::Number(rhs)) = (lhs, rhs) {
        Ok(InternValue::Number(lhs + rhs))
    } else {
        Err(EvalError::TypeMistmatch)
    }
}

fn eval_rval_op_sub(lhs: &ast::RVal, rhs: &ast::RVal, ctx: &mut EvalContext)
                    -> Result<InternValue, EvalError> {
    let lhs = eval_rval(lhs, ctx)?;
    let rhs = eval_rval(rhs, ctx)?;

    if let (InternValue::Number(lhs), InternValue::Number(rhs)) = (lhs, rhs) {
        Ok(InternValue::Number(lhs - rhs))
    } else {
        Err(EvalError::TypeMistmatch)
    }
}

fn eval_rval_op_mul(lhs: &ast::RVal, rhs: &ast::RVal, ctx: &mut EvalContext)
                    -> Result<InternValue, EvalError> {
    let lhs = eval_rval(lhs, ctx)?;
    let rhs = eval_rval(rhs, ctx)?;

    if let (InternValue::Number(lhs), InternValue::Number(rhs)) = (lhs, rhs) {
        Ok(InternValue::Number(lhs * rhs))
    } else {
        Err(EvalError::TypeMistmatch)
    }
}

fn eval_rval_op_div(lhs: &ast::RVal, rhs: &ast::RVal, ctx: &mut EvalContext)
                    -> Result<InternValue, EvalError> {
    let lhs = eval_rval(lhs, ctx)?;
    let rhs = eval_rval(rhs, ctx)?;

    if let (InternValue::Number(lhs), InternValue::Number(rhs)) = (lhs, rhs) {
        Ok(InternValue::Number(lhs / rhs))
    } else {
        Err(EvalError::TypeMistmatch)
    }
}

fn eval_rval_funccall(call: &ast::FuncCall, ctx: &mut EvalContext)
                      -> Result<InternValue, EvalError> {
    // TODO
    let name = call.name.0.to_string();
    let mut args = Vec::with_capacity(call.args.len());
    for a in call.args.iter() {
        let res = eval_rval(a, ctx)?;
        args.push(res);
    };
    let builtins = ctx.builtins.clone();
    if let Some(f) = builtins.functions.get(&name) {
        let result = f(ctx, args)?;
        Ok(result)
    } else {
        Err(EvalError::UnboundFunction)
    }
}

fn eval_lval<'a>(lval: &ast::LVal, ctx: &'a mut EvalContext)
             -> Result<&'a InternValue, EvalError> {
    match *lval {
        ast::LVal::Var(ref id) => eval_lval_var(&id, ctx),
    }
}

fn eval_lval_var<'a>(id: &ast::Ident, ctx: &'a mut EvalContext)
                 -> Result<&'a InternValue, EvalError> {
    if let Some(val) = ctx.bindings.get(&id.0) {
        Ok(val)
    } else {
        Err(EvalError::UnboundVariable)
    }
}

pub enum EvalError {
    TypeMistmatch,
    UnboundVariable,
    UnboundFunction,
    WrongNumberOfArguments,

    NotImplemented,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            EvalError::TypeMistmatch => write!(f, "TypeMistmatch"),
            EvalError::UnboundVariable => write!(f, "Unbound Variable"),
            EvalError::UnboundFunction => write!(f, "Unbound Function"),
            EvalError::WrongNumberOfArguments => write!(f, "Wrong number of Arguments"),
            EvalError::NotImplemented => write!(f, "Not implemented!"),
        }
    }
}
