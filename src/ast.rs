#[derive(Debug)] 
pub enum Expr {
    Number(i32),
    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UnOp(UnOpcode, Box<Expr>),
    Error,
}

#[derive(Debug)] 
pub enum BinOpcode {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
}

#[derive(Debug)]
pub enum UnOpcode {
    Neg,
}
