use crate::visitor::ExpressionVisitor;
use scanner::Token;
use types::KirinType;
use crate::span::AstSpan;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub span: AstSpan,
    pub inferred_type: Option<KirinType>,
}

impl Variable {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_variable(self)
    }

    pub fn new(name: String, span: AstSpan) -> Self {
        Self {
            name,
            span,
            inferred_type: None,
        }
    }
}
