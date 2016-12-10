#[derive(Eq, PartialEq, Debug)]
pub struct Ident(pub String);

#[derive(Eq, PartialEq, Debug)]
pub struct Num(pub isize);

#[derive(Eq, PartialEq, Debug)]
pub struct FuncHead {
    pub name: Ident,
    pub params: ParamList,
}

pub type ParamList = Vec<Ident>;
