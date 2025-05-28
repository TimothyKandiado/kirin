use types::KirinType;
use crate::value::ParsedValue;
use crate::visitor::ExpressionVisitor;

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: ParsedValue,
    pub inferred_type: Option<KirinType>
}

impl Literal {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_literal(self)
    }

    pub fn new(value: ParsedValue) -> Self {
        Self { value, inferred_type: None }
    }
}
