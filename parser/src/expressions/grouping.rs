use crate::expressions::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Expression,
    pub inferred_type: Option<KirinType>,
    pub span: AstSpan,
}

impl Grouping {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_grouping(self)
    }

    pub fn new(expression: Expression, span: AstSpan) -> Self {
        Self {
            expression,
            inferred_type: None,
            span,
        }
    }
}
