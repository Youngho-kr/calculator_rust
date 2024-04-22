use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator);
pub mod ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CalculatorError {
    InputTooBig,
    InvalidInput
}


fn main() {
    // let text = "(100 + 20) * x";

    // Too big number
    // let text = "2147483648";

    let text = "100 + (20 - (10)) ^ 2 * 5";

    let result = calculator::ExprParser::new()
        .parse(text)
        .unwrap();

    println!("{:?}", result);
}
