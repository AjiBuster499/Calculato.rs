use num_bigint::BigInt;
use num_traits::{FromPrimitive, ToPrimitive};
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};
use pest_derive::Parser;
use std::f64::consts;

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
            .op(Op::prefix(sin) | Op::prefix(cos) | Op::prefix(tan))
    };
}

fn parse_expr(pairs: Pairs<Rule>) -> f64 {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::num => primary.as_str().parse::<f64>().unwrap_or_else(|_| {
                match primary.as_str() {
                    "e" => Ok(consts::E),
                    "pi" => Ok(consts::PI),
                    _ => Err("Unknown non-float value: COMING SOON"),
                }
                .unwrap()
            }),
            Rule::expr => parse_expr(primary.into_inner()).to_f64().unwrap(),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::pow => lhs.powf(rhs),
            Rule::comb => todo!("Implement Combinations"),
            Rule::perm => todo!("Implement Permutations"),
            rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::log => rhs.log10(),
            Rule::ln => rhs.ln(),
            Rule::negative => rhs * -1_f64,
            Rule::sin => round_trig(rhs.to_radians().sin()),
            Rule::cos => round_trig(rhs.to_radians().cos()),
            Rule::tan => round_trig(rhs.to_radians().tan()),
            rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
        })
        .map_postfix(|lhs, op| match op.as_rule() {
            Rule::factorial => factorial(&BigInt::from_f64(lhs).unwrap())
                .to_f64()
                .expect("Failed to convert"),
            rule => unreachable!("Expr::parse expected postfix operation, found {:?}", rule),
        })
        .parse(pairs)
}

fn factorial(num: &BigInt) -> BigInt {
    let one = BigInt::from_i8(1).unwrap();
    let zero = BigInt::from_i8(0).unwrap();
    if *num == zero {
        return one;
    };
    num * factorial(&(num - one))
}

fn round_trig(val: f64) -> f64 {
    ((val * 100_f64).round()) / 100_f64
}

pub(crate) fn calculate(equation: &str) -> f64 {
    let line = CalculatorParser::parse(Rule::calculation, equation);
    match line {
        Ok(mut l) => parse_expr(l.next().unwrap().into_inner()),
        Err(e) => todo!("Handle Errors at Calculation: {e}"),
    }
}

#[cfg(test)]
mod tests;
