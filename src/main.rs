mod ast;
mod parser;

#[cfg(test)]
mod tests;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let head = parser::parse_FuncHead(&s);
    println!("{:#?}", head);
}
