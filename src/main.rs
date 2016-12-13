extern crate rustyline;

mod ast;
mod eval;
mod parser;

fn main() {
    let mut ctx = eval::EvalContext::new();
    let mut ed = rustyline::Editor::<()>::new();

    loop {
        let rl = ed.readline(">> ");
        match rl {
            Ok(s) => {
                ed.add_history_entry(&s);
                if s.trim() == "exit" {
                    break;
                }

                let head = match parser::parse_Command(&s) {
                    Ok(h) => h,
                    Err(e) => {
                        println!("Couldn't parse '{}': {:#?}", s, e);
                        continue;
                    }
                };
                let result = eval::eval_cmd(&head, &mut ctx);
                match result {
                    Ok(val)  => println!("{}", val),
                    Err(err) => println!("{}", err),
                }
            },
            Err(_) => { }
        }
    }
}
