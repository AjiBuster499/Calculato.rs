use pest::Parser;

use crate::parser::{parse_expr, CalculatorParser, Rule};

pub(crate) struct Calculator;

impl Calculator {
    pub fn calculate(&self, equation: &str) -> f32 {
        let line = CalculatorParser::parse(Rule::calculation, equation);
        match line {
            Ok(mut l) => parse_expr(l.next().unwrap().into_inner()),
            Err(e) => panic!("{e}"),
        }
    }
}

// Tests for the Calculator struct
#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn simple_parsing_test() {
        let calculator = Calculator;
        let equation_to_parse = "2 + 2";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 4.0);
        let equation_to_parse = "2+2";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 4.0)
    }

    #[test]
    fn floating_parsing_test() {
        let calculator = Calculator;
        let equation_to_parse = "1.2 + 3.5";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == (1.2 + 3.5));
        /* let equation = ".3 + 1";
        let answer = calculator.calculate(equation);
        assert!(answer == (0.3 + 1f32)); */
    }

    #[test]
    fn negative_numbers_test() {
        let calculator = Calculator;
        let equation_to_parse = "-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -1f32);
        let equation_to_parse = "0-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -1f32);
    }

    #[test]
    fn negative_arithmetic_test() {
        let calculator = Calculator;
        let equation_to_parse = "2*-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -2.0);
    }

    #[test]
    #[should_panic]
    fn multiple_negatives_test() {
        let calculator = Calculator;
        let equation_to_parse = "--1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 1f32)
    }
}
