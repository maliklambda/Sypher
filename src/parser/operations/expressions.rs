use crate::{
    constants::special_chars::{
        DOT, DOUBLE_QUOTE, SINGLE_QUOTE,
        arithmetic::{DIVIDE, MINUS, MODULO, MULTIPLY, PLUS},
        pattern::{PATTERN_END, PATTERN_START},
    },
    parser::errors::{ExpressionError, ExpressionErrorReason},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Simple(SimpleExpression),
    Arithmetic(ArithmeticExpression),
    String(String),
    Pattern(PatternExpression), // construct an object-pattern that objects are matched onto
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArithmeticExpression {
    lh: String,
    rh: String,
    operator: ArithmeticOperator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticOperator {}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpression {
    Constant(ConstantExpression),
    Variable(VariablExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantExpression {
    Float32(f32),
    Int32(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariablExpression {
    identifier_name: String,
    property_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternExpression {}

pub fn parse_expression(expr_str: &str) -> Result<Expression, ExpressionError> {
    let expr: Expression = {
        if is_pattern(expr_str) {
            Expression::Pattern(parse_pattern_expr(expr_str)?)
        } else if is_string(expr_str)? {
            Expression::String(expr_str[1..expr_str.len() - 1].to_string())
        } else if is_arithmetic_expr(expr_str) {
            Expression::Arithmetic(parse_arithmetic_expr(expr_str)?)
        } else {
            Expression::Simple(parse_simple_expr(expr_str)?)
        }
    };
    Ok(expr)
}

fn is_arithmetic_expr(expr_str: &str) -> bool {
    assert_eq!(expr_str, expr_str.trim());
    let arithmetic_operators = [PLUS, MINUS, MULTIPLY, DIVIDE, MODULO];
    if expr_str.starts_with(MINUS) {
        // might still be constant
        return arithmetic_operators.iter().any(|ao| expr_str[1..].contains(*ao));
    }
    arithmetic_operators.iter().any(|ao| expr_str.contains(*ao))
}

fn parse_arithmetic_expr(expr_str: &str) -> Result<ArithmeticExpression, ExpressionError> {
    todo!("parse arithmetic expression '{expr_str}'");
}

fn is_string(s: &str) -> Result<bool, ExpressionError> {
    for quote_char in [SINGLE_QUOTE, DOUBLE_QUOTE] {
        if s.starts_with(quote_char) {
            if s.ends_with(quote_char) {
                return Ok(true);
            } else {
                return Err(ExpressionError::new(
                    ExpressionErrorReason::MissingExpectedChar(quote_char),
                    s.to_string(),
                ));
            }
        }
    }
    Ok(false)
}

fn is_pattern(s: &str) -> bool {
    assert_eq!(s, s.trim());
    s.starts_with(PATTERN_START) && s.ends_with(PATTERN_END)
}

fn parse_pattern_expr(expr_str: &str) -> Result<PatternExpression, ExpressionError> {
    todo!("parse pattern expression '{expr_str}'");
}

fn is_constant(expr_str: &str) -> bool {
    println!("checking if expr: '{expr_str}' is constant");
    if expr_str.chars().next().unwrap().is_alphabetic() {
        return false;
    }
    true
}

fn parse_simple_expr(expr_str: &str) -> Result<SimpleExpression, ExpressionError> {
    if is_constant(expr_str) {
        return Ok(SimpleExpression::Constant(parse_simple_constant_expr(
            expr_str,
        )?));
    }
    Ok(SimpleExpression::Variable(parse_simple_variable_expr(
        expr_str,
    )?))
}

fn parse_simple_constant_expr(expr_str: &str) -> Result<ConstantExpression, ExpressionError> {
    if expr_str
        .chars()
        .all(|c| c.is_numeric() || c == DOT || c == MINUS)
    {
        parse_constant_number(expr_str)
    } else {
        Err(ExpressionError::new(ExpressionErrorReason::InvalidConstant, expr_str.to_string()))
    }
}

fn parse_constant_number(numeric_str: &str) -> Result<ConstantExpression, ExpressionError> {
    if numeric_str.contains(DOT) {
        let f_val = numeric_str.parse::<f32>().map_err(|_| {
            ExpressionError::new(
                ExpressionErrorReason::ParseConstant,
                numeric_str.to_string(),
            )
        })?;
        Ok(ConstantExpression::Float32(f_val))
    } else {
        let i_val = numeric_str.parse::<i32>().map_err(|_| {
            ExpressionError::new(
                ExpressionErrorReason::ParseConstant,
                numeric_str.to_string(),
            )
        })?;
        Ok(ConstantExpression::Int32(i_val))
    }
}

fn parse_simple_variable_expr(expr_str: &str) -> Result<VariablExpression, ExpressionError> {
    let (id_name, prop_name): (String, Option<String>) = {
        let parts = expr_str.split(DOT).collect::<Vec<&str>>();
        match parts.len() {
            1 => (expr_str.to_string(), None),
            2 => (parts[0].to_string(), Some(parts[1].to_string())),
            _ => {
                return Err(ExpressionError::new(
                    ExpressionErrorReason::PropertyOfProperty,
                    expr_str.to_string(),
                ));
            }
        }
    };
    Ok(VariablExpression {
        identifier_name: id_name,
        property_name: prop_name,
    })
}
