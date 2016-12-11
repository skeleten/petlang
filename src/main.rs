mod ast;
mod eval;
mod parser;

fn main() {
    let mut ctx = eval::EvalContext::new();

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
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
    }
}
