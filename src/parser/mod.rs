use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/grammar.pest"]
struct CalculatorParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            // Encoding Precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::prefix(negative))
            .op(Op::infix(pow, Left) | Op::prefix(log) | Op::prefix(ln))
            .op(Op::postfix(factorial))
    };
}

fn parse_expr(pairs: Pairs<Rule>) -> f32 {
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
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::log => rhs.log10(),
            Rule::ln => rhs.ln(),
            Rule::negative => rhs * -1_f32,
            rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
        })
        .map_postfix(|_lhs, op| match op.as_rule() {
            // TODO: Implement factorial function
            Rule::factorial => todo!("Factorials are currently not implemented."),
            rule => unreachable!("Expr::parse expected postfix operation, found {:?}", rule),
        })
        .parse(pairs)
}

pub(crate) fn calculate(equation: &str) -> f32 {
    let line = CalculatorParser::parse(Rule::calculation, equation);
    match line {
        Ok(mut l) => parse_expr(l.next().unwrap().into_inner()),
        Err(e) => todo!("Handle Errors at Calculation: {e}"),
    }
}

#[cfg(test)]
mod tests;
