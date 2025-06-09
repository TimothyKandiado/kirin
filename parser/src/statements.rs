mod declaration;

use crate::expressions::Expression;

pub use declaration::VariableDeclaration;
use crate::visitor::StatementVisitor;

#[derive(Debug, Clone)]
pub enum Statement {
    None,
    ExpressionStatement(Expression),
    VarDeclaration(VariableDeclaration),
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut impl StatementVisitor<Output = T>) -> T {
        match self {
            Statement::None => visitor.visit_none(),
            Statement::ExpressionStatement(expression) => visitor.visit_expression(expression),
            Statement::VarDeclaration(var_declaration) => visitor.visit_var_declaration(var_declaration),
        }
    }
}
