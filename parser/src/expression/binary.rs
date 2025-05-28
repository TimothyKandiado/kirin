use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;
use scanner::token::Token;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Expression,
    pub right: Expression,
    pub operator: Token,
    pub inferred_type: Option<KirinType>
}

impl Binary {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_binary(self)
    }

    pub fn new(left: Expression, right: Expression, operator: Token) -> Self {
        Self {
            left,
            right,
            operator,
            inferred_type: None
        }
    }
}
