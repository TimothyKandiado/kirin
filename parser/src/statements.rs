mod declaration;

use crate::expressions::Expression;

pub use declaration::VariableDeclaration;

#[derive(Debug, Clone)]
pub enum Statement {
    None,
    ExpressionStatement(Expression),
    VarDeclaration(VariableDeclaration)
}
