use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;
use scanner::token::Token;

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Expression,
    pub line: usize,
    pub filename: String,
}

impl Assign {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_assign(self)
    }
}
