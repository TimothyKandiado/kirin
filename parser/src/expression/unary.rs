use crate::expression::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use scanner::Token;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression,
    pub inferred_type: Option<KirinType>,
}

impl Unary {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_unary(self)
    }

    pub fn new(right: Expression, operator: Token) -> Self {
        Self {
            operator,
            right,
            inferred_type: None,
        }
    }
}
