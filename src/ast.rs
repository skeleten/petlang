#[derive(Eq, PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(Eq, PartialEq, Debug)]
pub struct Num(pub isize);

#[derive(Eq, PartialEq, Debug)]
pub struct FuncHead {
    pub name: Ident,
    pub params: ParamList,
}

pub type ParamList = Vec<RVal>;

#[derive(Eq, PartialEq, Debug)]
pub enum RVal {
    Num(Num),
    LVal(LVal),
}

#[derive(Eq, PartialEq, Debug)]
pub enum LVal {
    Var(Ident),
}

#[derive(Eq, PartialEq, Debug)]
pub enum AExp {
    Val(RVal),
    Add(Box<AExp>, Box<AExp>),
    Sub(Box<AExp>, Box<AExp>),
    Mul(Box<AExp>, Box<AExp>),
    Div(Box<AExp>, Box<AExp>),
}
