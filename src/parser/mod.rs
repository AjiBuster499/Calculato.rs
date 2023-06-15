#[derive(Parser)]
#[grammar = "src/parser/grammar.pest"]
struct CalculatorParser;

#[derive(Debug)]
pub enum Expr {
    Number(f32),
    Operator {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
