use crate::object::ParsedObject;
use crate::visitor::ExpressionVisitor;

#[derive(Debug, Clone)]
pub struct Literal {
    pub object: ParsedObject,
}

impl Literal {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_literal(self)
    }

    pub fn new(object: ParsedObject) -> Self {
        Self { object }
    }
}
