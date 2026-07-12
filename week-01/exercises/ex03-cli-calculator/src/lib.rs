#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalculatorError {
    InvalidExpression,
    InvalidNumber,
    DivisionByZero,
}

pub fn parse_operation(token: &str) -> Result<Operation, CalculatorError> {
    todo!("parse one of: +, -, *, /")
}

pub fn evaluate_expression(input: &str) -> Result<f64, CalculatorError> {
    todo!("parse expressions like `12 * 3` with exactly three whitespace-separated tokens")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_operations() {
        assert_eq!(parse_operation("+"), Ok(Operation::Add));
        assert_eq!(parse_operation("-"), Ok(Operation::Subtract));
        assert_eq!(parse_operation("*"), Ok(Operation::Multiply));
        assert_eq!(parse_operation("/"), Ok(Operation::Divide));
    }

    #[test]
    fn evaluates_basic_expressions() {
        assert_eq!(evaluate_expression("2 + 3"), Ok(5.0));
        assert_eq!(evaluate_expression("9 - 4"), Ok(5.0));
        assert_eq!(evaluate_expression("3 * 7"), Ok(21.0));
        assert_eq!(evaluate_expression("8 / 2"), Ok(4.0));
    }

    #[test]
    fn rejects_invalid_shapes() {
        assert_eq!(
            evaluate_expression("3 +"),
            Err(CalculatorError::InvalidExpression)
        );
    }

    #[test]
    fn rejects_division_by_zero() {
        assert_eq!(
            evaluate_expression("8 / 0"),
            Err(CalculatorError::DivisionByZero)
        );
    }
}
