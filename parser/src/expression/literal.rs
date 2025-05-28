use crate::span::AstSpan;
use crate::value::ParsedValue;
use crate::visitor::ExpressionVisitor;
use types::KirinType;

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: ParsedValue,
    pub inferred_type: Option<KirinType>,
    pub span: AstSpan,
}

impl Literal {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        visitor.visit_literal(self)
    }

    pub fn new(value: ParsedValue, span: AstSpan) -> Self {
        let inferred_type  = value.try_infer_type();

        Self {
            value,
            inferred_type,
            span,
        }
    }
}
