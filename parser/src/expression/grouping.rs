use types::KirinType;
use crate::expression::Expression;
use crate::visitor::ExpressionVisitor;

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Expression,
    pub inferred_type: Option<KirinType>
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_grouping(self)
    }

    pub fn new(expression: Expression) -> Self {
        Self { expression, inferred_type: None }
    }
}
