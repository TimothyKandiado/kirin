use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;
use scanner::Token;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Expression,
    pub inferred_type: Option<KirinType>,
}

impl Assign {
    pub fn new(name: Token, value: Expression) -> Self {
        return Self {
            name,
            value,
            inferred_type: None,
        };
    }

    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_assign(self)
    }
}
