use super::{ BuiltinTable,
             BuiltinFunction,
             EvalContext,
             EvalError,
             InternValue };

pub fn register(tbl: &mut BuiltinTable) {
    /* TODO Fill these in */
    tbl.functions.insert("sin".to_string(), abstract_f64_to_f64(f64::sin));
    tbl.functions.insert("cos".to_string(), abstract_f64_to_f64(f64::cos));
    tbl.functions.insert("tan".to_string(), abstract_f64_to_f64(f64::tan));
}


// pub type BuiltinFunction = Box<Fn(&mut EvalContext, Vec<InternValue>)
fn abstract_f64_to_f64<F>(f: F) -> BuiltinFunction where F: Fn(f64) -> f64 + 'static {
    Box::new(move |_ctx: &mut EvalContext, args: Vec<InternValue>| {
        if args.len() != 1 {
            Err(EvalError::WrongNumberOfArguments)
        } else {
            let arg = args[0].clone();
            if let InternValue::Number(num) = arg {
                let res = f(num);
                Ok(InternValue::Number(res))
            } else {
                Err(EvalError::TypeMistmatch)
            }
        }
    })
}
