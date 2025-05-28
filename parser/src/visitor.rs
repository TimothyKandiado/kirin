use crate::expression::{Assign, Binary, Call, Grouping, Literal, Unary, Variable};

pub trait ExpressionVisitor {
    type Output;
    fn visit_binary(&mut self, binary: &Binary) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;
    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_call(&mut self, math_function: &Call) -> Self::Output;
    fn visit_variable(&mut self, variable: &Variable) -> Self::Output;
    fn visit_assign(&mut self, assign: &Assign) -> Self::Output;
}
