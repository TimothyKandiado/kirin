use crate::expressions::{Assign, Binary, Call, Expression, Grouping, Literal, Unary, Variable};
use crate::statements::VariableDeclaration;

pub trait ExpressionVisitor {
    type Output;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_call(&mut self, callable: &Call) -> Self::Output;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Output;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Output;
}

pub trait StatementVisitor {
    type Output;

    fn visit_none(&mut self) -> Self::Output;
    fn visit_var_declaration(&mut self, var_declaration: &VariableDeclaration) -> Self::Output;
    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output;
}
