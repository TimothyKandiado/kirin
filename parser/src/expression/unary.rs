use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;
use scanner::token::Token;

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Expression,
}

impl Unary {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_unary(self)
    }

    pub fn new(right: Expression, operator: Token) -> Self {
        Self { operator, right }
    }
}
