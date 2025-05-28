use crate::expression::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use scanner::Token;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Expression,
    pub paren: Token,
    pub arguments: Vec<Expression>,
    pub inferred_type: Option<KirinType>,
}

impl Call {
    pub fn new(callee: Expression, paren: Token, arguments: Vec<Expression>) -> Self {
        Self {
            callee,
            paren,
            arguments,
            inferred_type: None,
        }
    }

    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_call(self)
    }
}
