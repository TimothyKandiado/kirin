use crate::expressions::Expression;
use crate::span::AstSpan;
use crate::visitor::ExpressionVisitor;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Expression,
    pub span: AstSpan,
    pub arguments: Vec<Expression>,
    pub inferred_type: Option<KirinType>,
}

impl Call {
    pub fn new(callee: Expression, span: AstSpan, arguments: Vec<Expression>) -> Self {
        Self {
            callee,
            span,
            arguments,
            inferred_type: None,
        }
    }

    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_call(self)
    }
}
