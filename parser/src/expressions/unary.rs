use errors::{KirinError, SpannedError};
use crate::expressions::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use scanner::{Token, TokenType};
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: UnaryOp,
    pub right: Expression,
    pub span: AstSpan,
    pub inferred_type: Option<KirinType>,
}

impl Unary {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_unary(self)
    }

    pub fn new(right: Expression, operator: UnaryOp, span: AstSpan) -> Self {
        Self {
            operator,
            right,
            span,
            inferred_type: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    Negate,
    Not
}

impl UnaryOp {
    pub fn from_token(token: &Token) -> Result<UnaryOp, KirinError> {
        match token.token_type {
            TokenType::Minus => Ok(UnaryOp::Negate),
            TokenType::Not => Ok(UnaryOp::Not),

            _ => Err(KirinError::Parse(
                SpannedError{
                    message: format!(
                        "token `{:?}` is not a unary operator", token.token_type
                    ),
                    line: token.span.line,
                    column: token.span.column
                }
            ))
        }
    }
}
