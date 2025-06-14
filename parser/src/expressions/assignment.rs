use crate::expressions::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: String,
    pub span: AstSpan,
    pub value: Expression,
    pub inferred_type: Option<KirinType>,
}

impl Assign {
    pub fn new(name: String, value: Expression, span: AstSpan) -> Self {
        Self {
            name,
            value,
            span,
            inferred_type: None,
        }
    }

    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_assign(self)
    }
}
