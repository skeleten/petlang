use super::{ BuiltinTable,
             EvalContext,
             EvalError,
             InternValue };

pub fn register(tbl: &mut BuiltinTable) {
    /* TODO */
    tbl.functions.insert("sin".to_string(), Box::new(builtin_sin));
}

fn builtin_sin(_ctx: &mut EvalContext, args: Vec<InternValue>)
               -> Result<InternValue, EvalError> {
    if args.len() != 1 {
        Err(EvalError::WrongNumberOfArguments)
    } else {
        let arg = args[0].clone();
        if let InternValue::Number(num) = arg {
            let res = num.sin();
            Ok(InternValue::Number(res))
        } else {
            Err(EvalError::TypeMistmatch)
        }
    }
}
