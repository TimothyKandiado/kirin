use errors::{KirinError, SpannedError};
use scanner::{Token, TokenSpan, TokenType};
use types::KirinType;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ParsedValue {
    Null,
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Array(Vec<ParsedValue>),
    Vector(Vec<ParsedValue>),
}

impl ParsedValue {
    pub fn from_token(token: &Token) -> Result<ParsedValue, KirinError> {
        match token.token_type {
            TokenType::None => Ok(ParsedValue::Null),
            TokenType::True => Ok(ParsedValue::Bool(true)),
            TokenType::False => Ok(ParsedValue::Bool(false)),
            TokenType::String => Ok(ParsedValue::String(token.lexeme.clone())),
            TokenType::Number => parse_number(&token.lexeme, token.span),

            _ => Err(KirinError::Parse(SpannedError {
                message: format!(
                    "cannot parse Token: `{:?}` into a literal value",
                    token.token_type
                ),
                line: token.span.line,
                column: token.span.column,
            })),
        }
    }

    pub fn try_infer_type(&self) -> Option<KirinType> {
        match self {
            ParsedValue::Bool(_) => Some(KirinType::Bool),
            ParsedValue::Int(_) => Some(KirinType::Int),
            ParsedValue::Float(_) => Some(KirinType::Float),
            ParsedValue::String(_) => Some(KirinType::String),
            ParsedValue::Null => Some(KirinType::Null),

            _ => None,
        }
    }
}

fn parse_number(number: &str, span: TokenSpan) -> Result<ParsedValue, KirinError> {
    let split = number.split("E").collect::<Vec<&str>>();

    let base_result = split[0].parse::<f64>();

    match base_result {
        Ok(n) => {
            if split.len() == 1 {
                if n.fract() == 0.0 {
                    return Ok(ParsedValue::Int(n as i64));
                }

                return Ok(ParsedValue::Float(n));
            }

            let exponent_result = split[1].parse::<i64>();
            match exponent_result {
                Ok(e) => Ok(ParsedValue::Float(n * 10f64.powi(e as i32))),

                Err(error) => Err(KirinError::Parse(SpannedError {
                    line: span.line,
                    column: span.column,
                    message: format!(
                        "failed to parse exponent of number: `{}`",
                        error.to_string()
                    ),
                })),
            }
        }
        Err(err) => Err(KirinError::Parse(SpannedError {
            line: span.line,
            column: span.column,
            message: err.to_string(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use crate::value::{ParsedValue, parse_number};
    use scanner::TokenSpan;

    #[test]
    fn test_parse_number() {
        let src = vec!["20.9", "10E5", "2E-3", "1000"];

        let calculated = src
            .iter()
            .map(|&s| parse_number(s, TokenSpan::default()).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![
            ParsedValue::Float(20.9),
            ParsedValue::Float(10E5),
            ParsedValue::Float(2E-3),
            ParsedValue::Int(1000),
        ];

        assert_eq!(calculated, expected);
    }
}
