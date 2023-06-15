lalrpop_mod!(pub parser);

pub(crate) struct Calculator {
    // Parser
    parser: parser::ExprParser,
}

// Implement Parser Functions
impl Calculator {
    pub fn new() -> Self {
        Self {
            parser: parser::ExprParser::new(),
        }
    }
    pub fn calculate(&self, equation: &str) -> f32 {
        self.parser.parse(equation).unwrap()
    }
}

// Tests for the Calculator struct
#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn simple_parsing_test() {
        let calculator = Calculator::new();
        let equation_to_parse = "2 + 2";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 4.0);
        let equation_to_parse = "2+2";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 4.0)
    }

    #[test]
    fn floating_parsing_test() {
        let calculator = Calculator::new();
        let equation_to_parse = "1.2 + 3.5";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == (1.2 + 3.5));
        let equation = ".3 + 1";
        let answer = calculator.calculate(equation);
        assert!(answer == (0.3 + 1f32));
    }

    #[test]
    fn negative_numbers_test() {
        let calculator = Calculator::new();
        let equation_to_parse = "-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -1f32);
        let equation_to_parse = "0-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -1f32);
    }

    #[test]
    fn negative_arithmetic_test() {
        let calculator = Calculator::new();
        let equation_to_parse = "2*-1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == -2.0);
    }

    #[test]
    #[should_panic]
    fn multiple_negatives_test() {
        let calculator = Calculator::new();
        let equation_to_parse = "--1";
        let answer = calculator.calculate(equation_to_parse);
        assert!(answer == 1f32)
    }
}
