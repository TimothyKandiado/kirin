mod declaration;

use crate::expressions::Expression;

use crate::visitor::StatementVisitor;
pub use declaration::VariableDeclaration;

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
            Statement::ExpressionStatement(expression) => {
                visitor.visit_expression_statement(expression)
            }
            Statement::VarDeclaration(var_declaration) => {
                visitor.visit_var_declaration(var_declaration)
            }
        }
    }
}
