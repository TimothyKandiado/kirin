use crate::expressions::Expression;
use crate::span::AstSpan;

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub initializer: Option<Expression>,
    pub span: AstSpan
}

impl VariableDeclaration {
    pub fn new(name: String, initializer: Option<Expression>, span: AstSpan) -> Self {
        Self { name, initializer, span }
    }
}