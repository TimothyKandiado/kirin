use crate::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
}
