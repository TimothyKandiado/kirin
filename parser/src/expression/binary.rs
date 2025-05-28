use errors::{KirinError, SpannedError};
use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;
use scanner::{Token, TokenType};
use types::KirinType;
use crate::span::AstSpan;

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Expression,
    pub right: Expression,
    pub operator: BinaryOp,
    pub inferred_type: Option<KirinType>,
    pub span: AstSpan
}

impl Binary {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_binary(self)
    }

    pub fn new(left: Expression, right: Expression, operator: BinaryOp, span: AstSpan) -> Self {
        Self {
            left,
            right,
            operator,
            inferred_type: None,
            span,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Or,
    And,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Modulus
}

impl BinaryOp {
    pub fn from_token(token: &Token) -> Result<BinaryOp, KirinError> {
        match token.token_type {
            TokenType::Plus => Ok(BinaryOp::Add),
            TokenType::Minus => Ok(BinaryOp::Subtract),
            TokenType::Star => Ok(BinaryOp::Multiply),
            TokenType::Slash => Ok(BinaryOp::Divide),
            TokenType::Percent => Ok(BinaryOp::Modulus),
            TokenType::Caret => Ok(BinaryOp::Power),
            TokenType::Greater => Ok(BinaryOp::Greater),
            TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
            TokenType::Less => Ok(BinaryOp::Less),
            TokenType::LessEqual => Ok(BinaryOp::LessEqual),
            TokenType::Equal => Ok(BinaryOp::Equal),
            TokenType::NotEqual => Ok(BinaryOp::NotEqual),
            TokenType::EqualEqual => Ok(BinaryOp::Equal),

            _ => Err(KirinError::Parse(
                SpannedError {
                    message: format!("token `{:?}` is not a binary operation", token.token_type),
                    line: token.span.line,
                    column: token.span.column,
                }
            ))

        }
    }
}
