use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/grammar.pest"]
pub(crate) struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            // Encoding Precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

pub fn parse_expr(pairs: Pairs<Rule>) -> f32 {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::num => primary.as_str().parse::<f32>().unwrap(),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::pow => lhs.powf(rhs),
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .parse(pairs)
}
