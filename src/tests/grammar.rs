use ::ast;
use ::parser;

#[test]
fn parse_ident_only_alpha() {
    let test_str = "foo";
    assert_eq!(parser::parse_Ident(test_str),
               Ok(ast::Ident(test_str.to_string())));
}

#[test]
fn parse_ident_alphanum() {
    let test_str = "foo123";
    assert_eq!(parser::parse_Ident(test_str),
               Ok(ast::Ident(test_str.to_string())));
}

#[test]
fn parse_ident_alpha_underscore() {
    let test_str = "foo_bar";
    assert_eq!(parser::parse_Ident(test_str),
               Ok(ast::Ident(test_str.to_string())));
}

#[test]
fn parse_ident_leading_underscore() {
    let test_str = "_foo";
    assert_eq!(parser::parse_Ident(test_str),
               Ok(ast::Ident(test_str.to_string())));
}

#[test]
fn parse_paramlist_empty() {
    assert_eq!(parser::parse_ParamList(""),
               Ok(vec![ ]));
}

#[test]
fn parse_paramlist_single() {
    assert_eq!(parser::parse_ParamList("foo"),
               Ok(vec![ ast::RVal::LVal(ast::LVal::Var(ast::Ident("foo".to_string()))) ]));
}

#[test]
fn parse_paramlist_many() {
    assert_eq!(parser::parse_ParamList("foo, bar"),
               Ok(vec![ ast::RVal::LVal(ast::LVal::Var(ast::Ident("foo".to_string()))),
                        ast::RVal::LVal(ast::LVal::Var(ast::Ident("bar".to_string())))]));
}

#[test]
fn parse_funchead() {
    // we don't need to exhaustively test the parsing of parmlist here
    // since we already did that above
    assert_eq!(parser::parse_FuncHead("fn foo(1, 2)"),
               Ok(ast::FuncHead {
                   name: ast::Ident("foo".to_string()),
                   params: vec![ ast::RVal::Num(ast::Num(1)),
                                 ast::RVal::Num(ast::Num(2)) ],
               }));
}

#[test]
fn parse_num() {
    assert_eq!(parser::parse_Num("123"),
               Ok(ast::Num(123)));
    assert_eq!(parser::parse_Num("-123"),
               Ok(ast::Num(-123)));
    assert_eq!(parser::parse_Num("0"),
               Ok(ast::Num(0)));
}

#[test]
fn parse_val_var() {
    assert_eq!(parser::parse_RVal("foo"),
               Ok(ast::RVal::LVal(
                   ast::LVal::Var(
                       ast::Ident("foo".to_string())))));
}

#[test]
fn parse_val_num() {
    assert_eq!(parser::parse_RVal("123"),
               Ok(ast::RVal::Num(ast::Num(123))));
}

#[test]
fn parse_aexp() {
    assert_eq!(parser::parse_AExp("1 + 2"),
               Ok(ast::AExp::Add(Box::new(ast::AExp::Val(ast::RVal::Num(ast::Num(1)))),
                                 Box::new(ast::AExp::Val(ast::RVal::Num(ast::Num(2)))))));
    assert_eq!(parser::parse_AExp("1 + 2 * bar"),
               Ok(ast::AExp::Add(Box::new(ast::AExp::Val(ast::RVal::Num(ast::Num(1)))),
                                 Box::new(ast::AExp::Mul(
                                     Box::new(ast::AExp::Val(
                                         ast::RVal::Num(ast::Num(2)))),
                                     Box::new(ast::AExp::Val(
                                         ast::RVal::LVal(
                                             ast::LVal::Var(ast::Ident("bar".to_string()))))))))));
}
