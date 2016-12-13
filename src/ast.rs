#[derive(Eq, PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub struct Num(pub f64);

#[derive(Debug)]
pub struct FuncHead {
    pub name: Ident,
    pub params: ParamList,
}

pub type ParamList = Vec<RVal>;

#[derive(Debug)]
pub enum RVal {
    Num(Num),
    LVal(LVal),
    FuncCall(FuncCall),

    OpAdd(Box<RVal>, Box<RVal>),
    OpSub(Box<RVal>, Box<RVal>),
    OpMul(Box<RVal>, Box<RVal>),
    OpDiv(Box<RVal>, Box<RVal>),
}

#[derive(Eq, PartialEq, Debug)]
pub enum LVal {
    Var(Ident),
}

#[derive(Debug)]
pub struct FuncCall {
    pub name: Ident,
    pub args: ParamList,
}

#[derive(Debug)]
pub enum Command {
    RVal(RVal),
    Assign(Assign),
}

#[derive(Debug)]
pub struct Assign(pub LVal, pub RVal);

#[derive(Debug)]
pub enum BExp {
    Eq(RVal, RVal),
    Neq(RVal, RVal),

    Le(RVal, RVal),
    Leq(RVal, RVal),

    Ge(RVal, RVal),
    Geq(RVal, RVal),

    Not(Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Or(Box<BExp>, Box<BExp>),

    Val(BVal),
}

#[derive(Debug)]
pub enum BVal {
    True,
    False,
}
