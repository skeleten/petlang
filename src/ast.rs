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

#[derive(Eq, PartialEq, Debug)]
pub struct FuncCall {
    pub name: Ident,
    pub args: ParamList,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Assign(pub LVal, pub RVal);
