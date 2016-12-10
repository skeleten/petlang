mod ast;
mod eval;
mod parser;

#[cfg(test)]
mod tests;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let head = parser::parse_AExp(&s);
    println!("{:#?}", head);
}
